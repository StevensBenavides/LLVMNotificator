/*

    MIT License

    Copyright (c) 2026 Stevens Benavides

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.

*/

use std::path::Path;
use std::time::Duration;

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use serde::Deserialize;
use serde_json::{Value, json};
use ureq::unversioned::multipart::{Form, Part};

use crate::changelog::Changelog;
use crate::config::{Config, Webhook};
use crate::error::Result;
use crate::github::Release;

const DISCORD_API_PREFIX: &str = "https://discord.com/api/webhooks/";
const DISCORD_API_PREFIX_ALT: &str = "https://discordapp.com/api/webhooks/";

const PATH_SEGMENT: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

type Attachment = (&'static str, &'static str, Vec<u8>);

#[derive(Debug, Clone)]
pub struct DiscordClient {
    agent: ureq::Agent,
    webhook: Webhook,
    max_response_bytes: usize,
}

impl DiscordClient {
    pub fn new(webhook: &Webhook, max_response_bytes: usize) -> DiscordClient {
        let config: ureq::config::Config = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .timeout_global(Some(Duration::from_secs(20)))
            .user_agent("LLVMNotificator")
            .build();

        DiscordClient {
            agent: ureq::Agent::new_with_config(config),
            webhook: webhook.clone(),
            max_response_bytes,
        }
    }
}

impl DiscordClient {
    #[inline]
    pub fn get_webhook(&self) -> &Webhook {
        &self.webhook
    }

    #[inline]
    pub fn get_max_response_bytes(&self) -> usize {
        self.max_response_bytes
    }
}

impl DiscordClient {
    #[inline]
    pub fn set_webhook(&mut self, webhook: Webhook) {
        self.webhook = webhook;
    }

    #[inline]
    pub fn set_max_response_bytes(&mut self, max_response_bytes: usize) {
        self.max_response_bytes = max_response_bytes;
    }
}

impl DiscordClient {
    #[inline]
    pub fn get_mut_webhook(&mut self) -> &mut Webhook {
        &mut self.webhook
    }

    #[inline]
    pub fn get_mut_max_response_bytes(&mut self) -> &mut usize {
        &mut self.max_response_bytes
    }
}

impl DiscordClient {
    pub fn validate(&self) -> Result<String> {
        if !self.webhook.get_enabled() {
            return Err(std::io::Error::other("Discord webhook is disabled").into());
        }

        let url = self.validated_url()?;

        let mut attempt = 0u32;

        let mut last_error: String = String::from("Discord webhook validation failed");

        while attempt < 2 {
            match self
                .agent
                .get(&url)
                .header("Accept", "application/json")
                .call()
            {
                Ok(mut response) => {
                    let status: u16 = response.status().as_u16();

                    let body: String = response
                        .body_mut()
                        .with_config()
                        .limit(self.max_response_bytes as u64)
                        .read_to_string()?;

                    if status == 200 {
                        let info: WebhookInfo = serde_json::from_str(&body)?;

                        if info.channel_id.trim().is_empty() {
                            return Err(
                                std::io::Error::other("Discord webhook has no channel_id").into()
                            );
                        }

                        let channel: String = if info.name.trim().is_empty() {
                            info.channel_id
                        } else {
                            info.name
                        };

                        return Ok(channel);
                    }

                    if status >= 500 {
                        last_error = format!("Discord webhook validation returned HTTP {status}");
                    } else {
                        return Err(std::io::Error::other(format!(
                            "Discord webhook validation returned HTTP {status}"
                        ))
                        .into());
                    }
                }
                Err(error) => {
                    last_error = error.to_string();
                }
            }

            attempt += 1;

            if attempt < 2 {
                std::thread::sleep(Duration::from_secs(2));
            }
        }

        Err(std::io::Error::other(last_error).into())
    }

    pub fn send_test(&self) -> Result<String> {
        let channel: String = self.validate()?;

        let mut payload: Value = json!({
            "content": "LLVMNotificator webhook test",
            "allowed_mentions": { "parse": [] }
        });

        if !self.webhook.get_username().trim().is_empty() {
            payload["username"] = json!(self.webhook.get_username());
        }

        self.post_json(&payload.to_string())?;

        Ok(channel)
    }

    pub fn send_release(
        &self,
        changelog: &Changelog,
        release: &Release,
        folder: &Path,
        config: &Config,
    ) -> Result<()> {
        let channel: String = self.validate()?;

        let (payload, files, links) =
            self.build_release_messages(changelog, release, folder, config);

        self.post_multipart(&payload, &files)?;

        self.post_json(&links)?;

        eprintln!("Discord: sent {} to #{channel}", changelog.get_tag());

        Ok(())
    }
}

impl DiscordClient {
    fn validated_url(&self) -> Result<String> {
        let url: &str = self.webhook.get_url().trim();

        let prefix_ok: bool =
            url.starts_with(DISCORD_API_PREFIX) || url.starts_with(DISCORD_API_PREFIX_ALT);

        let has_whitespace: bool = url.chars().any(char::is_whitespace);

        if !prefix_ok || has_whitespace || url.len() <= DISCORD_API_PREFIX.len() {
            return Err(std::io::Error::other("invalid Discord webhook URL").into());
        }

        Ok(url.to_string())
    }

    fn wait_url(&self) -> Result<String> {
        let base: String = self.validated_url()?;

        let separator: char = if base.contains('?') { '&' } else { '?' };

        Ok(format!("{base}{separator}wait=true"))
    }

    fn build_release_messages(
        &self,
        changelog: &Changelog,
        release: &Release,
        folder: &Path,
        config: &Config,
    ) -> (String, Vec<Attachment>, String) {
        let title: String = format!("{} v{}", config.get_display_name(), changelog.get_version());

        let previous: &str = changelog
            .get_previous_tag()
            .unwrap_or("the previous release");

        let description: String = format!(
            "{} commits since {}. Full LLVM release notes attached.",
            changelog.get_total_commits(),
            previous
        );

        let mut embed: Value = json!({
            "title": title,
            "description": description,
            "color": 0x1F6FEB,
            "fields": [
                {
                    "name": "Commits",
                    "value": changelog.get_total_commits().to_string(),
                    "inline": true
                },
                {
                    "name": "Previous tag",
                    "value": changelog.get_previous_tag().unwrap_or("none"),
                    "inline": true
                }
            ],
            "footer": { "text": "LLVMNotificator" }
        });

        if !release.get_published_at().trim().is_empty() {
            embed["timestamp"] = json!(release.get_published_at());
        }

        let mut files: Vec<(&str, &str, Vec<u8>)> = Vec::<Attachment>::new();

        if let Ok(content) = std::fs::read_to_string(folder.join("ReleaseNotes.md")) {
            files.push((
                "ReleaseNotes.md",
                "LLVM release notes",
                content.into_bytes(),
            ));
        }

        if let Ok(content) = std::fs::read_to_string(folder.join("notes-diff.md")) {
            let filtered: String = self.filter_llvm_diff(&content, changelog);

            if !filtered.trim().is_empty() {
                files.push((
                    "notes-diff.md",
                    "LLVM release notes diff",
                    filtered.into_bytes(),
                ));
            }
        }

        if let Ok(content) = std::fs::read_to_string(folder.join("commits.md")) {
            files.push(("commits.md", "Release commits", content.into_bytes()));
        }

        let attachments: Vec<Value> = files
            .iter()
            .enumerate()
            .map(|(index, (filename, description, _content))| {
                json!({
                    "id": index,
                    "filename": filename,
                    "description": description
                })
            })
            .collect::<Vec<Value>>();

        let mut payload = json!({
            "content": format!("**{title}**"),
            "embeds": [embed],
            "attachments": attachments,
            "allowed_mentions": { "parse": [] }
        });

        if !self.webhook.get_username().trim().is_empty() {
            payload["username"] = json!(self.webhook.get_username());
        }

        let links = self.build_links_message(changelog, release, config);

        (payload.to_string(), files, links)
    }

    fn build_links_message(
        &self,
        changelog: &Changelog,
        release: &Release,
        config: &Config,
    ) -> String {
        let release_url: &str = if changelog.get_github_url().trim().is_empty() {
            release.get_html_url()
        } else {
            changelog.get_github_url()
        };

        let mut lines: Vec<String> =
            vec!["**Links**".to_string(), format!("- Release: {release_url}")];

        if let Some(previous) = changelog.get_previous_tag() {
            let previous_encoded: String =
                self::utf8_percent_encode(previous, PATH_SEGMENT).to_string();
            let current_encoded: String =
                self::utf8_percent_encode(changelog.get_tag(), PATH_SEGMENT).to_string();

            lines.push(format!(
                "- Compare: https://github.com/{}/compare/{previous_encoded}...{current_encoded}",
                config.get_repository()
            ));
        }

        if let Some(component) = config.get_components().first() {
            let tag: String =
                self::utf8_percent_encode(changelog.get_tag(), PATH_SEGMENT).to_string();
            let notes_path: String = self::encode_path(component.get_notes_path());

            lines.push(format!(
                "- LLVM notes: https://github.com/{}/blob/{tag}/{notes_path}",
                config.get_repository()
            ));
        }

        lines.join("\n")
    }

    fn filter_llvm_diff(&self, diff: &str, changelog: &Changelog) -> String {
        let Some(start) = diff.find("## llvm") else {
            return String::new();
        };

        let rest: &str = &diff[start + "## llvm".len()..];

        let end: usize = rest.find("\n## ").unwrap_or(rest.len());

        format!(
            "# LLVM release notes diff\n\n## llvm{}\n\n---\n\nGitHub: {}\n",
            &rest[..end],
            changelog.get_github_url()
        )
    }

    fn post_multipart(&self, payload: &str, files: &[Attachment]) -> Result<()> {
        let url: String = self.wait_url()?;

        let mut attempt: u32 = 0u32;

        let mut last_error: String = String::from("Discord request failed");

        while attempt < 4 {
            let mut form: Form<'_> = Form::new().text("payload_json", payload);

            if let Some(file) = files.first() {
                form = form.part(
                    "files[0]",
                    Part::bytes(&file.2)
                        .file_name(file.0)
                        .mime_str("text/markdown")?,
                );
            }

            if let Some(file) = files.get(1) {
                form = form.part(
                    "files[1]",
                    Part::bytes(&file.2)
                        .file_name(file.0)
                        .mime_str("text/markdown")?,
                );
            }

            if let Some(file) = files.get(2) {
                form = form.part(
                    "files[2]",
                    Part::bytes(&file.2)
                        .file_name(file.0)
                        .mime_str("text/markdown")?,
                );
            }

            match self.agent.post(&url).send(form) {
                Ok(mut response) => {
                    let status: u16 = response.status().as_u16();

                    let body: String = response
                        .body_mut()
                        .with_config()
                        .limit(self.max_response_bytes as u64)
                        .read_to_string()?;

                    if status == 200 || status == 204 {
                        return Ok(());
                    }

                    if status == 429 {
                        last_error = String::from("Discord rate limited the request");

                        let delay: Duration = self::retry_delay(&body);

                        std::thread::sleep(delay);
                    } else if status >= 500 {
                        last_error = format!("Discord returned HTTP {status}");

                        std::thread::sleep(Duration::from_secs(1u64 << attempt.min(3)));
                    } else {
                        return Err(std::io::Error::other(format!(
                            "Discord webhook returned HTTP {status}"
                        ))
                        .into());
                    }
                }
                Err(error) => {
                    last_error = error.to_string();

                    std::thread::sleep(Duration::from_secs(1u64 << attempt.min(3)));
                }
            }

            attempt += 1;
        }

        Err(std::io::Error::other(last_error).into())
    }

    fn post_json(&self, payload: &str) -> Result<()> {
        let url: String = self.wait_url()?;
        let mut attempt: u32 = 0u32;
        let mut last_error: String = String::from("Discord request failed");

        while attempt < 4 {
            let result = self
                .agent
                .post(&url)
                .header("Content-Type", "application/json")
                .send(payload);

            match result {
                Ok(mut response) => {
                    let status = response.status().as_u16();

                    let body = response
                        .body_mut()
                        .with_config()
                        .limit(self.max_response_bytes as u64)
                        .read_to_string()?;

                    if status == 200 || status == 204 {
                        return Ok(());
                    }

                    if status == 429 {
                        last_error = String::from("Discord rate limited the request");

                        std::thread::sleep(self::retry_delay(&body));
                    } else if status >= 500 {
                        last_error = format!("Discord returned HTTP {status}");

                        std::thread::sleep(Duration::from_secs(1u64 << attempt.min(3)));
                    } else {
                        return Err(std::io::Error::other(format!(
                            "Discord webhook returned HTTP {status}"
                        ))
                        .into());
                    }
                }
                Err(error) => {
                    last_error = error.to_string();

                    std::thread::sleep(Duration::from_secs(1u64 << attempt.min(3)));
                }
            }

            attempt += 1;
        }

        Err(std::io::Error::other(last_error).into())
    }
}

fn retry_delay(body: &str) -> Duration {
    let retry_after = serde_json::from_str::<RetryInfo>(body)
        .map(|info| info.retry_after)
        .unwrap_or(1.0);

    Duration::from_secs_f64(retry_after.clamp(0.1, 30.0))
}

fn encode_path(path: &str) -> String {
    path.split('/')
        .map(|segment| utf8_percent_encode(segment, PATH_SEGMENT).to_string())
        .collect::<Vec<String>>()
        .join("/")
}

#[derive(Debug, Deserialize)]
struct WebhookInfo {
    #[serde(default)]
    channel_id: String,
    #[serde(default)]
    name: String,
}

#[derive(Debug, Deserialize)]
struct RetryInfo {
    #[serde(default)]
    retry_after: f64,
}

#[cfg(test)]
mod tests {
    use crate::changelog::Changelog;
    use crate::config::{Config, Webhook};
    use crate::github::Release;

    use super::DiscordClient;

    fn client() -> DiscordClient {
        DiscordClient::new(
            &Webhook::new(
                true,
                "https://discord.com/api/webhooks/123456789/test-token",
                "LLVMNotificator",
            ),
            1024 * 1024,
        )
    }

    #[test]
    fn accepts_discord_webhook_urls() {
        assert!(client().validated_url().is_ok());
    }

    #[test]
    fn rejects_invalid_webhook_urls() {
        let invalid = DiscordClient::new(&Webhook::new(true, "http://example.com", ""), 1024);

        assert!(invalid.validated_url().is_err());
    }

    #[test]
    fn filters_llvm_diff_only() {
        let changelog = Changelog::new(
            "llvmorg-23.1.4",
            "23.1.4",
            Some("llvmorg-23.1.3".to_string()),
            "https://github.com/llvm/llvm-project/releases/tag/llvmorg-23.1.4",
            "2026-10-20T13:03:13Z",
            vec![("llvm".to_string(), "# LLVM notes".to_string())],
            "## llvm\nLLVM changes\n\n## clang\nClang changes\n".to_string(),
            Vec::new(),
            34,
        );

        let filtered = client().filter_llvm_diff(changelog.get_notes_diff(), &changelog);

        assert!(filtered.contains("LLVM changes"));
        assert!(!filtered.contains("Clang changes"));
    }

    #[test]
    fn builds_llvm_only_messages() {
        let folder = std::env::temp_dir().join(format!(
            "llvmnotificator-discord-test-{}",
            std::process::id()
        ));

        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(folder.join("ReleaseNotes.md"), "# LLVM notes\n").unwrap();
        std::fs::write(
            folder.join("notes-diff.md"),
            "## llvm\nLLVM changes\n\n## clang\nClang changes\n",
        )
        .unwrap();
        std::fs::write(folder.join("commits.md"), "- commit\n").unwrap();

        let changelog = Changelog::new(
            "llvmorg-23.1.4",
            "23.1.4",
            Some("llvmorg-23.1.3".to_string()),
            "https://github.com/llvm/llvm-project/releases/tag/llvmorg-23.1.4",
            "2026-10-20T13:03:13Z",
            vec![("llvm".to_string(), "# LLVM notes".to_string())],
            "## llvm\nLLVM changes\n\n## clang\nClang changes\n".to_string(),
            Vec::new(),
            34,
        );

        let release = Release::new(
            "llvmorg-23.1.4",
            "LLVM 23.1.4",
            "https://github.com/llvm/llvm-project/releases/tag/llvmorg-23.1.4",
            "2026-10-20T13:03:13Z",
            false,
            false,
        );

        let (payload, files, links) =
            client().build_release_messages(&changelog, &release, &folder, &Config::new());

        assert!(payload.contains("**LLVM v23.1.4**"));
        assert!(payload.contains("ReleaseNotes.md"));
        assert!(payload.contains("notes-diff.md"));
        assert!(payload.contains("commits.md"));
        assert!(!payload.contains("Clang changes"));
        assert!(links.contains("Compare:"));
        assert_eq!(files.len(), 3);

        let notes_diff = String::from_utf8(files[1].2.clone()).unwrap();

        assert!(notes_diff.contains("LLVM changes"));
        assert!(!notes_diff.contains("Clang changes"));

        std::fs::remove_dir_all(&folder).unwrap();
    }
}
