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

pub(crate) const TAG_PREFIX: &str = "llvmorg-";
pub(crate) const VERSION_DIR_PREFIX: &str = "v";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    #[inline]
    pub fn new(major: u32, minor: u32, patch: u32) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }
}

impl Version {
    #[inline]
    pub fn get_major(&self) -> u32 {
        self.major
    }

    #[inline]
    pub fn get_minor(&self) -> u32 {
        self.minor
    }

    #[inline]
    pub fn get_patch(&self) -> u32 {
        self.patch
    }
}

impl Version {
    #[inline]
    pub fn set_major(&mut self, major: u32) {
        self.major = major;
    }

    #[inline]
    pub fn set_minor(&mut self, minor: u32) {
        self.minor = minor;
    }

    #[inline]
    pub fn set_patch(&mut self, patch: u32) {
        self.patch = patch;
    }
}

impl Version {
    #[inline]
    pub fn get_mut_major(&mut self) -> &mut u32 {
        &mut self.major
    }

    #[inline]
    pub fn get_mut_minor(&mut self) -> &mut u32 {
        &mut self.minor
    }

    #[inline]
    pub fn get_mut_patch(&mut self) -> &mut u32 {
        &mut self.patch
    }
}

impl Version {
    pub fn parse(tag: &str, prefix: &str) -> Option<Version> {
        let core: &str = tag.strip_prefix(prefix)?.split('-').next()?;

        let mut parts: std::str::Split<'_, char> = core.split('.');

        let major: u32 = parts.next()?.parse::<u32>().ok()?;
        let minor: u32 = parts.next()?.parse::<u32>().ok()?;
        let patch: u32 = parts.next()?.parse::<u32>().ok()?;

        if parts.next().is_some() {
            return None;
        }

        Some(Version::new(major, minor, patch))
    }

    #[inline]
    pub fn is_newer_than(&self, other: &Version) -> bool {
        (self.major, self.minor, self.patch) > (other.major, other.minor, other.patch)
    }

    #[inline]
    pub fn folder_name(&self, prefix: &str) -> String {
        format!("{prefix}{self}")
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::Version;

    #[test]
    fn parse_and_compare() {
        let version = Version::parse("llvmorg-23.1.4", "llvmorg-").unwrap();
        let previous = Version::parse("llvmorg-23.1.3", "llvmorg-").unwrap();

        assert_eq!(version.to_string(), "23.1.4");
        assert!(version.is_newer_than(&previous));
        assert_eq!(version.folder_name("v"), "v23.1.4");
    }

    #[test]
    fn reject_invalid_tags() {
        assert!(Version::parse("llvmorg-23.1", "llvmorg-").is_none());
        assert!(Version::parse("llvmorg-23.1.4.1", "llvmorg-").is_none());
        assert!(Version::parse("release-23.1.4", "llvmorg-").is_none());
    }
}
