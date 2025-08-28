use assembler::tokenizer::Token;
use assembler::tokenizer::Tokenizer;
use assembler::tokenizer::TokenKind;

#[test]
fn test_empty_source() {
    let source = "";
    let mut tokenizer = Tokenizer::new(source);
    let next_token = tokenizer.next_token().unwrap();
    println!("{:?}", next_token);
    assert_eq!(next_token, None);
}

#[test]
fn test_dots() {
    let source = ".";
    let mut tokenizer = Tokenizer::new(source);
    let next_token = tokenizer.next_token().unwrap();
    let expected_token = Token::new(TokenKind::Dot, None);
    assert_eq!(next_token, Some(expected_token));
}

#[test]
fn test_modulo() {
    let source = "%";
    let mut tokenizer = Tokenizer::new(source);
    let next_token = tokenizer.next_token().unwrap();
    let expected_token = Token::new(TokenKind::Modulo, None);
    assert_eq!(next_token, Some(expected_token))
}

#[test]
fn test_comma() {
    let source = ",";
    let mut tokenizer = Tokenizer::new(source);
    let next_token = tokenizer.next_token().unwrap();
    let expected_token = Token::new(TokenKind::Comma, None);
    assert_eq!(next_token, Some(expected_token))
}

#[test]
fn test_string() {
    let source = "\"hello\"";
    let mut tokenizer = Tokenizer::new(source);
    let next_token = tokenizer.next_token().unwrap();
    let expected_token = Token::new(TokenKind::String, Some("hello".to_string()));
    assert_eq!(next_token, Some(expected_token))
}

#[test]
fn test_string_without_closing_qoute() {
    let source = "\"hello";
    let mut tokenizer = Tokenizer::new(source);
    let next_token = tokenizer.next_token();
    assert!(next_token.is_err());
}

#[test]
fn test_colon() {
    let source = ":";
    let mut tokenizer = Tokenizer::new(source);
    let next_token = tokenizer.next_token().unwrap();
    let expected_token = Token::new(TokenKind::Colon, None);
    assert_eq!(next_token, Some(expected_token))
}

#[test]
fn test_minus() {
    let source = "-";
    let mut tokenizer = Tokenizer::new(source);
    let next_token = tokenizer.next_token().unwrap();
    let expected_token = Token::new(TokenKind::Minus, None);
    assert_eq!(next_token, Some(expected_token))
}

#[test]
fn test_mixed_tokens() {
    let source = r#". % "hello" identifier 42"#;
    let mut tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.tokens().unwrap();

    assert_eq!(tokens.len(), 5);

    // Dot token
    assert_eq!(tokens[0].kind, TokenKind::Dot);
    assert_eq!(tokens[0].value, None);

    // Modulo token
    assert_eq!(tokens[1].kind, TokenKind::Modulo);
    assert_eq!(tokens[1].value, None);

    // String token
    assert_eq!(tokens[2].kind, TokenKind::String);
    assert_eq!(tokens[2].value, Some("hello".to_string()));

    // Identifier token
    assert_eq!(tokens[3].kind, TokenKind::Id);
    assert_eq!(tokens[3].value, Some("identifier".to_string()));

    // Integer number token
    assert_eq!(tokens[4].kind, TokenKind::Number);
    assert_eq!(tokens[4].value, Some("42".to_string()));
}

#[test]
fn test_integer_number() {
    let mut tokenizer = Tokenizer::new("42");
    let token = tokenizer.next_token().unwrap().unwrap();
    assert_eq!(token.kind, TokenKind::Number);
    assert_eq!(token.value, Some("42".to_string()));
}

#[test]
fn test_zero() {
    let mut tokenizer = Tokenizer::new("0");
    let token = tokenizer.next_token().unwrap().unwrap();
    assert_eq!(token.kind, TokenKind::Number);
    assert_eq!(token.value, Some("0".to_string()));
}

#[test]
fn test_multi_digit_number() {
    let mut tokenizer = Tokenizer::new("12345");
    let token = tokenizer.next_token().unwrap().unwrap();
    assert_eq!(token.kind, TokenKind::Number);
    assert_eq!(token.value, Some("12345".to_string()));
}

#[test]
fn test_number_followed_by_identifier() {
    let mut tokenizer = Tokenizer::new("42 hello");

    let token1 = tokenizer.next_token().unwrap().unwrap();
    assert_eq!(token1.kind, TokenKind::Number);
    assert_eq!(token1.value, Some("42".to_string()));

    let token2 = tokenizer.next_token().unwrap().unwrap();
    assert_eq!(token2.kind, TokenKind::Id);
    assert_eq!(token2.value, Some("hello".to_string()));
}

#[test]
fn test_multiple_numbers() {
    let mut tokenizer = Tokenizer::new("1 999");
    let tokens = tokenizer.tokens().unwrap();

    assert_eq!(tokens.len(), 2);

    assert_eq!(tokens[0].kind, TokenKind::Number);
    assert_eq!(tokens[0].value, Some("1".to_string()));

    assert_eq!(tokens[1].kind, TokenKind::Number);
    assert_eq!(tokens[1].value, Some("999".to_string()));
}
