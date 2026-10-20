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

#![allow(clippy::new_without_default)]

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::Result;

#[derive(Debug, Clone, Deserialize)]
pub struct Component {
    name: String,
    notes_path: String,
}

impl Component {
    #[inline]
    pub fn new(name: &str, notes_path: &str) -> Component {
        Component {
            name: name.to_string(),
            notes_path: notes_path.to_string(),
        }
    }
}

impl Component {
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn get_notes_path(&self) -> &str {
        &self.notes_path
    }
}

impl Component {
    #[inline]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[inline]
    pub fn set_notes_path(&mut self, notes_path: String) {
        self.notes_path = notes_path;
    }
}

impl Component {
    #[inline]
    pub fn get_mut_name(&mut self) -> &mut String {
        &mut self.name
    }

    #[inline]
    pub fn get_mut_notes_path(&mut self) -> &mut String {
        &mut self.notes_path
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Webhook {
    #[serde(default = "Webhook::default_enabled")]
    enabled: bool,
    #[serde(default)]
    url: String,
    #[serde(default)]
    username: String,
}

impl Webhook {
    #[inline]
    pub fn new(enabled: bool, url: &str, username: &str) -> Webhook {
        Webhook {
            enabled,
            url: url.to_string(),
            username: username.to_string(),
        }
    }

    #[inline]
    fn default_enabled() -> bool {
        true
    }
}

impl Webhook {
    #[inline]
    pub fn get_enabled(&self) -> bool {
        self.enabled
    }

    #[inline]
    pub fn get_url(&self) -> &str {
        &self.url
    }

    #[inline]
    pub fn get_username(&self) -> &str {
        &self.username
    }
}

impl Webhook {
    #[inline]
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    #[inline]
    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    #[inline]
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
}

impl Webhook {
    #[inline]
    pub fn get_mut_enabled(&mut self) -> &mut bool {
        &mut self.enabled
    }

    #[inline]
    pub fn get_mut_url(&mut self) -> &mut String {
        &mut self.url
    }

    #[inline]
    pub fn get_mut_username(&mut self) -> &mut String {
        &mut self.username
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default = "Config::new")]
pub struct Config {
    repository: String,
    display_name: String,
    tag_prefix: String,
    version_dir_prefix: String,
    include_prerelease: bool,
    poll_interval_secs: u64,
    notification_expire_ms: i32,
    action_label: String,
    max_response_bytes: usize,
    max_commits: usize,
    components: Vec<Component>,
    webhook: Option<Webhook>,
    #[serde(skip)]
    token: Option<String>,
}

impl Config {
    #[inline]
    pub fn new() -> Config {
        Config {
            repository: "llvm/llvm-project".to_string(),
            display_name: "LLVM".to_string(),
            tag_prefix: "llvmorg-".to_string(),
            version_dir_prefix: "v".to_string(),
            include_prerelease: false,
            poll_interval_secs: 5 * 60 * 60,
            notification_expire_ms: 0,
            action_label: "Open folder".to_string(),
            max_response_bytes: 8 * 1024 * 1024,
            max_commits: 1000,
            components: vec![
                Component::new("llvm", "llvm/docs/ReleaseNotes.md"),
                Component::new("clang", "clang/docs/ReleaseNotes.md"),
                Component::new("mlir", "mlir/docs/ReleaseNotes.md"),
                Component::new("flang", "flang/docs/ReleaseNotes.md"),
            ],
            webhook: None,
            token: None,
        }
    }
}

impl Config {
    #[inline]
    pub fn get_repository(&self) -> &str {
        &self.repository
    }

    #[inline]
    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }

    #[inline]
    pub fn get_tag_prefix(&self) -> &str {
        &self.tag_prefix
    }

    #[inline]
    pub fn get_version_dir_prefix(&self) -> &str {
        &self.version_dir_prefix
    }

    #[inline]
    pub fn get_include_prerelease(&self) -> bool {
        self.include_prerelease
    }

    #[inline]
    pub fn get_poll_interval_secs(&self) -> u64 {
        self.poll_interval_secs
    }

    #[inline]
    pub fn get_notification_expire_ms(&self) -> i32 {
        self.notification_expire_ms
    }

    #[inline]
    pub fn get_action_label(&self) -> &str {
        &self.action_label
    }

    #[inline]
    pub fn get_max_response_bytes(&self) -> usize {
        self.max_response_bytes
    }

    #[inline]
    pub fn get_max_commits(&self) -> usize {
        self.max_commits
    }

    #[inline]
    pub fn get_components(&self) -> &[Component] {
        &self.components
    }

    #[inline]
    pub fn get_webhook(&self) -> Option<&Webhook> {
        self.webhook.as_ref()
    }

    #[inline]
    pub fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}

impl Config {
    #[inline]
    pub fn set_repository(&mut self, repository: String) {
        self.repository = repository;
    }

    #[inline]
    pub fn set_display_name(&mut self, display_name: String) {
        self.display_name = display_name;
    }

    #[inline]
    pub fn set_tag_prefix(&mut self, tag_prefix: String) {
        self.tag_prefix = tag_prefix;
    }

    #[inline]
    pub fn set_version_dir_prefix(&mut self, version_dir_prefix: String) {
        self.version_dir_prefix = version_dir_prefix;
    }

    #[inline]
    pub fn set_include_prerelease(&mut self, include_prerelease: bool) {
        self.include_prerelease = include_prerelease;
    }

    #[inline]
    pub fn set_poll_interval_secs(&mut self, poll_interval_secs: u64) {
        self.poll_interval_secs = poll_interval_secs;
    }

    #[inline]
    pub fn set_notification_expire_ms(&mut self, notification_expire_ms: i32) {
        self.notification_expire_ms = notification_expire_ms;
    }

    #[inline]
    pub fn set_action_label(&mut self, action_label: String) {
        self.action_label = action_label;
    }

    #[inline]
    pub fn set_max_response_bytes(&mut self, max_response_bytes: usize) {
        self.max_response_bytes = max_response_bytes;
    }

    #[inline]
    pub fn set_max_commits(&mut self, max_commits: usize) {
        self.max_commits = max_commits;
    }

    #[inline]
    pub fn set_components(&mut self, components: Vec<Component>) {
        self.components = components;
    }

    #[inline]
    pub fn set_webhook(&mut self, webhook: Option<Webhook>) {
        self.webhook = webhook;
    }

    #[inline]
    pub fn set_token(&mut self, token: Option<String>) {
        self.token = token;
    }
}

impl Config {
    #[inline]
    pub fn get_mut_repository(&mut self) -> &mut String {
        &mut self.repository
    }

    #[inline]
    pub fn get_mut_display_name(&mut self) -> &mut String {
        &mut self.display_name
    }

    #[inline]
    pub fn get_mut_tag_prefix(&mut self) -> &mut String {
        &mut self.tag_prefix
    }

    #[inline]
    pub fn get_mut_version_dir_prefix(&mut self) -> &mut String {
        &mut self.version_dir_prefix
    }

    #[inline]
    pub fn get_mut_include_prerelease(&mut self) -> &mut bool {
        &mut self.include_prerelease
    }

    #[inline]
    pub fn get_mut_poll_interval_secs(&mut self) -> &mut u64 {
        &mut self.poll_interval_secs
    }

    #[inline]
    pub fn get_mut_notification_expire_ms(&mut self) -> &mut i32 {
        &mut self.notification_expire_ms
    }

    #[inline]
    pub fn get_mut_max_commits(&mut self) -> &mut usize {
        &mut self.max_commits
    }

    #[inline]
    pub fn get_mut_components(&mut self) -> &mut Vec<Component> {
        &mut self.components
    }

    #[inline]
    pub fn get_mut_webhook(&mut self) -> &mut Option<Webhook> {
        &mut self.webhook
    }

    #[inline]
    pub fn get_mut_token(&mut self) -> &mut Option<String> {
        &mut self.token
    }
}

impl Config {
    pub fn load(paths: &Paths, explicit: Option<&PathBuf>) -> Result<Config> {
        let path: PathBuf = explicit.cloned().unwrap_or_else(|| paths.config_file());

        let mut config: Config = if path.is_file() {
            let contents: String = std::fs::read_to_string(&path)?;

            toml::from_str::<Config>(&contents)?
        } else {
            Config::new()
        };

        config.apply_env();

        config.load_token(paths);

        config.validate()?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let webhook_enabled = config
                .get_webhook()
                .is_some_and(|webhook| webhook.get_enabled());

            if webhook_enabled
                && let Ok(metadata) = std::fs::metadata(&path)
                && metadata.permissions().mode() & 0o077 != 0
            {
                eprintln!(
                    "warning: config contains a webhook URL and is readable by other users; run chmod 600 {}",
                    path.display()
                );
            }
        }

        Ok(config)
    }

    pub fn apply_env(&mut self) {
        if let Ok(value) = std::env::var("LLVM_NOTIFICATOR_REPOSITORY")
            && !value.trim().is_empty()
        {
            self.repository = value.trim().to_string();
        }

        if let Ok(value) = std::env::var("LLVM_NOTIFICATOR_DISPLAY_NAME")
            && !value.trim().is_empty()
        {
            self.display_name = value.trim().to_string();
        }

        if let Ok(value) = std::env::var("LLVM_NOTIFICATOR_POLL_INTERVAL_SECS")
            && let Ok(seconds) = value.trim().parse::<u64>()
        {
            self.poll_interval_secs = seconds;
        }
    }

    pub fn load_token(&mut self, paths: &Paths) {
        if let Ok(value) = std::env::var("GITHUB_TOKEN")
            && !value.trim().is_empty()
        {
            self.token = Some(value.trim().to_string());

            return;
        }

        let path: PathBuf = paths.token_file();

        if let Ok(value) = std::fs::read_to_string(&path) {
            let trimmed = value.trim();

            if !trimmed.is_empty() {
                self.token = Some(trimmed.to_string());
            }
        }
    }

    pub fn validate(&self) -> Result<()> {
        let Some((owner, project)) = self.repository.split_once('/') else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "repository must use owner/project format",
            )
            .into());
        };

        if owner.is_empty() || project.is_empty() || project.contains('/') {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "repository must use owner/project format",
            )
            .into());
        }

        if self.poll_interval_secs < 30 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "poll_interval_secs must be at least 30",
            )
            .into());
        }

        if self.max_response_bytes < 1024 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "max_response_bytes must be at least 1024",
            )
            .into());
        }

        if self.max_commits == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "max_commits must be greater than zero",
            )
            .into());
        }

        if self.components.iter().any(|component| {
            let name = component.get_name();
            let notes_path = component.get_notes_path();

            name.trim().is_empty()
                || notes_path.trim().is_empty()
                || name.contains('/')
                || name.contains('\\')
                || name == "."
                || name == ".."
                || notes_path.starts_with('/')
                || notes_path.split('/').any(|segment| segment == "..")
        }) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "each component needs a safe, non-empty name and notes_path",
            )
            .into());
        }

        let names: HashSet<&str> = self
            .components
            .iter()
            .map(|component| component.get_name())
            .collect();

        if names.len() != self.components.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "component names must be unique",
            )
            .into());
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Paths {
    config_dir: PathBuf,
    state_dir: PathBuf,
    data_dir: PathBuf,
}

impl Paths {
    pub fn new() -> Paths {
        let home: PathBuf = std::env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));

        let config_home: PathBuf = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .filter(|path| path.is_absolute())
            .unwrap_or_else(|| home.join(".config"));

        let state_home: PathBuf = std::env::var_os("XDG_STATE_HOME")
            .map(PathBuf::from)
            .filter(|path| path.is_absolute())
            .unwrap_or_else(|| home.join(".local/state"));

        let data_home: PathBuf = std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .filter(|path| path.is_absolute())
            .unwrap_or_else(|| home.join(".local/share"));

        Paths {
            config_dir: config_home.join("LLVMNotificator"),
            state_dir: state_home.join("LLVMNotificator"),
            data_dir: data_home.join("LLVMNotificator"),
        }
    }
}

impl Paths {
    #[inline]
    pub fn get_config_dir(&self) -> &Path {
        &self.config_dir
    }

    #[inline]
    pub fn get_state_dir(&self) -> &Path {
        &self.state_dir
    }

    #[inline]
    pub fn get_data_dir(&self) -> &Path {
        &self.data_dir
    }
}

impl Paths {
    #[inline]
    pub fn set_config_dir(&mut self, config_dir: PathBuf) {
        self.config_dir = config_dir;
    }

    #[inline]
    pub fn set_state_dir(&mut self, state_dir: PathBuf) {
        self.state_dir = state_dir;
    }

    #[inline]
    pub fn set_data_dir(&mut self, data_dir: PathBuf) {
        self.data_dir = data_dir;
    }
}

impl Paths {
    #[inline]
    pub fn get_mut_config_dir(&mut self) -> &mut PathBuf {
        &mut self.config_dir
    }

    #[inline]
    pub fn get_mut_state_dir(&mut self) -> &mut PathBuf {
        &mut self.state_dir
    }

    #[inline]
    pub fn get_mut_data_dir(&mut self) -> &mut PathBuf {
        &mut self.data_dir
    }
}

impl Paths {
    #[inline]
    pub fn config_file(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    #[inline]
    pub fn token_file(&self) -> PathBuf {
        self.config_dir.join("token")
    }

    #[inline]
    pub fn state_file(&self) -> PathBuf {
        self.state_dir.join("state.json")
    }

    #[inline]
    pub fn changelogs_dir(&self) -> PathBuf {
        self.data_dir.join("changelogs")
    }

    pub fn ensure(&self) -> Result<()> {
        std::fs::create_dir_all(&self.state_dir)?;
        std::fs::create_dir_all(&self.data_dir)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn defaults_are_valid() {
        assert!(Config::new().validate().is_ok());
    }

    #[test]
    fn reject_bad_repository() {
        let mut config = Config::new();

        config.set_repository("llvm".to_string());

        assert!(config.validate().is_err());
    }

    #[test]
    fn partial_toml_uses_new_defaults() {
        let config = toml::from_str::<Config>("repository = \"owner/project\"").unwrap();

        assert_eq!(config.get_repository(), "owner/project");
        assert_eq!(config.get_display_name(), "LLVM");
        assert_eq!(config.get_poll_interval_secs(), 5 * 60 * 60);
        assert_eq!(config.get_components().len(), 4);
    }

    #[test]
    fn webhook_section_defaults_to_enabled() {
        let config = toml::from_str::<Config>(
            "repository = \"owner/project\"\n[webhook]\nurl = \"https://discord.com/api/webhooks/1/token\"\n",
        )
        .unwrap();

        let webhook = config.get_webhook().unwrap();

        assert!(webhook.get_enabled());
        assert_eq!(webhook.get_url(), "https://discord.com/api/webhooks/1/token");
        assert_eq!(webhook.get_username(), "");
    }

    #[test]
    fn webhook_can_be_disabled() {
        let config = toml::from_str::<Config>(
            "repository = \"owner/project\"\n[webhook]\nenabled = false\nurl = \"https://discord.com/api/webhooks/1/token\"\n",
        )
        .unwrap();

        assert!(!config.get_webhook().unwrap().get_enabled());
    }
}
