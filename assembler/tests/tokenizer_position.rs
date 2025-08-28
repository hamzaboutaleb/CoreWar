use assembler::tokenizer::{ Tokenizer, TokenKind };

#[test]
fn test_line_and_column_tracking() {
    let source = r#"hello
.
%"#;
    let mut tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.tokens().unwrap();

    assert_eq!(tokens.len(), 3);

    assert_eq!(tokens[0].kind, TokenKind::Id);
    assert_eq!(tokens[0].value, Some("hello".to_string()));
    assert_eq!(tokens[0].line, 1);
    assert_eq!(tokens[0].column, 1);

    assert_eq!(tokens[1].kind, TokenKind::Dot);
    assert_eq!(tokens[1].line, 2);
    assert_eq!(tokens[1].column, 1);

    assert_eq!(tokens[2].kind, TokenKind::Modulo);
    assert_eq!(tokens[2].line, 3);
    assert_eq!(tokens[2].column, 1);
}

#[test]
fn test_column_tracking_within_line() {
    let source = "hello . % 42";
    let mut tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.tokens().unwrap();

    assert_eq!(tokens.len(), 4);

    assert_eq!(tokens[0].kind, TokenKind::Id);
    assert_eq!(tokens[0].line, 1);
    assert_eq!(tokens[0].column, 1);

    assert_eq!(tokens[1].kind, TokenKind::Dot);
    assert_eq!(tokens[1].line, 1);
    assert_eq!(tokens[1].column, 7);

    assert_eq!(tokens[2].kind, TokenKind::Modulo);
    assert_eq!(tokens[2].line, 1);
    assert_eq!(tokens[2].column, 9);

    assert_eq!(tokens[3].kind, TokenKind::Number);
    assert_eq!(tokens[3].line, 1);
    assert_eq!(tokens[3].column, 11);
}

#[test]
fn test_string_token_position() {
    let source = r#"before "hello world" after"#;
    let mut tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.tokens().unwrap();

    assert_eq!(tokens.len(), 3);

    assert_eq!(tokens[0].kind, TokenKind::Id);
    assert_eq!(tokens[0].line, 1);
    assert_eq!(tokens[0].column, 1);

    assert_eq!(tokens[1].kind, TokenKind::String);
    assert_eq!(tokens[1].value, Some("hello world".to_string()));
    assert_eq!(tokens[1].line, 1);
    assert_eq!(tokens[1].column, 8);

    assert_eq!(tokens[2].kind, TokenKind::Id);
    assert_eq!(tokens[2].line, 1);
    assert_eq!(tokens[2].column, 22);
}
