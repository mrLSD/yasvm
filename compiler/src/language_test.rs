use super::{LanguageParser, Rule};
use pest::Parser;

#[test]
fn test_characters() {
    // newline
    let rules = [
        (" ", false),
        ("Ю", false),
        ("⇨", false),
        ("_", false),
        ("1", false),
        ("\n", true),
    ];
    rules.iter().for_each(|(code, expect)| {
        let res = LanguageParser::parse(Rule::newline, *code);
        assert_eq!(res.is_ok(), *expect);
    });

    // unicode_char
    let rules = [
        (" ", false),
        ("Ю", true),
        ("⇨", true),
        ("_", true),
        ("1", true),
        ("\n", false),
    ];
    rules.iter().for_each(|(code, expect)| {
        let res = LanguageParser::parse(Rule::unicode_char, *code);
        assert_eq!(res.is_ok(), *expect);
    });

    // unicode_letter
    let rules = [
        (" ", false),
        ("ЮЮ", true),
        ("⇨", false),
        ("_", false),
        ("1", false),
        ("\n", false),
    ];
    rules.iter().for_each(|(code, expect)| {
        let res = LanguageParser::parse(Rule::unicode_letter, *code);
        assert_eq!(res.is_ok(), *expect);
    });

    // unicode_digit
    let rules = [
        (" ", false),
        ("Ю", false),
        ("⇨", false),
        ("_", false),
        ("1", true),
        ("9", true),
        ("\n", false),
    ];
    rules.iter().for_each(|(code, expect)| {
        let res = LanguageParser::parse(Rule::unicode_digit, *code);
        assert_eq!(res.is_ok(), *expect);
    });
}

#[test]
fn test_letters_and_digits() {
    // letter
    let rules = [
        (" ", false),
        ("Ю", true),
        ("⇨", false),
        ("_", true),
        ("1", false),
        ("\n", false),
    ];
    rules.iter().for_each(|(code, expect)| {
        let res = LanguageParser::parse(Rule::letter, *code);
        assert_eq!(res.is_ok(), *expect);
    });

    // decimal_digit
    let rules = [
        ("Ю", false),
        ("⇨", false),
        ("0", true),
        ("7", true),
        ("9", true),
        ("А", false),
        ("F", false),
        ("G", false),
    ];
    rules.iter().for_each(|(code, expect)| {
        let res = LanguageParser::parse(Rule::decimal_digit, *code);
        assert_eq!(res.is_ok(), *expect);
    });

    // octal_digit
    let rules = [
        ("Ю", false),
        ("⇨", false),
        ("0", true),
        ("7", true),
        ("8", false),
        ("F", false),
        ("G", false),
    ];
    rules.iter().for_each(|(code, expect)| {
        let res = LanguageParser::parse(Rule::octal_digit, *code);
        assert_eq!(res.is_ok(), *expect);
    });

    // hex_digit
    let rules = [
        ("Ю", false),
        ("⇨", false),
        ("0", true),
        ("9", true),
        ("F", true),
        ("G", false),
    ];
    rules.iter().for_each(|(code, expect)| {
        let res = LanguageParser::parse(Rule::hex_digit, *code);
        assert_eq!(res.is_ok(), *expect);
    });
}

#[test]
fn test_identifier() {
    // identifier
    let rules = [
        ("bool", false),
        ("int64", false),
        ("true", false),
        //("ints", true),
        ("the_test", true),
        ("the test", true),
        ("_test", true),
        ("test", true),
        ("test123_test", true),
    ];
    rules.iter().for_each(|(code, expect)| {
        let res = LanguageParser::parse(Rule::identifier, *code);
        if res.is_ok() {
            let res = res.clone().unwrap().next().unwrap();
            dbg!((&res.as_str(), &code));
        }
        assert_eq!(res.is_ok(), *expect);
    });

    let code = r#"
package main

func main() {
const (
    tst tst =;
)
    tst();
}
"#;
    let res = LanguageParser::parse(Rule::SourceFile, code);

    if res.is_ok() {
        dbg!(&res.clone().unwrap().tokens());
        let res = res.clone().unwrap().next().unwrap();
        dbg!((&res.as_str(), &code));
    } else {
        dbg!(&res);
    }

    assert_eq!(res.is_ok(), true);
}
