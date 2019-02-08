#![feature(custom_attribute)]
#![allow(unused_imports)]

mod language_test;

use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser, Debug)]
#[grammar = "grammar/simple.pest"]
pub struct LanguageParser;

#[derive(Debug)]
pub struct LanguageAST;

pub struct AST;

/*
impl AST {
    pub fn parse_code(code: &str) -> Result<LanguageAST, Error<Rule>> {
        let ast = LanguageAST {};
        let res = LanguageParser::parse(Rule::identifier, code)?
            .next()
            .unwrap();
        //dbg!(res);
        Ok(ast)
    }
}
*/
