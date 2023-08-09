use std::collections::HashMap;
use std::sync::Arc;
use std::{fs, path::Path, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

use crate::errors::{Error, MinosResult};

use crate::language::storage::Storage;

use self::tokens::FileVersion;

pub mod tokens;
pub(crate) mod v0_16;
pub(crate) mod v0_16_m;

lazy_static! {
    static ref VERSION_REGEX: Regex =
        Regex::new(r"syntax\s*=\s*(?P<VERSION>\d+\.+\d+M*)").expect("regex syntax error");
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

    /// Build a [Storage] with the file.
    pub(crate) fn parse_file(
        path: &Path,
        values_map: &mut HashMap<String, Arc<str>>,
    ) -> MinosResult<Storage> {
        let file_content = fs::read_to_string(path)?;
        let version = Self::get_file_version(&file_content).ok_or(Error::SyntaxNotSupported)?;

        Self::optimized_parse_str(version, &file_content, values_map)
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
                let file_environments = Self::parse_file(&path, values_map)?;
                storage.merge(file_environments);
            }
        }

        Ok(storage)
    }

    fn optimized_parse_str(
        version: FileVersion,
        file_content: &str,
        values_map: &mut HashMap<String, Arc<str>>,
    ) -> MinosResult<Storage> {
        match version {
            FileVersion::V0_16 => v0_16::MinosParserV0_16::parse_file_content(file_content, values_map),
            FileVersion::V0_16M => {
                v0_16_m::MinosParserV0_16M::parse_file_content(file_content, values_map)
            }
        }
    }

    /// Read and parse a valid minos file content, returns an [Storage]
    /// built with it.
    ///
    /// **WARNING**: Use macro versions (fe. `0.16M`) if the file contains macros.
    ///
    /// ## Errors
    /// * Isn't a valid minos file.
    /// * Is partial minos file.
    /// * File contains syntax errors.
    /// * File uses unsupported syntax version.
    pub fn parse_str(version: FileVersion, file_content: &str) -> MinosResult<Storage> {
        let mut values_map = HashMap::new();
        Self::optimized_parse_str(version, file_content, &mut values_map)
    }
}
