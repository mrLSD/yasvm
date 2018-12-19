#![feature(custom_attribute)]
use pest::error::Error;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/language.pest"]
struct LanguageParser;

pub struct LanguageAST;

pub fn parse_code(_code: &str) -> Result<LanguageAST, Error<Rule>> {
    let ast = LanguageAST {};
    Ok(ast)
}
