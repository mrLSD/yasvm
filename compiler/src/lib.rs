#![feature(custom_attribute)]

mod language_test;

use pest::error::Error;
use pest_derive::Parser;
use pest::Parser;

#[derive(Parser, Debug)]
#[grammar = "grammar/language.pest"]
struct LanguageParser;

#[derive(Debug)]
pub struct LanguageAST;

pub struct AST;

impl AST {
    pub fn parse_code(code: &str) -> Result<LanguageAST, Error<Rule>> {
        let ast = LanguageAST {};
        let res = LanguageParser::parse(Rule::identifier, code)?.next().unwrap();
        //dbg!(res);
        Ok(ast)
    }
}
