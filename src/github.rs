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

use std::time::Duration;

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use serde::Deserialize;

use crate::error::Result;

const PATH_SEGMENT: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

const API_ROOT: &str = "https://api.github.com";
const RAW_ROOT: &str = "https://raw.githubusercontent.com";

#[derive(Debug, Clone, Deserialize)]
pub struct Release {
    #[serde(default)]
    tag_name: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    html_url: String,
    #[serde(default)]
    published_at: String,
    #[serde(default)]
    prerelease: bool,
    #[serde(default)]
    draft: bool,
}

impl Release {
    #[inline]
    pub fn new(
        tag_name: &str,
        name: &str,
        html_url: &str,
        published_at: &str,
        prerelease: bool,
        draft: bool,
    ) -> Release {
        Release {
            tag_name: tag_name.to_string(),
            name: name.to_string(),
            html_url: html_url.to_string(),
            published_at: published_at.to_string(),
            prerelease,
            draft,
        }
    }
}

impl Release {
    #[inline]
    pub fn get_tag_name(&self) -> &str {
        &self.tag_name
    }

    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn get_html_url(&self) -> &str {
        &self.html_url
    }

    #[inline]
    pub fn get_published_at(&self) -> &str {
        &self.published_at
    }

    #[inline]
    pub fn get_prerelease(&self) -> bool {
        self.prerelease
    }

    #[inline]
    pub fn get_draft(&self) -> bool {
        self.draft
    }
}

impl Release {
    #[inline]
    pub fn set_tag_name(&mut self, tag_name: String) {
        self.tag_name = tag_name;
    }

    #[inline]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[inline]
    pub fn set_html_url(&mut self, html_url: String) {
        self.html_url = html_url;
    }

    #[inline]
    pub fn set_published_at(&mut self, published_at: String) {
        self.published_at = published_at;
    }

    #[inline]
    pub fn set_prerelease(&mut self, prerelease: bool) {
        self.prerelease = prerelease;
    }

    #[inline]
    pub fn set_draft(&mut self, draft: bool) {
        self.draft = draft;
    }
}

impl Release {
    #[inline]
    pub fn get_mut_tag_name(&mut self) -> &mut String {
        &mut self.tag_name
    }

    #[inline]
    pub fn get_mut_name(&mut self) -> &mut String {
        &mut self.name
    }

    #[inline]
    pub fn get_mut_html_url(&mut self) -> &mut String {
        &mut self.html_url
    }

    #[inline]
    pub fn get_mut_published_at(&mut self) -> &mut String {
        &mut self.published_at
    }

    #[inline]
    pub fn get_mut_prerelease(&mut self) -> &mut bool {
        &mut self.prerelease
    }

    #[inline]
    pub fn get_mut_draft(&mut self) -> &mut bool {
        &mut self.draft
    }
}

#[derive(Debug, Clone)]
pub struct Commit {
    sha: String,
    summary: String,
    author: String,
    html_url: String,
}

impl Commit {
    #[inline]
    pub fn new(sha: &str, summary: &str, author: &str, html_url: &str) -> Commit {
        Commit {
            sha: sha.to_string(),
            summary: summary.to_string(),
            author: author.to_string(),
            html_url: html_url.to_string(),
        }
    }
}

impl Commit {
    #[inline]
    pub fn get_sha(&self) -> &str {
        &self.sha
    }

    #[inline]
    pub fn get_summary(&self) -> &str {
        &self.summary
    }

    #[inline]
    pub fn get_author(&self) -> &str {
        &self.author
    }

    #[inline]
    pub fn get_html_url(&self) -> &str {
        &self.html_url
    }
}

impl Commit {
    #[inline]
    pub fn set_sha(&mut self, sha: String) {
        self.sha = sha;
    }

    #[inline]
    pub fn set_summary(&mut self, summary: String) {
        self.summary = summary;
    }

    #[inline]
    pub fn set_author(&mut self, author: String) {
        self.author = author;
    }

    #[inline]
    pub fn set_html_url(&mut self, html_url: String) {
        self.html_url = html_url;
    }
}

impl Commit {
    #[inline]
    pub fn get_mut_sha(&mut self) -> &mut String {
        &mut self.sha
    }

    #[inline]
    pub fn get_mut_summary(&mut self) -> &mut String {
        &mut self.summary
    }

    #[inline]
    pub fn get_mut_author(&mut self) -> &mut String {
        &mut self.author
    }

    #[inline]
    pub fn get_mut_html_url(&mut self) -> &mut String {
        &mut self.html_url
    }
}

#[derive(Debug, Clone)]
pub struct ComparePage {
    commits: Vec<Commit>,
    total_commits: usize,
}

impl ComparePage {
    #[inline]
    pub fn new(commits: Vec<Commit>, total_commits: usize) -> ComparePage {
        ComparePage {
            commits,
            total_commits,
        }
    }
}

impl ComparePage {
    #[inline]
    pub fn get_commits(&self) -> &[Commit] {
        &self.commits
    }

    #[inline]
    pub fn get_total_commits(&self) -> usize {
        self.total_commits
    }
}

impl ComparePage {
    #[inline]
    pub fn set_commits(&mut self, commits: Vec<Commit>) {
        self.commits = commits;
    }

    #[inline]
    pub fn set_total_commits(&mut self, total_commits: usize) {
        self.total_commits = total_commits;
    }
}

impl ComparePage {
    #[inline]
    pub fn get_mut_commits(&mut self) -> &mut Vec<Commit> {
        &mut self.commits
    }

    #[inline]
    pub fn get_mut_total_commits(&mut self) -> &mut usize {
        &mut self.total_commits
    }
}

#[derive(Debug)]
struct HttpResponse {
    status: u16,
    body: String,
    etag: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct ApiAuthor {
    #[serde(default)]
    name: String,
}

#[derive(Debug, Default, Deserialize)]
struct ApiCommitDetails {
    #[serde(default)]
    message: String,
    #[serde(default)]
    author: ApiAuthor,
}

#[derive(Debug, Deserialize)]
struct ApiCommit {
    #[serde(default)]
    sha: String,
    #[serde(default)]
    html_url: String,
    #[serde(default)]
    commit: ApiCommitDetails,
}

#[derive(Debug, Deserialize)]
struct CompareResponse {
    #[serde(default)]
    commits: Vec<ApiCommit>,
    #[serde(default)]
    total_commits: usize,
}

#[derive(Debug, Clone)]
pub struct GithubClient {
    agent: ureq::Agent,
    repository: String,
    token: Option<String>,
    etag: Option<String>,
    max_response_bytes: usize,
    max_commits: usize,
}

impl GithubClient {
    pub fn new(
        repository: &str,
        token: Option<String>,
        max_response_bytes: usize,
        max_commits: usize,
    ) -> GithubClient {
        let config: ureq::config::Config = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .timeout_global(Some(Duration::from_secs(30)))
            .user_agent("LLVMNotificator")
            .build();

        GithubClient {
            agent: ureq::Agent::new_with_config(config),
            repository: repository.to_string(),
            token,
            etag: None,
            max_response_bytes,
            max_commits,
        }
    }
}

impl GithubClient {
    #[inline]
    pub fn get_repository(&self) -> &str {
        &self.repository
    }

    #[inline]
    pub fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    #[inline]
    pub fn get_etag(&self) -> Option<&str> {
        self.etag.as_deref()
    }

    #[inline]
    pub fn get_max_response_bytes(&self) -> usize {
        self.max_response_bytes
    }

    #[inline]
    pub fn get_max_commits(&self) -> usize {
        self.max_commits
    }
}

impl GithubClient {
    #[inline]
    pub fn set_repository(&mut self, repository: String) {
        self.repository = repository;
    }

    #[inline]
    pub fn set_token(&mut self, token: Option<String>) {
        self.token = token;
    }

    #[inline]
    pub fn set_etag(&mut self, etag: Option<String>) {
        self.etag = etag;
    }

    #[inline]
    pub fn set_max_response_bytes(&mut self, max_response_bytes: usize) {
        self.max_response_bytes = max_response_bytes;
    }

    #[inline]
    pub fn set_max_commits(&mut self, max_commits: usize) {
        self.max_commits = max_commits;
    }
}

impl GithubClient {
    #[inline]
    pub fn get_mut_repository(&mut self) -> &mut String {
        &mut self.repository
    }

    #[inline]
    pub fn get_mut_token(&mut self) -> &mut Option<String> {
        &mut self.token
    }

    #[inline]
    pub fn get_mut_etag(&mut self) -> &mut Option<String> {
        &mut self.etag
    }

    #[inline]
    pub fn get_mut_max_response_bytes(&mut self) -> &mut usize {
        &mut self.max_response_bytes
    }

    #[inline]
    pub fn get_mut_max_commits(&mut self) -> &mut usize {
        &mut self.max_commits
    }
}

impl GithubClient {
    pub fn fetch_latest(&mut self) -> Result<Option<Release>> {
        let url: String = format!("{API_ROOT}/repos/{}/releases/latest", self.repository);
        let response: HttpResponse = self.request(&url, self.etag.as_deref())?;

        match response.status {
            304 => Ok(None),
            200 => {
                let release: Release = serde_json::from_str::<Release>(&response.body)?;

                self.etag = response.etag;

                Ok(Some(release))
            }
            status => Err(std::io::Error::other(format!(
                "GitHub returned HTTP {status}: {}",
                response.body.chars().take(300).collect::<String>()
            ))
            .into()),
        }
    }

    pub fn fetch_releases(&self) -> Result<Vec<Release>> {
        let url: String = format!("{API_ROOT}/repos/{}/releases?per_page=100", self.repository);
        let response: HttpResponse = self.request(&url, None)?;

        if response.status != 200 {
            return Err(std::io::Error::other(format!(
                "GitHub returned HTTP {}: {}",
                response.status,
                response.body.chars().take(300).collect::<String>()
            ))
            .into());
        }

        Ok(serde_json::from_str::<Vec<Release>>(&response.body)?)
    }

    pub fn fetch_release_by_tag(&self, tag: &str) -> Result<Release> {
        let encoded: String = self::utf8_percent_encode(tag, PATH_SEGMENT).to_string();
        let url: String = format!(
            "{API_ROOT}/repos/{}/releases/tags/{encoded}",
            self.repository
        );

        let response: HttpResponse = self.request(&url, None)?;

        if response.status != 200 {
            return Err(std::io::Error::other(format!(
                "GitHub returned HTTP {} for {tag}: {}",
                response.status,
                response.body.chars().take(300).collect::<String>()
            ))
            .into());
        }

        Ok(serde_json::from_str::<Release>(&response.body)?)
    }

    pub fn fetch_notes(&self, tag: &str, notes_path: &str) -> Result<Option<String>> {
        let encoded_tag: String = self::utf8_percent_encode(tag, PATH_SEGMENT).to_string();

        let encoded_path: String = notes_path
            .split('/')
            .map(|segment| self::utf8_percent_encode(segment, PATH_SEGMENT).to_string())
            .collect::<Vec<String>>()
            .join("/");

        let url: String = format!(
            "{RAW_ROOT}/{}/{encoded_tag}/{encoded_path}",
            self.repository
        );

        let response: HttpResponse = self.request(&url, None)?;

        match response.status {
            200 => Ok(Some(response.body)),
            404 => Ok(None),
            status => Err(std::io::Error::other(format!(
                "raw.githubusercontent.com returned HTTP {status} for {notes_path}"
            ))
            .into()),
        }
    }

    pub fn fetch_compare(&self, previous: &str, current: &str) -> Result<ComparePage> {
        let encoded_previous: String =
            self::utf8_percent_encode(previous, PATH_SEGMENT).to_string();
        let encoded_current: String = self::utf8_percent_encode(current, PATH_SEGMENT).to_string();

        let url = format!(
            "{API_ROOT}/repos/{}/compare/{encoded_previous}...{encoded_current}",
            self.repository
        );

        let response: HttpResponse = self.request(&url, None)?;

        if response.status != 200 {
            return Err(std::io::Error::other(format!(
                "GitHub compare returned HTTP {}: {}",
                response.status,
                response.body.chars().take(300).collect::<String>()
            ))
            .into());
        }

        let parsed: CompareResponse = serde_json::from_str::<CompareResponse>(&response.body)?;

        let total_commits: usize = parsed.total_commits.max(parsed.commits.len());

        let commits = parsed
            .commits
            .into_iter()
            .take(self.max_commits)
            .map(|item| {
                let summary = item
                    .commit
                    .message
                    .lines()
                    .next()
                    .unwrap_or_default()
                    .to_string();

                Commit::new(
                    &item.sha,
                    &summary,
                    &item.commit.author.name,
                    &item.html_url,
                )
            })
            .collect::<Vec<Commit>>();

        Ok(ComparePage::new(commits, total_commits))
    }

    fn request(&self, url: &str, etag: Option<&str>) -> Result<HttpResponse> {
        let mut attempt: u32 = 0u32;
        let mut last_error: String = String::from("request failed");

        while attempt < 3 {
            let mut request = self
                .agent
                .get(url)
                .header("Accept", "application/vnd.github+json")
                .header("X-GitHub-Api-Version", "2022-11-28");

            if let Some(token) = &self.token {
                request = request.header("Authorization", &format!("Bearer {token}"));
            }

            if let Some(value) = etag {
                request = request.header("If-None-Match", value);
            }

            match request.call() {
                Ok(mut response) => {
                    let status: u16 = response.status().as_u16();

                    let response_etag: Option<String> = response
                        .headers()
                        .get("etag")
                        .and_then(|value| value.to_str().ok())
                        .map(str::to_string);

                    if status == 304 || status == 204 {
                        return Ok(HttpResponse {
                            status,
                            body: String::new(),
                            etag: response_etag,
                        });
                    }

                    let body: String = response
                        .body_mut()
                        .with_config()
                        .limit(self.max_response_bytes as u64)
                        .read_to_string()?;

                    if status == 429 || status >= 500 {
                        last_error = format!("HTTP {status}");
                    } else {
                        return Ok(HttpResponse {
                            status,
                            body,
                            etag: response_etag,
                        });
                    }
                }
                Err(error) => {
                    last_error = error.to_string();
                }
            }

            attempt += 1;

            if attempt < 3 {
                std::thread::sleep(Duration::from_secs(1u64 << attempt));
            }
        }

        Err(std::io::Error::other(last_error).into())
    }
}
