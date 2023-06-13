use std::{collections::HashMap, fs, path::Path, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

use crate::errors::{Error, MinosResult};

use super::{
    environment::{EnvName, Environment},
    file::File,
    lang::FileVersion,
};

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

        match version {
            FileVersion::V0_14 => v0_14::MinosParserV0_14::parse_file_content(&file_content),
        }
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
                let mut file_environments = Self::parse_file(&path)?;
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
            FileVersion::V0_14 => v0_14::MinosParserV0_14::parse_file_content(file_content),
        }
    }
}

pub(crate) mod v0_14 {
    use pest::{iterators::Pair, Parser};
    use pest_derive::Parser;
    use std::collections::HashMap;
    use std::path::Path;
    use std::str::FromStr;

    use crate::errors::MinosResult;
    use crate::minos::environment::{EnvName, Environment};
    use crate::minos::file::File;
    use crate::minos::lang::{self, Array, Indentifier, ListValueAttribute};

    #[derive(Debug, Parser)]
    #[grammar = "../assets/minos.pest"]
    pub struct MinosParserV0_14;

    impl MinosParserV0_14 {
        fn parse_tokens(pair: Pair<Rule>) -> MinosResult<Vec<lang::Token>> {
            pair.into_inner().map(Self::parse_token).collect()
        }

        pub(crate) fn parse_token(pair: Pair<Rule>) -> MinosResult<lang::Token> {
            use lang::Token;

            Ok(match pair.as_rule() {
                Rule::file => Token::File(Self::parse_tokens(pair)?),
                Rule::version => Token::Version(lang::FileVersion::from_str(pair.as_str())?),
                Rule::env => Token::Env(Self::parse_tokens(pair)?),
                Rule::resource => Token::Resource(Self::parse_tokens(pair)?),
                Rule::rule => Token::Rule(Self::parse_tokens(pair)?),
                Rule::policy => Token::Policy(Self::parse_tokens(pair)?),
                Rule::allow => Token::Allow(Self::parse_tokens(pair)?),
                Rule::requirement => Token::Requirement(Self::parse_tokens(pair)?),
                Rule::single_value_requirement => {
                    Token::SingleValueRequirement(Self::parse_tokens(pair)?)
                }
                Rule::list_value_attribute => {
                    Token::ListValueAttribute(ListValueAttribute::from_str(pair.as_str())?)
                }
                Rule::array => {
                    let inner_values = pair.into_inner().map(|p| p.as_str()).collect();

                    Token::Array(Array(inner_values))
                }
                Rule::identifier => Token::Identifier(Indentifier(pair.as_str())),
                Rule::string => Token::String(pair.as_str()),
                Rule::resource_id => Token::String(pair.as_str()),
                Rule::single_value_attribute => Token::SingleValueAttribute(
                    lang::SingleValueAttribute::from_str(pair.as_str())?,
                ),
                Rule::single_value_operator => {
                    Token::SingleValueOperator(lang::SingleValueOperator::from_str(pair.as_str())?)
                }
                Rule::list_value_operator => {
                    Token::ListValueOperator(lang::ListValueOperator::from_str(pair.as_str())?)
                }
                Rule::list_value_requirement => {
                    let inner_tokens: MinosResult<Vec<Token>> =
                        pair.into_inner().map(Self::parse_token).collect();
                    Token::ListValueRequirement(inner_tokens?)
                }
                Rule::COMMENT | Rule::char | Rule::WHITESPACE | Rule::EOI => Token::Null,
            })
        }

        pub fn parse_file_content(content: &str) -> MinosResult<HashMap<EnvName, Environment>> {
            let file_rules = MinosParserV0_14::parse(Rule::file, content)?
                .next()
                .unwrap();
            let file_token = MinosParserV0_14::parse_token(file_rules)?;

            Ok(File::try_from(file_token)?.environments().clone())
        }
    }
}
