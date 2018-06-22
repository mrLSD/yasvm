use crate::AST;

#[test]
fn parse_code() {
    let code = r#"test123"#;
    let res = AST::parse_code(code);
    assert!(res.is_ok());

    let code = r#"1test123"#;
    let res = AST::parse_code(code);
    assert!(res.is_err());
}
