use assembler::tokenizer::{ Tokenizer, TokenError, TokenKind };

#[test]
fn test_unterminated_string_error() {
    let mut tokenizer = Tokenizer::new(r#""hello world"#);
    let result = tokenizer.next_token();

    assert!(result.is_err());
    match result.unwrap_err() {
        TokenError::UnterminatedString { line, column } => {
            assert_eq!(line, 1);
            assert_eq!(column, 1);
        }
        _ => panic!("Expected UnterminatedString error"),
    }
}

#[test]
fn test_unexpected_character_error() {
    let mut tokenizer = Tokenizer::new("@invalid");
    let result = tokenizer.next_token();

    assert!(result.is_err());
    match result.unwrap_err() {
        TokenError::UnexpectedCharacter { character, line, column } => {
            assert_eq!(character, '@');
            assert_eq!(line, 1);
            assert_eq!(column, 1);
        }
        _ => panic!("Expected UnexpectedCharacter error"),
    }
}

#[test]
fn test_successful_tokenization() {
    let mut tokenizer = Tokenizer::new(r#". % "hello" identifier"#);
    let tokens = tokenizer.tokens().unwrap();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].kind, TokenKind::Dot);
    assert_eq!(tokens[1].kind, TokenKind::Modulo);
    assert_eq!(tokens[2].kind, TokenKind::String);
    assert_eq!(tokens[2].value, Some("hello".to_string()));
    assert_eq!(tokens[3].kind, TokenKind::Id);
    assert_eq!(tokens[3].value, Some("identifier".to_string()));
}

#[test]
fn test_error_propagation_in_tokens() {
    let mut tokenizer = Tokenizer::new(r#". "unterminated"#);
    let result = tokenizer.tokens();

    assert!(result.is_err());
    match result.unwrap_err() {
        TokenError::UnterminatedString { line, column } => {
            assert_eq!(line, 1);
            assert_eq!(column, 3);
        }
        _ => panic!("Expected UnterminatedString error"),
    }
}
