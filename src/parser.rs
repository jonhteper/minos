use std::{collections::HashMap, fs, path::Path, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

use crate::errors::{Error, MinosResult};

use crate::parser::v0_14::MinosParserV0_14;

use self::tokens::FileVersion;
use self::v0_15::MinosParserV0_15;

use crate::language::environment::{EnvName, Environment};

pub mod tokens;
pub(crate) mod v0_14;
pub(crate) mod v0_15;
pub(crate) mod v0_16;

lazy_static! {
    static ref VERSION_REGEX: Regex =
        Regex::new(r"sintaxis\s*=\s*(?P<VERSION>\d+\.+\d+)").expect("regex sintax error");
}

#[derive(Debug)]
pub struct MinosParser;

impl MinosParser {
    fn get_file_version(content: &str) -> Option<FileVersion> {
        if let Some(captures) = VERSION_REGEX.captures(content) {
            return captures
                .name("VERSION")
                .and_then(|v| FileVersion::from_str(v.as_str()).ok());
        }

        None
    }

    /// Return the list of [Environment] inside a valid minos file.
    pub fn parse_file(path: &Path) -> MinosResult<HashMap<EnvName, Environment>> {
        let file_content = fs::read_to_string(path)?;
        let version = Self::get_file_version(&file_content).ok_or(Error::SintaxisNotSupported)?;

        Self::parse_str(version, &file_content)
    }

    pub fn parse_dir(path: &Path) -> MinosResult<HashMap<EnvName, Environment>> {
        let dir = fs::read_dir(path)?;
        let mut environments = HashMap::new();

        for entry in dir {
            let path = entry?.path();
            if !path.is_file() {
                continue;
            }

            let is_minos_file = path.extension().map(|p| p == "minos").unwrap_or_default();
            if is_minos_file {
                let file_environments = Self::parse_file(&path)?;
                environments.extend(file_environments);
            }
        }

        Ok(environments)
    }

    pub fn parse_str(
        version: FileVersion,
        file_content: &str,
    ) -> MinosResult<HashMap<EnvName, Environment>> {
        match version {
            FileVersion::V0_14 => MinosParserV0_14::parse_file_content(file_content),
            FileVersion::V0_15 => MinosParserV0_15::parse_file_content(file_content),
        }
    }
}
