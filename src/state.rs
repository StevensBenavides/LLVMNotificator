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

use serde::{Deserialize, Serialize};

use crate::config::Paths;
use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingNotification {
    tag: String,
    version: String,
    folder: String,
    github_url: String,
    commits: usize,
    created_unix: u64,
}

impl PendingNotification {
    #[inline]
    pub fn new(
        tag: &str,
        version: &str,
        folder: &str,
        github_url: &str,
        commits: usize,
        created_unix: u64,
    ) -> PendingNotification {
        PendingNotification {
            tag: tag.to_string(),
            version: version.to_string(),
            folder: folder.to_string(),
            github_url: github_url.to_string(),
            commits,
            created_unix,
        }
    }
}

impl PendingNotification {
    #[inline]
    pub fn get_tag(&self) -> &str {
        &self.tag
    }

    #[inline]
    pub fn get_version(&self) -> &str {
        &self.version
    }

    #[inline]
    pub fn get_folder(&self) -> &str {
        &self.folder
    }

    #[inline]
    pub fn get_github_url(&self) -> &str {
        &self.github_url
    }

    #[inline]
    pub fn get_commits(&self) -> usize {
        self.commits
    }

    #[inline]
    pub fn get_created_unix(&self) -> u64 {
        self.created_unix
    }
}

impl PendingNotification {
    #[inline]
    pub fn set_tag(&mut self, tag: String) {
        self.tag = tag;
    }

    #[inline]
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    #[inline]
    pub fn set_folder(&mut self, folder: String) {
        self.folder = folder;
    }

    #[inline]
    pub fn set_github_url(&mut self, github_url: String) {
        self.github_url = github_url;
    }

    #[inline]
    pub fn set_commits(&mut self, commits: usize) {
        self.commits = commits;
    }

    #[inline]
    pub fn set_created_unix(&mut self, created_unix: u64) {
        self.created_unix = created_unix;
    }
}

impl PendingNotification {
    #[inline]
    pub fn get_mut_tag(&mut self) -> &mut String {
        &mut self.tag
    }

    #[inline]
    pub fn get_mut_version(&mut self) -> &mut String {
        &mut self.version
    }

    #[inline]
    pub fn get_mut_folder(&mut self) -> &mut String {
        &mut self.folder
    }

    #[inline]
    pub fn get_mut_github_url(&mut self) -> &mut String {
        &mut self.github_url
    }

    #[inline]
    pub fn get_mut_commits(&mut self) -> &mut usize {
        &mut self.commits
    }

    #[inline]
    pub fn get_mut_created_unix(&mut self) -> &mut u64 {
        &mut self.created_unix
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default = "State::new")]
pub struct State {
    schema_version: u32,
    repository: String,
    last_tag: Option<String>,
    etag: Option<String>,
    last_checked_unix: u64,
    pending: Vec<PendingNotification>,
    delivered: Vec<String>,
}

impl State {
    #[inline]
    pub fn new() -> State {
        State {
            schema_version: 1,
            repository: String::new(),
            last_tag: None,
            etag: None,
            last_checked_unix: 0,
            pending: Vec::new(),
            delivered: Vec::new(),
        }
    }
}

impl State {
    #[inline]
    pub fn get_schema_version(&self) -> u32 {
        self.schema_version
    }

    #[inline]
    pub fn get_repository(&self) -> &str {
        &self.repository
    }

    #[inline]
    pub fn get_last_tag(&self) -> Option<&str> {
        self.last_tag.as_deref()
    }

    #[inline]
    pub fn get_etag(&self) -> Option<&str> {
        self.etag.as_deref()
    }

    #[inline]
    pub fn get_last_checked_unix(&self) -> u64 {
        self.last_checked_unix
    }

    #[inline]
    pub fn get_pending(&self) -> &[PendingNotification] {
        &self.pending
    }

    #[inline]
    pub fn get_delivered(&self) -> &[String] {
        &self.delivered
    }
}

impl State {
    #[inline]
    pub fn set_schema_version(&mut self, schema_version: u32) {
        self.schema_version = schema_version;
    }

    #[inline]
    pub fn set_repository(&mut self, repository: String) {
        self.repository = repository;
    }

    #[inline]
    pub fn set_last_tag(&mut self, last_tag: Option<String>) {
        self.last_tag = last_tag;
    }

    #[inline]
    pub fn set_etag(&mut self, etag: Option<String>) {
        self.etag = etag;
    }

    #[inline]
    pub fn set_last_checked_unix(&mut self, last_checked_unix: u64) {
        self.last_checked_unix = last_checked_unix;
    }

    #[inline]
    pub fn set_pending(&mut self, pending: Vec<PendingNotification>) {
        self.pending = pending;
    }

    #[inline]
    pub fn set_delivered(&mut self, delivered: Vec<String>) {
        self.delivered = delivered;
    }
}

impl State {
    #[inline]
    pub fn get_mut_schema_version(&mut self) -> &mut u32 {
        &mut self.schema_version
    }

    #[inline]
    pub fn get_mut_repository(&mut self) -> &mut String {
        &mut self.repository
    }

    #[inline]
    pub fn get_mut_last_tag(&mut self) -> &mut Option<String> {
        &mut self.last_tag
    }

    #[inline]
    pub fn get_mut_etag(&mut self) -> &mut Option<String> {
        &mut self.etag
    }

    #[inline]
    pub fn get_mut_last_checked_unix(&mut self) -> &mut u64 {
        &mut self.last_checked_unix
    }

    #[inline]
    pub fn get_mut_pending(&mut self) -> &mut Vec<PendingNotification> {
        &mut self.pending
    }

    #[inline]
    pub fn get_mut_delivered(&mut self) -> &mut Vec<String> {
        &mut self.delivered
    }
}

impl State {
    pub fn load(paths: &Paths) -> Result<State> {
        let path: std::path::PathBuf = paths.state_file();

        if !path.is_file() {
            return Ok(State::new());
        }

        let contents: String = std::fs::read_to_string(&path)?;

        match serde_json::from_str::<State>(&contents) {
            Ok(state) => Ok(state),
            Err(error) => {
                eprintln!("warning: invalid state at {}: {error}", path.display());

                let backup: std::path::PathBuf = path.with_extension("json.bak");

                let _ = std::fs::rename(&path, &backup);

                Ok(State::new())
            }
        }
    }

    pub fn save(&self, paths: &Paths) -> Result<()> {
        let path: std::path::PathBuf = paths.state_file();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let serialized: Vec<u8> = serde_json::to_vec_pretty(self)?;

        let temporary: std::path::PathBuf = path.with_extension("json.tmp");

        std::fs::write(&temporary, serialized)?;
        std::fs::rename(&temporary, &path)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::State;

    #[test]
    fn roundtrip() {
        let mut state = State::new();

        state.set_repository("llvm/llvm-project".to_string());
        state.set_last_tag(Some("llvmorg-23.1.4".to_string()));

        let encoded = serde_json::to_string(&state).unwrap();
        let decoded = serde_json::from_str::<State>(&encoded).unwrap();

        assert_eq!(decoded.get_repository(), "llvm/llvm-project");
        assert_eq!(decoded.get_last_tag(), Some("llvmorg-23.1.4"));
    }

    #[test]
    fn missing_fields_use_new_defaults() {
        let decoded = serde_json::from_str::<State>("{}").unwrap();

        assert_eq!(decoded.get_schema_version(), 1);
        assert_eq!(decoded.get_last_tag(), None);
        assert!(decoded.get_pending().is_empty());
    }
}
