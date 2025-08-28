use assembler::tokenizer::{ Tokenizer, TokenError };

#[test]
fn test_error_position_reporting() {
    let source = r#"valid_token
"unterminated_string
another_token"#;

    let mut tokenizer = Tokenizer::new(source);

    let token1 = tokenizer.next_token().unwrap().unwrap();
    assert_eq!(token1.line, 1);
    assert_eq!(token1.column, 1);

    let error_result = tokenizer.next_token();
    assert!(error_result.is_err());

    match error_result.unwrap_err() {
        TokenError::UnterminatedString { line, column } => {
            assert_eq!(line, 2);
            assert_eq!(column, 1);
        }
        _ => panic!("Expected UnterminatedString error"),
    }
}

#[test]
fn test_unexpected_character_position() {
    let source = "valid @invalid";
    let mut tokenizer = Tokenizer::new(source);

    let token1 = tokenizer.next_token().unwrap().unwrap();
    assert_eq!(token1.line, 1);
    assert_eq!(token1.column, 1);

    let error_result = tokenizer.next_token();
    assert!(error_result.is_err());

    match error_result.unwrap_err() {
        TokenError::UnexpectedCharacter { character, line, column } => {
            assert_eq!(character, '@');
            assert_eq!(line, 1);
            assert_eq!(column, 7);
        }
        _ => panic!("Expected UnexpectedCharacter error"),
    }
}
