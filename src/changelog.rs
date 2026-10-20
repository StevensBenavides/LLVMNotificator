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

#![allow(clippy::too_many_arguments)]

use std::path::Path;

use similar::TextDiff;

use crate::config::Config;
use crate::error::Result;
use crate::github::{Commit, GithubClient, Release};
use crate::version::Version;

#[derive(Debug, Clone)]
pub struct Changelog {
    tag: String,
    version: String,
    previous_tag: Option<String>,
    github_url: String,
    published_at: String,
    notes: Vec<(String, String)>,
    notes_diff: String,
    commits: Vec<Commit>,
    total_commits: usize,
}

impl Changelog {
    pub fn new(
        tag: &str,
        version: &str,
        previous_tag: Option<String>,
        github_url: &str,
        published_at: &str,
        notes: Vec<(String, String)>,
        notes_diff: String,
        commits: Vec<Commit>,
        total_commits: usize,
    ) -> Changelog {
        Changelog {
            tag: tag.to_string(),
            version: version.to_string(),
            previous_tag,
            github_url: github_url.to_string(),
            published_at: published_at.to_string(),
            notes,
            notes_diff,
            commits,
            total_commits,
        }
    }
}

impl Changelog {
    #[inline]
    pub fn get_tag(&self) -> &str {
        &self.tag
    }

    #[inline]
    pub fn get_version(&self) -> &str {
        &self.version
    }

    #[inline]
    pub fn get_previous_tag(&self) -> Option<&str> {
        self.previous_tag.as_deref()
    }

    #[inline]
    pub fn get_github_url(&self) -> &str {
        &self.github_url
    }

    #[inline]
    pub fn get_published_at(&self) -> &str {
        &self.published_at
    }

    #[inline]
    pub fn get_notes(&self) -> &[(String, String)] {
        &self.notes
    }

    #[inline]
    pub fn get_notes_diff(&self) -> &str {
        &self.notes_diff
    }

    #[inline]
    pub fn get_commits(&self) -> &[Commit] {
        &self.commits
    }

    #[inline]
    pub fn get_total_commits(&self) -> usize {
        self.total_commits
    }
}

impl Changelog {
    #[inline]
    pub fn set_tag(&mut self, tag: String) {
        self.tag = tag;
    }

    #[inline]
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    #[inline]
    pub fn set_previous_tag(&mut self, previous_tag: Option<String>) {
        self.previous_tag = previous_tag;
    }

    #[inline]
    pub fn set_github_url(&mut self, github_url: String) {
        self.github_url = github_url;
    }

    #[inline]
    pub fn set_published_at(&mut self, published_at: String) {
        self.published_at = published_at;
    }

    #[inline]
    pub fn set_notes(&mut self, notes: Vec<(String, String)>) {
        self.notes = notes;
    }

    #[inline]
    pub fn set_notes_diff(&mut self, notes_diff: String) {
        self.notes_diff = notes_diff;
    }

    #[inline]
    pub fn set_commits(&mut self, commits: Vec<Commit>) {
        self.commits = commits;
    }

    #[inline]
    pub fn set_total_commits(&mut self, total_commits: usize) {
        self.total_commits = total_commits;
    }
}

impl Changelog {
    #[inline]
    pub fn get_mut_tag(&mut self) -> &mut String {
        &mut self.tag
    }

    #[inline]
    pub fn get_mut_version(&mut self) -> &mut String {
        &mut self.version
    }

    #[inline]
    pub fn get_mut_previous_tag(&mut self) -> &mut Option<String> {
        &mut self.previous_tag
    }

    #[inline]
    pub fn get_mut_github_url(&mut self) -> &mut String {
        &mut self.github_url
    }

    #[inline]
    pub fn get_mut_published_at(&mut self) -> &mut String {
        &mut self.published_at
    }

    #[inline]
    pub fn get_mut_notes(&mut self) -> &mut Vec<(String, String)> {
        &mut self.notes
    }

    #[inline]
    pub fn get_mut_notes_diff(&mut self) -> &mut String {
        &mut self.notes_diff
    }

    #[inline]
    pub fn get_mut_commits(&mut self) -> &mut Vec<Commit> {
        &mut self.commits
    }

    #[inline]
    pub fn get_mut_total_commits(&mut self) -> &mut usize {
        &mut self.total_commits
    }
}

impl Changelog {
    pub fn generate(
        client: &GithubClient,
        config: &Config,
        release: &Release,
        previous_tag: Option<&str>,
    ) -> Result<Changelog> {
        let Some(version) = Version::parse(release.get_tag_name(), config.get_tag_prefix()) else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "tag does not match prefix {}: {}",
                    config.get_tag_prefix(),
                    release.get_tag_name()
                ),
            )
            .into());
        };

        let mut notes: Vec<(String, String)> = Vec::<(String, String)>::new();
        let mut diffs: Vec<String> = Vec::<String>::new();

        config
            .get_components()
            .iter()
            .try_for_each(|component| -> Result<()> {
                if let Some(content) =
                    client.fetch_notes(release.get_tag_name(), component.get_notes_path())?
                {
                    notes.push((component.get_name().to_string(), content.clone()));

                    if let Some(previous) = previous_tag
                        && let Some(previous_content) =
                            client.fetch_notes(previous, component.get_notes_path())?
                    {
                        let diff: String = TextDiff::from_lines(&previous_content, &content)
                            .unified_diff()
                            .context_radius(3)
                            .header(previous, release.get_tag_name())
                            .to_string();

                        diffs.push(format!(
                            "## {}\n\n```diff\n{}\n```\n",
                            component.get_name(),
                            diff
                        ));
                    }
                }

                Ok(())
            })?;

        let (commits, total_commits) = if let Some(previous) = previous_tag {
            let page: crate::github::ComparePage =
                client.fetch_compare(previous, release.get_tag_name())?;

            (page.get_commits().to_vec(), page.get_total_commits())
        } else {
            (Vec::<Commit>::new(), 0usize)
        };

        let notes_diff = if diffs.is_empty() {
            String::new()
        } else {
            diffs.join("\n")
        };

        Ok(Changelog::new(
            release.get_tag_name(),
            &version.to_string(),
            previous_tag.map(str::to_string),
            release.get_html_url(),
            release.get_published_at(),
            notes,
            notes_diff,
            commits,
            total_commits,
        ))
    }

    pub fn write(&self, folder: &Path, display_name: &str) -> Result<()> {
        std::fs::create_dir_all(folder)?;

        let footer = format!("\n---\n\nGitHub: {}\n", self.github_url);

        if let Some((_name, content)) = self.notes.first() {
            std::fs::write(folder.join("ReleaseNotes.md"), format!("{content}{footer}"))?;
        }

        self.notes
            .iter()
            .skip(1)
            .try_for_each(|(name, content)| -> Result<()> {
                let directory = folder.join(name);

                std::fs::create_dir_all(&directory)?;

                std::fs::write(
                    directory.join("ReleaseNotes.md"),
                    format!("{content}{footer}"),
                )?;

                Ok(())
            })?;

        let diff_body: String = if self.notes_diff.is_empty() {
            "No changes in release notes for this version.".to_string()
        } else {
            self.notes_diff.clone()
        };

        std::fs::write(
            folder.join("notes-diff.md"),
            format!(
                "# {display_name} v{} - release notes diff\n\n{}\n{footer}",
                self.version, diff_body
            ),
        )?;

        let commit_lines: String = self
            .commits
            .iter()
            .map(|commit| {
                let short = commit.get_sha().chars().take(7).collect::<String>();

                format!(
                    "- [`{short}`]({}) {} - {}",
                    commit.get_html_url(),
                    commit.get_summary(),
                    commit.get_author()
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let truncated = if self.total_commits > self.commits.len() {
            format!(
                "\n\n> Showing {} of {} commits.\n",
                self.commits.len(),
                self.total_commits
            )
        } else {
            String::new()
        };

        std::fs::write(
            folder.join("commits.md"),
            format!(
                "# {display_name} v{} - commits\n\n{}\n{truncated}{footer}",
                self.version, commit_lines
            ),
        )?;

        let component_lines: String = self
            .notes
            .iter()
            .enumerate()
            .map(|(index, (name, _content))| {
                if index == 0 {
                    "- [ReleaseNotes.md](./ReleaseNotes.md)".to_string()
                } else {
                    format!("- [{name}](./{name}/ReleaseNotes.md)")
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        std::fs::write(
            folder.join("summary.md"),
            format!(
                "# {display_name} v{}\n\n\
                 - Release: {}\n\
                 - Published: {}\n\
                 - Commits: {}\n\
                 - Previous tag: {}\n\n\
                 ## Files\n\n\
                 {}\n\
                 - [notes-diff.md](./notes-diff.md)\n\
                 - [commits.md](./commits.md)\n{footer}",
                self.version,
                self.github_url,
                self.published_at,
                self.total_commits,
                self.previous_tag.as_deref().unwrap_or("none"),
                component_lines
            ),
        )?;

        Ok(())
    }
}
