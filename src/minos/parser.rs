use pest_derive::Parser;

use super::lang::File;


#[derive(Debug, Parser)]
#[grammar = "../assets/minos.pest"]
pub struct MinosParser;

impl MinosParser {
    pub fn parse_file() -> File {
        todo!()
    }
}