// ArcScript lexer (MVP subset)

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Identifiers & literals
    Identifier,
    Int,
    Float,
    String,

    // Keywords
    KwVar,
    KwFunc,
    KwObject,
    KwOn,
    KwIf,
    KwElif,
    KwElse,
    KwWhile,
    KwFor,
    KwDo,
    KwThen,
    KwEnd,
    KwReturn,
    KwBreak,
    KwContinue,
    KwTrue,
    KwFalse,
    KwNil,
    KwAnd,
    KwOr,
    KwNot,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Colon,
    Semicolon,

    // Special
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer<'a> {
    source: &'a [u8],
    pub line: usize,
    pub column: usize,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            line: 1,
            column: 1,
            pos: 0,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.source.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let ch = self.source.get(self.pos).copied();
        if let Some(c) = ch {
            self.pos += 1;
            if c == b'\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        ch
    }

    fn match_next(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        let start_line = self.line;
        let start_column = self.column;

        let ch = match self.advance() {
            Some(c) => c,
            None => {
                return Token {
                    kind: TokenKind::Eof,
                    lexeme: String::new(),
                    line: start_line,
                    column: start_column,
                }
            }
        };

        match ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.lex_identifier_or_keyword(ch, start_line, start_column),
            b'0'..=b'9' => self.lex_number(ch, start_line, start_column),
            b'"' => self.lex_string(start_line, start_column),
            b'(' => self.simple_token(TokenKind::LParen, "(", start_line, start_column),
            b')' => self.simple_token(TokenKind::RParen, ")", start_line, start_column),
            b'{' => self.simple_token(TokenKind::LBrace, "{", start_line, start_column),
            b'}' => self.simple_token(TokenKind::RBrace, "}", start_line, start_column),
            b'[' => self.simple_token(TokenKind::LBracket, "[", start_line, start_column),
            b']' => self.simple_token(TokenKind::RBracket, "]", start_line, start_column),
            b',' => self.simple_token(TokenKind::Comma, ",", start_line, start_column),
            b'.' => self.simple_token(TokenKind::Dot, ".", start_line, start_column),
            b':' => self.simple_token(TokenKind::Colon, ":", start_line, start_column),
            b';' => self.simple_token(TokenKind::Semicolon, ";", start_line, start_column),
            b'+' => {
                if self.match_next(b'=') {
                    self.simple_token(TokenKind::PlusEqual, "+=", start_line, start_column)
                } else {
                    self.simple_token(TokenKind::Plus, "+", start_line, start_column)
                }
            }
            b'-' => {
                if self.match_next(b'=') {
                    self.simple_token(TokenKind::MinusEqual, "-=", start_line, start_column)
                } else {
                    self.simple_token(TokenKind::Minus, "-", start_line, start_column)
                }
            }
            b'*' => {
                if self.match_next(b'=') {
                    self.simple_token(TokenKind::StarEqual, "*=", start_line, start_column)
                } else {
                    self.simple_token(TokenKind::Star, "*", start_line, start_column)
                }
            }
            b'/' => {
                if self.match_next(b'=') {
                    self.simple_token(TokenKind::SlashEqual, "/=", start_line, start_column)
                } else {
                    self.simple_token(TokenKind::Slash, "/", start_line, start_column)
                }
            }
            b'%' => self.simple_token(TokenKind::Percent, "%", start_line, start_column),
            b'=' => {
                if self.match_next(b'=') {
                    self.simple_token(TokenKind::EqualEqual, "==", start_line, start_column)
                } else {
                    self.simple_token(TokenKind::Equal, "=", start_line, start_column)
                }
            }
            b'!' => {
                if self.match_next(b'=') {
                    self.simple_token(TokenKind::BangEqual, "!=", start_line, start_column)
                } else {
                    self.simple_token(TokenKind::KwNot, "!", start_line, start_column)
                }
            }
            b'<' => {
                if self.match_next(b'=') {
                    self.simple_token(TokenKind::LessEqual, "<=", start_line, start_column)
                } else {
                    self.simple_token(TokenKind::Less, "<", start_line, start_column)
                }
            }
            b'>' => {
                if self.match_next(b'=') {
                    self.simple_token(TokenKind::GreaterEqual, ">=", start_line, start_column)
                } else {
                    self.simple_token(TokenKind::Greater, ">", start_line, start_column)
                }
            }
            _ => Token {
                kind: TokenKind::Eof,
                lexeme: String::new(),
                line: start_line,
                column: start_column,
            },
        }
    }

    fn simple_token(&self, kind: TokenKind, lexeme: &str, line: usize, column: usize) -> Token {
        Token {
            kind,
            lexeme: lexeme.to_string(),
            line,
            column,
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            let c = self.peek();
            match c {
                Some(b' ' | b'\r' | b'\t' | b'\n') => {
                    self.advance();
                }
                Some(b'/') => {
                    if self.source.get(self.pos + 1) == Some(&b'/') {
                        // line comment
                        while let Some(ch) = self.peek() {
                            if ch == b'\n' {
                                break;
                            }
                            self.advance();
                        }
                    } else if self.source.get(self.pos + 1) == Some(&b'*') {
                        // block comment
                        self.advance(); // consume '/'
                        self.advance(); // consume '*'
                        while let Some(ch) = self.peek() {
                            if ch == b'*' && self.source.get(self.pos + 1) == Some(&b'/') {
                                self.advance(); // consume '*'
                                self.advance(); // consume '/'
                                break;
                            }
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn lex_identifier_or_keyword(&mut self, first: u8, line: usize, column: usize) -> Token {
        let mut buf = Vec::new();
        buf.push(first);
        while let Some(c) = self.peek() {
            if (c as char).is_ascii_alphanumeric() || c == b'_' {
                self.advance();
                buf.push(c);
            } else {
                break;
            }
        }
        let s = String::from_utf8(buf).unwrap_or_default();
        let kind = match s.as_str() {
            "var" => TokenKind::KwVar,
            "func" => TokenKind::KwFunc,
            "object" => TokenKind::KwObject,
            "on" => TokenKind::KwOn,
            "if" => TokenKind::KwIf,
            "elif" => TokenKind::KwElif,
            "else" => TokenKind::KwElse,
            "while" => TokenKind::KwWhile,
            "for" => TokenKind::KwFor,
            "do" => TokenKind::KwDo,
            "then" => TokenKind::KwThen,
            "end" => TokenKind::KwEnd,
            "return" => TokenKind::KwReturn,
            "break" => TokenKind::KwBreak,
            "continue" => TokenKind::KwContinue,
            "true" => TokenKind::KwTrue,
            "false" => TokenKind::KwFalse,
            "nil" => TokenKind::KwNil,
            "and" => TokenKind::KwAnd,
            "or" => TokenKind::KwOr,
            "not" => TokenKind::KwNot,
            _ => TokenKind::Identifier,
        };
        Token { kind, lexeme: s, line, column }
    }

    fn lex_number(&mut self, first: u8, line: usize, column: usize) -> Token {
        let mut buf = Vec::new();
        buf.push(first);
        let mut is_float = false;

        while let Some(c) = self.peek() {
            if (c as char).is_ascii_digit() {
                self.advance();
                buf.push(c);
            } else if c == b'.' && !is_float {
                is_float = true;
                self.advance();
                buf.push(c);
            } else {
                break;
            }
        }

        let s = String::from_utf8(buf).unwrap_or_default();
        let kind = if is_float { TokenKind::Float } else { TokenKind::Int };
        Token { kind, lexeme: s, line, column }
    }

    fn lex_string(&mut self, line: usize, column: usize) -> Token {
        let mut buf = Vec::new();
        while let Some(c) = self.peek() {
            self.advance();
            if c == b'"' {
                break;
            }
            if c == b'\\' {
                if let Some(esc) = self.advance() {
                    match esc {
                        b'n' => buf.push(b'\n'),
                        b't' => buf.push(b'\t'),
                        b'r' => buf.push(b'\r'),
                        b'\\' => buf.push(b'\\'),
                        b'"' => buf.push(b'"'),
                        other => buf.push(other),
                    }
                } else {
                    break;
                }
            } else {
                buf.push(c);
            }
        }
        let s = String::from_utf8(buf).unwrap_or_default();
        Token { kind: TokenKind::String, lexeme: s, line, column }
    }
}
