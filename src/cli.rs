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

use std::path::PathBuf;

use crate::error::Result;

const USAGE: &str = "\
LLVMNotificator - GitHub release watcher with desktop notifications

USAGE:
    LLVMNotificator [OPTIONS]

OPTIONS:
    --check          Check for new releases now, notify, and exit
    --flush          Deliver pending notifications and exit
    --test-webhook   Validate the Discord webhook and send a test message
    --simulate       Write changelogs without notifying or updating state
    --no-notify      Generate changelogs without sending notifications
    --tag <tag>      Process a specific release (backfill)
    --config <path>  Use a custom configuration file
    --verbose        Print detailed progress
    -h, --help       Show this help
    -V, --version    Show the version
";

#[derive(Debug)]
pub struct Cli {
    simulate: bool,
    no_notify: bool,
    flush: bool,
    check: bool,
    test_webhook: bool,
    tag: Option<String>,
    config: Option<PathBuf>,
    verbose: bool,
    help: bool,
    version: bool,
}

impl Cli {
    #[inline]
    pub fn new() -> Cli {
        Cli {
            simulate: false,
            no_notify: false,
            flush: false,
            check: false,
            test_webhook: false,
            tag: None,
            config: None,
            verbose: false,
            help: false,
            version: false,
        }
    }
}

impl Cli {
    #[inline]
    pub fn get_simulate(&self) -> bool {
        self.simulate
    }

    #[inline]
    pub fn get_no_notify(&self) -> bool {
        self.no_notify
    }

    #[inline]
    pub fn get_flush(&self) -> bool {
        self.flush
    }

    #[inline]
    pub fn get_check(&self) -> bool {
        self.check
    }

    #[inline]
    pub fn get_test_webhook(&self) -> bool {
        self.test_webhook
    }

    #[inline]
    pub fn get_tag(&self) -> Option<&str> {
        self.tag.as_deref()
    }

    #[inline]
    pub fn get_config(&self) -> Option<&PathBuf> {
        self.config.as_ref()
    }

    #[inline]
    pub fn get_verbose(&self) -> bool {
        self.verbose
    }

    #[inline]
    pub fn get_help(&self) -> bool {
        self.help
    }

    #[inline]
    pub fn get_version(&self) -> bool {
        self.version
    }
}

impl Cli {
    #[inline]
    pub fn set_simulate(&mut self, simulate: bool) {
        self.simulate = simulate;
    }

    #[inline]
    pub fn set_no_notify(&mut self, no_notify: bool) {
        self.no_notify = no_notify;
    }

    #[inline]
    pub fn set_flush(&mut self, flush: bool) {
        self.flush = flush;
    }

    #[inline]
    pub fn set_check(&mut self, check: bool) {
        self.check = check;
    }

    #[inline]
    pub fn set_test_webhook(&mut self, test_webhook: bool) {
        self.test_webhook = test_webhook;
    }

    #[inline]
    pub fn set_tag(&mut self, tag: Option<String>) {
        self.tag = tag;
    }

    #[inline]
    pub fn set_config(&mut self, config: Option<PathBuf>) {
        self.config = config;
    }

    #[inline]
    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    #[inline]
    pub fn set_help(&mut self, help: bool) {
        self.help = help;
    }

    #[inline]
    pub fn set_version(&mut self, version: bool) {
        self.version = version;
    }
}

impl Cli {
    #[inline]
    pub fn get_mut_simulate(&mut self) -> &mut bool {
        &mut self.simulate
    }

    #[inline]
    pub fn get_mut_no_notify(&mut self) -> &mut bool {
        &mut self.no_notify
    }

    #[inline]
    pub fn get_mut_flush(&mut self) -> &mut bool {
        &mut self.flush
    }

    #[inline]
    pub fn get_mut_check(&mut self) -> &mut bool {
        &mut self.check
    }

    #[inline]
    pub fn get_mut_test_webhook(&mut self) -> &mut bool {
        &mut self.test_webhook
    }

    #[inline]
    pub fn get_mut_tag(&mut self) -> &mut Option<String> {
        &mut self.tag
    }

    #[inline]
    pub fn get_mut_config(&mut self) -> &mut Option<PathBuf> {
        &mut self.config
    }

    #[inline]
    pub fn get_mut_verbose(&mut self) -> &mut bool {
        &mut self.verbose
    }

    #[inline]
    pub fn get_mut_help(&mut self) -> &mut bool {
        &mut self.help
    }

    #[inline]
    pub fn get_mut_version(&mut self) -> &mut bool {
        &mut self.version
    }
}

impl Cli {
    pub fn parse(args: &[String]) -> Result<Cli> {
        let mut cli: Cli = Cli::new();

        let mut index: usize = 0usize;

        while index < args.len() {
            let argument: &str = args[index].as_str();

            match argument {
                "--simulate" => cli.set_simulate(true),
                "--no-notify" => cli.set_no_notify(true),
                "--flush" => cli.set_flush(true),
                "--check" => cli.set_check(true),
                "--test-webhook" => cli.set_test_webhook(true),
                "--verbose" => cli.set_verbose(true),
                "-h" | "--help" => cli.set_help(true),
                "-V" | "--version" => cli.set_version(true),
                "--tag" => {
                    index += 1;

                    let value: &String = args.get(index).ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "--tag requires a value",
                        )
                    })?;

                    cli.set_tag(Some(value.clone()));
                }
                "--config" => {
                    index += 1;

                    let value: &String = args.get(index).ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "--config requires a value",
                        )
                    })?;

                    cli.set_config(Some(PathBuf::from(value)));
                }
                _ if argument.starts_with("--tag=") => {
                    let value: &str = argument.trim_start_matches("--tag=");

                    cli.set_tag(Some(value.to_string()));
                }
                _ if argument.starts_with("--config=") => {
                    let value: &str = argument.trim_start_matches("--config=");

                    cli.set_config(Some(PathBuf::from(value)));
                }
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("unknown argument: {argument}"),
                    )
                    .into());
                }
            }

            index += 1;
        }

        Ok(cli)
    }

    #[inline]
    pub fn usage(&self) -> &'static str {
        USAGE
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;

    #[test]
    fn parse_flags_and_values() {
        let args = vec![
            "--check".to_string(),
            "--simulate".to_string(),
            "--tag".to_string(),
            "llvmorg-23.1.4".to_string(),
            "--config=/tmp/config.toml".to_string(),
        ];

        let cli = Cli::parse(&args).unwrap();

        assert!(cli.get_check());
        assert!(cli.get_simulate());
        assert_eq!(cli.get_tag(), Some("llvmorg-23.1.4"));
        assert_eq!(
            cli.get_config()
                .map(|path| path.to_string_lossy().to_string()),
            Some("/tmp/config.toml".to_string())
        );
    }

    #[test]
    fn reject_unknown_argument() {
        let args = vec!["--inventado".to_string()];

        assert!(Cli::parse(&args).is_err());
    }
}
