use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use std::{fs, path::Path, str::FromStr};

use regex::Regex;

use crate::errors::{Error, MinosResult};

use crate::language::storage::Storage;
use crate::Engine;

use self::tokens::FileVersion;

pub mod tokens;
pub(crate) mod v0_16;
pub(crate) mod v0_16_m;

#[cfg(feature = "v0_17")]
pub(crate) mod v0_17;

static VERSION_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"syntax\s*=\s*(?P<VERSION>\d+\.+\d+M*)").expect("regex syntax error"));

/// Parser for Minos files.
///
/// Use it for individual file content, if you want to parse a directory or a file from FS,
/// use [`Container`](crate::container::Container) instead.
///
/// # Usage
/// From text
/// ```rust
/// use minos::parser::MinosParser;
/// use minos::Engine;
///
/// const minos_text: &str =  r#"
/// syntax = 0.16;
/// resource User {
///     env DEFAULT {
///         policy {
///             allow = ["create", "read", "update", "delete"];
///
///             rule {
///                 actor.type = RootUser;
///             }
///
///             rule {
///                 actor.type = resource.type;
///                 actor.id = resource.id;
///             }
///
///             rule {
///                 resource.owner = actor.id;
///             }
///         }
///     }
/// }
/// "#;
///
/// let engine: Engine = MinosParser::easy_parse_str(minos_text).unwrap();
///
///
///
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

    /// Read the directory and parse the files recursively. Build a [Storage] with the files content.
    pub(crate) fn parse_dir(
        path: &Path,
        values_map: &mut HashMap<String, Arc<str>>,
    ) -> MinosResult<Storage> {
        let dir = fs::read_dir(path)?;
        let mut storage = Storage::default();

        for entry in dir {
            let path = entry?.path();
            if !path.is_file() && !path.is_dir() {
                continue;
            }

            if path.is_dir() {
                let dir_storage = Self::parse_dir(&path, values_map)?;
                storage.merge(dir_storage);
            }

            let is_minos_file = path.extension().map(|p| p == "minos").unwrap_or_default();
            if is_minos_file {
                let file_storage = Self::parse_file(&path, values_map)?;
                storage.merge(file_storage);
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
            #[cfg(feature = "v0_17")]
            FileVersion::V0_17 => v0_17::MinosPestParser::parse_file_content(file_content, values_map),
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
    #[cfg(not(feature = "v0_17"))]
    #[deprecated(since = "0.12.0", note = "Use `MinosParser::easy_parse_str` method instead")]
    pub fn parse_str(version: FileVersion, file_content: &str) -> MinosResult<Storage> {
        let mut values_map = HashMap::new();
        Self::optimized_parse_str(version, file_content, &mut values_map)
    }

    /// Read and parse a valid minos file content, returns an [Engine]
    /// built with it.
    ///
    /// ## Errors
    /// * Isn't a valid minos file.
    /// * Is partial minos file.
    /// * File contains syntax errors.
    /// * File uses unsupported syntax version.
    pub fn easy_parse_str(file_content: &str) -> MinosResult<Engine> {
        let mut values_map = HashMap::new();
        let version = Self::get_file_version(&file_content).ok_or(Error::SyntaxNotSupported)?;
        let storage = Self::optimized_parse_str(version, file_content, &mut values_map)?;
        let engine = Engine::from(storage);

        Ok(engine)
    }

    #[cfg(feature = "v0_17")]
    #[inline]
    pub fn parse_str(file_content: &str) -> MinosResult<Engine> {
        Self::easy_parse_str(file_content)
    }
}
