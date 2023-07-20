automod::dir!(pub "src/models");

use std::fs;
use std::path::Path;
use crate::models::league::League;
use crate::models::paths::Paths;

use crate::error::*;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_can_decode_version() {
        let vi_from_str = VersionSpec::decode_from_str("1.23").unwrap();
        assert_eq!(vi_from_str.major, 1);
        assert_eq!(vi_from_str.minor, 23);
        let vi_from_string = VersionSpec::decode_from_string("1.23".to_string()).unwrap();
        assert_eq!(vi_from_string.major, 1);
        assert_eq!(vi_from_string.minor, 23);
    }

    #[test]
    fn it_can_decode_version_with_leading_zeroes() {
        let vi_from_str = VersionSpec::decode_from_str("4.03").unwrap();
        assert_eq!(vi_from_str.major, 4);
        assert_eq!(vi_from_str.minor, 3);
        let vi_from_string = VersionSpec::decode_from_string("4.03".to_string()).unwrap();
        assert_eq!(vi_from_string.major, 4);
        assert_eq!(vi_from_string.minor, 3);
    }
    #[test]
    fn it_can_encode_version() {
        let vi = VersionSpec {
            major: 1,
            minor: 23,
        };
        assert_eq!(vi.encode_to_string(), "1.23".to_string());
    }

    #[test]
    fn it_can_encode_version_with_leading_zeroes() {
        let vi = VersionSpec { major: 4, minor: 3 };
        assert_eq!(vi.encode_to_string(), "4.03".to_string());
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub paths: Paths,
    pub league: League,
}

impl Config {
    pub fn write_to(&self, p: &Path) -> Result<(), CaughtError> {
        fs::write(p, serde_json::to_vec(self).catch_err()?).catch_err()
    }
    pub fn read_from(p: &Path) -> Result<Config, CaughtError> {
        let rdr = fs::read(p).catch_err()?;
        serde_json::from_slice(rdr.as_slice()).catch_err()
    }
}

pub struct VersionInfo {
    pub base_version: VersionSpec,
    pub modpack_version: VersionSpec,
}

pub struct VersionSpec {
    pub minor: u8,
    pub major: u8,
}

impl VersionSpec {
    pub fn empty() -> VersionSpec {
        VersionSpec { minor: 0, major: 0 }
    }
    pub fn decode_from_string(version: String) -> Result<VersionSpec, CaughtError> {
        VersionSpec::decode_from_str(version.as_str())
    }
    pub fn decode_from_str(version: &str) -> Result<VersionSpec, CaughtError> {
        let (major_str, minor_str) = version.split_once('.').catch_none(
            "could not split version string into major and minor component".to_string(),
        )?;
        Ok(VersionSpec {
            minor: minor_str.parse::<u8>().catch_err()?,
            major: major_str.parse::<u8>().catch_err()?,
        })
    }
    pub fn encode_to_string(&self) -> String {
        format!("{}.{:0>2}", self.major, self.minor)
    }

    pub fn incremented(&self) -> VersionSpec {
        VersionSpec {
            major: self.major,
            minor: self.minor + 1,
        }
    }
}
