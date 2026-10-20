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

use crate::changelog::Changelog;
use crate::cli::Cli;
use crate::config::{Config, Paths};
use crate::discord::DiscordClient;
use crate::error::Result;
use crate::github::{GithubClient, Release};
use crate::notify::{Notifier, NotifierEvent};
use crate::state::{PendingNotification, State};
use crate::version::Version;

#[derive(Debug)]
pub struct App {
    cli: Cli,
    paths: Paths,
    config: Config,
    state: State,
    client: GithubClient,
    notifier: Notifier,
    discord: Option<DiscordClient>,
}

impl App {
    pub fn new(cli: Cli) -> Result<App> {
        let paths: Paths = Paths::new();

        paths.ensure()?;

        let config: Config = Config::load(&paths, cli.get_config())?;

        let state: State = State::load(&paths)?;

        let initial_etag: Option<String> = if state.get_repository() == config.get_repository() {
            state.get_etag().map(str::to_string)
        } else {
            None
        };

        let mut client: GithubClient = GithubClient::new(
            config.get_repository(),
            config.get_token().map(str::to_string),
            config.get_max_response_bytes(),
            config.get_max_commits(),
        );

        client.set_etag(initial_etag);

        let notifier: Notifier = Notifier::new(
            config.get_display_name(),
            "dialog-information",
            config.get_notification_expire_ms(),
            config.get_action_label(),
        );

        let discord: Option<DiscordClient> = config.get_webhook().and_then(|webhook| {
            if webhook.get_enabled() && !webhook.get_url().trim().is_empty() {
                Some(DiscordClient::new(webhook, config.get_max_response_bytes()))
            } else {
                if webhook.get_enabled() {
                    eprintln!("warning: Discord webhook is enabled but has no URL");
                }

                None
            }
        });

        Ok(App {
            cli,
            paths,
            config,
            state,
            client,
            notifier,
            discord,
        })
    }
}

impl App {
    #[inline]
    pub fn get_cli(&self) -> &Cli {
        &self.cli
    }

    #[inline]
    pub fn get_paths(&self) -> &Paths {
        &self.paths
    }

    #[inline]
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    #[inline]
    pub fn get_state(&self) -> &State {
        &self.state
    }

    #[inline]
    pub fn get_client(&self) -> &GithubClient {
        &self.client
    }

    #[inline]
    pub fn get_notifier(&self) -> &Notifier {
        &self.notifier
    }

    #[inline]
    pub fn get_discord(&self) -> Option<&DiscordClient> {
        self.discord.as_ref()
    }
}

impl App {
    #[inline]
    pub fn set_cli(&mut self, cli: Cli) {
        self.cli = cli;
    }

    #[inline]
    pub fn set_paths(&mut self, paths: Paths) {
        self.paths = paths;
    }

    #[inline]
    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    #[inline]
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    #[inline]
    pub fn set_client(&mut self, client: GithubClient) {
        self.client = client;
    }

    #[inline]
    pub fn set_notifier(&mut self, notifier: Notifier) {
        self.notifier = notifier;
    }

    #[inline]
    pub fn set_discord(&mut self, discord: Option<DiscordClient>) {
        self.discord = discord;
    }
}

impl App {
    #[inline]
    pub fn get_mut_cli(&mut self) -> &mut Cli {
        &mut self.cli
    }

    #[inline]
    pub fn get_mut_paths(&mut self) -> &mut Paths {
        &mut self.paths
    }

    #[inline]
    pub fn get_mut_config(&mut self) -> &mut Config {
        &mut self.config
    }

    #[inline]
    pub fn get_mut_state(&mut self) -> &mut State {
        &mut self.state
    }

    #[inline]
    pub fn get_mut_client(&mut self) -> &mut GithubClient {
        &mut self.client
    }

    #[inline]
    pub fn get_mut_notifier(&mut self) -> &mut Notifier {
        &mut self.notifier
    }

    #[inline]
    pub fn get_mut_discord(&mut self) -> &mut Option<DiscordClient> {
        &mut self.discord
    }
}

impl App {
    pub fn run(&mut self) -> Result<i32> {
        if self.cli.get_help() {
            println!("{}", self.cli.usage());

            return Ok(0);
        }

        if self.cli.get_version() {
            println!("{}", env!("CARGO_PKG_VERSION"));

            return Ok(0);
        }

        if self.cli.get_test_webhook() {
            let Some(discord) = self.discord.as_ref() else {
                eprintln!("error: Discord webhook is not configured");

                return Ok(1);
            };

            return match discord.send_test() {
                Ok(channel) => {
                    println!("Discord webhook valid: #{channel}");
                    println!("Test message sent.");

                    Ok(0)
                }
                Err(error) => {
                    eprintln!("error: {error}");

                    Ok(1)
                }
            };
        }

        if self.cli.get_flush() {
            if let Err(error) = self.notifier.ensure_connection() {
                eprintln!("warning: D-Bus is not available: {error}");

                return Ok(0);
            }

            self.flush_pending()?;

            return Ok(0);
        }

        if let Some(tag) = self.cli.get_tag().map(str::to_string) {
            self.backfill(&tag)?;

            return Ok(0);
        }

        if self.cli.get_check() || self.cli.get_simulate() || self.cli.get_no_notify() {
            self.check_once()?;

            return Ok(0);
        }

        self.run_daemon()
    }

    fn run_daemon(&mut self) -> Result<i32> {
        let mut failures: u32 = 0;

        loop {
            match self.check_once() {
                Ok(()) => failures = 0,
                Err(error) => {
                    eprintln!("warning: check failed: {error}");

                    failures = failures.saturating_add(1);
                }
            }

            if let Err(error) = self.flush_pending() {
                eprintln!("warning: could not deliver pending notifications: {error}");
            }

            let base: u64 = self.config.get_poll_interval_secs();

            let seconds: u64 = if failures == 0 || failures >= 6 {
                base
            } else {
                (30u64 << failures.saturating_sub(1)).min(base)
            };

            let timeout: Duration = Duration::from_secs(seconds);

            if let Some(event) = self.notifier.next_event(timeout) {
                match event {
                    NotifierEvent::NotificationsAvailable => {
                        if let Err(error) = self.flush_pending() {
                            eprintln!("warning: could not deliver pending notifications: {error}");
                        }
                    }
                    other => {
                        if let Err(error) = self.notifier.handle_event(other) {
                            eprintln!("warning: notification event failed: {error}");
                        }
                    }
                }
            }
        }
    }

    pub fn check_once(&mut self) -> Result<()> {
        if self.seed_if_needed()? {
            return Ok(());
        }

        let Some(latest) = self.fetch_current_release()? else {
            return Ok(());
        };

        let Some(latest_version) =
            Version::parse(latest.get_tag_name(), self.config.get_tag_prefix())
        else {
            eprintln!(
                "warning: release ignored due to incompatible tag prefix: {}",
                latest.get_tag_name()
            );

            return Ok(());
        };

        let last_version = self
            .state
            .get_last_tag()
            .and_then(|tag| Version::parse(tag, self.config.get_tag_prefix()));

        if let Some(last) = &last_version
            && !latest_version.is_newer_than(last)
        {
            if !self.cli.get_simulate() {
                self.state.set_last_checked_unix(self.now_unix());
                self.state
                    .set_etag(self.client.get_etag().map(str::to_string));
                self.state.save(&self.paths)?;
            }

            return Ok(());
        }

        let last_tag: Option<String> = self.state.get_last_tag().map(str::to_string);

        let include_prerelease: bool = self.config.get_include_prerelease();

        let mut releases: Vec<Release> = self.client.fetch_releases()?;

        releases.retain(|release| {
            if release.get_draft() {
                return false;
            }

            if !include_prerelease && release.get_prerelease() {
                return false;
            }

            let Some(version) =
                Version::parse(release.get_tag_name(), self.config.get_tag_prefix())
            else {
                return false;
            };

            let after_last: bool = last_version
                .as_ref()
                .is_none_or(|last| version.is_newer_than(last));

            let not_after_latest: bool = !version.is_newer_than(&latest_version);

            after_last && not_after_latest
        });

        releases.sort_by_key(|release| {
            Version::parse(release.get_tag_name(), self.config.get_tag_prefix())
        });

        if releases.is_empty() {
            releases.push(latest.clone());
        }

        let notify: bool = !self.cli.get_no_notify() && !self.cli.get_simulate();

        let mut previous_tag: Option<String> = last_tag;

        releases.into_iter().try_for_each(|release| -> Result<()> {
            self.process_release(&release, previous_tag.as_deref(), notify)?;

            previous_tag = Some(release.get_tag_name().to_string());

            Ok(())
        })?;

        if !self.cli.get_simulate() {
            self.state
                .set_repository(self.config.get_repository().to_string());

            self.state
                .set_last_tag(Some(latest.get_tag_name().to_string()));

            self.state
                .set_etag(self.client.get_etag().map(str::to_string));

            self.state.set_last_checked_unix(self.now_unix());

            self.state.save(&self.paths)?;
        }

        if notify && let Err(error) = self.flush_pending() {
            eprintln!("warning: could not deliver pending notifications: {error}");
        }

        Ok(())
    }

    pub fn backfill(&mut self, tag: &str) -> Result<()> {
        let release = self.client.fetch_release_by_tag(tag)?;

        let Some(version) = Version::parse(release.get_tag_name(), self.config.get_tag_prefix())
        else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("tag does not match the configured prefix: {tag}"),
            )
            .into());
        };

        let previous_tag: Option<String> = self.find_previous_tag(&version)?;

        let notify: bool = !self.cli.get_no_notify() && !self.cli.get_simulate();

        self.process_release(&release, previous_tag.as_deref(), notify)?;

        if notify {
            self.flush_pending()?;
        }

        if !self.cli.get_simulate() {
            let current_last = self
                .state
                .get_last_tag()
                .and_then(|tag| Version::parse(tag, self.config.get_tag_prefix()));

            if current_last.is_none_or(|last| version.is_newer_than(&last)) {
                self.state
                    .set_repository(self.config.get_repository().to_string());

                self.state
                    .set_last_tag(Some(release.get_tag_name().to_string()));

                self.state
                    .set_etag(self.client.get_etag().map(str::to_string));

                self.state.set_last_checked_unix(self.now_unix());

                self.state.save(&self.paths)?;
            }
        }

        Ok(())
    }

    fn seed_if_needed(&mut self) -> Result<bool> {
        let repository_changed: bool = self.state.get_repository() != self.config.get_repository();

        let missing_tag: bool = self.state.get_last_tag().is_none();

        if !repository_changed && !missing_tag {
            return Ok(false);
        }

        if repository_changed {
            self.client.set_etag(None);
        }

        let release: Option<Release> = self.fetch_current_release()?;

        let Some(release) = release else {
            return Ok(true);
        };

        if !self.cli.get_simulate() {
            self.state
                .set_repository(self.config.get_repository().to_string());

            self.state
                .set_last_tag(Some(release.get_tag_name().to_string()));

            self.state
                .set_etag(self.client.get_etag().map(str::to_string));

            self.state.set_last_checked_unix(self.now_unix());

            self.state.set_pending(Vec::new());
            self.state.set_delivered(Vec::new());

            self.state.save(&self.paths)?;
        }

        if self.cli.get_verbose() {
            eprintln!(
                "initial state seeded at {} (no notification)",
                release.get_tag_name()
            );
        }

        Ok(true)
    }

    fn fetch_current_release(&mut self) -> Result<Option<Release>> {
        if self.config.get_include_prerelease() {
            let releases = self.client.fetch_releases()?;

            let latest = releases
                .into_iter()
                .filter(|release| !release.get_draft())
                .filter(|release| {
                    Version::parse(release.get_tag_name(), self.config.get_tag_prefix()).is_some()
                })
                .max_by_key(|release| {
                    Version::parse(release.get_tag_name(), self.config.get_tag_prefix())
                });

            Ok(latest)
        } else {
            self.client.fetch_latest()
        }
    }

    fn find_previous_tag(&self, target: &Version) -> Result<Option<String>> {
        let include_prerelease: bool = self.config.get_include_prerelease();

        let releases: Vec<Release> = self.client.fetch_releases()?;

        let previous: Option<String> = releases
            .into_iter()
            .filter(|release| !release.get_draft())
            .filter(|release| include_prerelease || !release.get_prerelease())
            .filter_map(|release| {
                Version::parse(release.get_tag_name(), self.config.get_tag_prefix())
                    .map(|version| (version, release.get_tag_name().to_string()))
            })
            .filter(|(version, _tag)| version < target)
            .max_by(|(left, _), (right, _)| left.cmp(right))
            .map(|(_version, tag)| tag);

        Ok(previous)
    }

    fn process_release(
        &mut self,
        release: &Release,
        previous_tag: Option<&str>,
        notify: bool,
    ) -> Result<()> {
        let changelog: Changelog =
            Changelog::generate(&self.client, &self.config, release, previous_tag)?;

        let Some(version) = Version::parse(release.get_tag_name(), self.config.get_tag_prefix())
        else {
            return Ok(());
        };

        let folder: std::path::PathBuf = self
            .paths
            .changelogs_dir()
            .join(version.folder_name(self.config.get_version_dir_prefix()));

        changelog.write(&folder, self.config.get_display_name())?;

        if notify
            && let Some(discord) = self.discord.as_ref()
            && let Err(error) = discord.send_release(&changelog, release, &folder, &self.config)
        {
            eprintln!("warning: Discord delivery failed: {error}");
        }

        if notify {
            let pending: PendingNotification = PendingNotification::new(
                release.get_tag_name(),
                &version.to_string(),
                &folder.to_string_lossy(),
                release.get_html_url(),
                changelog.get_total_commits(),
                self.now_unix(),
            );

            self.state.get_mut_pending().push(pending);
        }

        if self.cli.get_verbose() {
            eprintln!(
                "{} v{}: {} commits, changelog at {}",
                self.config.get_display_name(),
                version,
                changelog.get_total_commits(),
                folder.display()
            );
        }

        Ok(())
    }

    pub fn flush_pending(&mut self) -> Result<()> {
        if self.cli.get_simulate() || self.cli.get_no_notify() {
            return Ok(());
        }

        if self.state.get_pending().is_empty() {
            return Ok(());
        }

        self.notifier.ensure_connection()?;

        let pending: Vec<PendingNotification> = std::mem::take(self.state.get_mut_pending());

        let mut delivered: Vec<String> = Vec::<String>::new();

        let mut remaining: Vec<PendingNotification> = Vec::<PendingNotification>::new();

        let notifier: &mut Notifier = &mut self.notifier;

        let verbose: bool = self.cli.get_verbose();

        pending.into_iter().for_each(|item| {
            let folder: &Path = Path::new(item.get_folder());

            match notifier.send(item.get_version(), item.get_commits(), folder) {
                Ok(id) => {
                    if verbose {
                        eprintln!("notification {id} sent for {}", item.get_tag());
                    }

                    delivered.push(item.get_tag().to_string());
                }
                Err(error) => {
                    eprintln!("warning: could not notify {}: {error}", item.get_tag());

                    remaining.push(item);
                }
            }
        });

        let delivered_tags: &mut Vec<String> = self.state.get_mut_delivered();

        delivered_tags.extend(delivered);

        let excess = delivered_tags.len().saturating_sub(50);

        if excess > 0 {
            delivered_tags.drain(0..excess);
        }

        self.state.set_pending(remaining);
        self.state.set_last_checked_unix(self.now_unix());
        self.state.save(&self.paths)?;

        Ok(())
    }

    fn now_unix(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0)
    }
}
