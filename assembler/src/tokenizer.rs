#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Dot,
    Id,
    Modulo,
    Colon,
    Minus,
    Comma,
    String,
    Number,
}

#[derive(Debug, PartialEq)]
pub enum TokenError {
    UnterminatedString {
        line: usize,
        column: usize,
    },
    UnexpectedCharacter {
        character: char,
        line: usize,
        column: usize,
    },
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenError::UnterminatedString { line, column } => {
                write!(f, "Unterminated string at line {}, column {}", line, column)
            }
            TokenError::UnexpectedCharacter { character, line, column } => {
                write!(
                    f,
                    "Unexpected character '{}' at line {}, column {}",
                    character,
                    line,
                    column
                )
            }
        }
    }
}

impl std::error::Error for TokenError {}

pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
    start: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(source: &str) -> Self {
        let input = source.chars().collect();
        Tokenizer {
            input,
            position: 0,
            start: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, TokenError> {
        self.skip_whitespace();

        if self.is_eof() {
            return Ok(None);
        }

        self.start = self.position;
        let start_line = self.line;
        let start_column = self.column;

        let c = self.advance();
        match c {
            '.' => Ok(self.new_token(TokenKind::Dot, None)),
            '%' => Ok(self.new_token(TokenKind::Modulo, None)),
            '"' => self.string(),
            ':' => Ok(self.new_token(TokenKind::Colon, None)),
            '-' => Ok(self.new_token(TokenKind::Minus, None)),
            ',' => Ok(self.new_token(TokenKind::Comma, None)),
            '#' => self.skip_comments(),
            c if c.is_alphabetic() || c == '_' => Ok(self.identifier()),
            c if c.is_ascii_digit() => self.number(),
            _ =>
                Err(TokenError::UnexpectedCharacter {
                    character: c,
                    line: start_line,
                    column: start_column,
                }),
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_eof() && self.peek().is_whitespace() {
            self.advance();
        }
    }

    fn skip_comments(&mut self) -> Result<Option<Token>, TokenError> {
        while !self.is_eof() && self.peek() != '\n' {
            self.advance();
        }
        self.next_token()
    }

    fn identifier(&mut self) -> Option<Token> {
        while !self.is_eof() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            self.advance();
        }
        let value: String = self.input[self.start..self.position].iter().collect();
        self.new_token(TokenKind::Id, Some(value))
    }

    fn string(&mut self) -> Result<Option<Token>, TokenError> {
        let start_line = self.line;
        let start_column = self.column - 1; // Account for the opening quote

        while !self.is_eof() && self.peek() != '"' {
            self.advance();
        }

        if self.is_eof() {
            return Err(TokenError::UnterminatedString {
                line: start_line,
                column: start_column,
            });
        }

        let value: String = self.input[self.start + 1..self.position].iter().collect();
        self.advance(); // consume closing quote
        Ok(self.new_token(TokenKind::String, Some(value)))
    }

    fn number(&mut self) -> Result<Option<Token>, TokenError> {
        while !self.is_eof() && self.peek().is_ascii_digit() {
            self.advance();
        }
        let value: String = self.input[self.start..self.position].iter().collect();
        Ok(self.new_token(TokenKind::Number, Some(value)))
    }

    fn new_token(&self, kind: TokenKind, value: Option<String>) -> Option<Token> {
        Some(Token::new(kind, value))
    }

    pub fn tokens(&mut self) -> Result<Vec<Token>, TokenError> {
        let mut tokens = Vec::new();
        while !self.is_eof() {
            match self.next_token()? {
                Some(token) => tokens.push(token),
                _ => {
                    break;
                }
            }
        }
        Ok(tokens)
    }

    fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }

    fn advance(&mut self) -> char {
        if self.is_eof() {
            return '\0';
        }
        let c = self.input[self.position];
        self.position += 1;

        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        c
    }

    fn peek(&self) -> char {
        if self.is_eof() {
            return '\0';
        }
        self.input[self.position]
    }
}

impl Token {
    pub fn new(kind: TokenKind, value: Option<String>) -> Self {
        Self { kind, value }
    }
}
