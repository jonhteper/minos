use std::{fs, path::Path, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

use crate::errors::{Error, MinosResult};

use crate::language::storage::Storage;

use self::tokens::FileVersion;


pub mod tokens;
pub(crate) mod v0_16;

lazy_static! {
    static ref VERSION_REGEX: Regex =
        Regex::new(r"syntax\s*=\s*(?P<VERSION>\d+\.+\d+)").expect("regex syntax error");
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
    pub fn parse_file(path: &Path) -> MinosResult<Storage> {
        let file_content = fs::read_to_string(path)?;
        let version = Self::get_file_version(&file_content).ok_or(Error::SyntaxNotSupported)?;

        Self::parse_str(version, &file_content)
    }

    pub fn parse_dir(path: &Path) -> MinosResult<Storage> {
        let dir = fs::read_dir(path)?;
        let mut storage = Storage::default();

        for entry in dir {
            let path = entry?.path();
            if !path.is_file() {
                continue;
            }

            let is_minos_file = path.extension().map(|p| p == "minos").unwrap_or_default();
            if is_minos_file {
                let file_environments = Self::parse_file(&path)?;
                storage.merge(file_environments);
            }
        }

        Ok(storage)
    }

    pub fn parse_str(
        version: FileVersion,
        file_content: &str,
    ) -> MinosResult<Storage> {
        match version {
            FileVersion::V0_16 => v0_16::MinosParserV0_16::parse_file_content(file_content),
        }
    }
}
