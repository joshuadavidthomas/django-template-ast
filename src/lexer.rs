use crate::error::LexerError;
use crate::token::{Token, TokenStream, TokenType};

pub struct Lexer {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: String::from(source),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<TokenStream, LexerError> {
        let mut tokens = TokenStream::new();
        while !self.is_at_end() {
            let token = self.next_token()?;
            tokens.add_token(token);
        }
        tokens.finalize(self.line);
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, LexerError> {
        let c = self.peek()?;
        let token_type = match c {
            ',' | '.' | '+' | ':' | '|' | '\'' | '"' => self.single_char(c)?,
            '{' => self.left_brace()?,
            '}' => self.right_brace()?,
            '%' => self.percent()?,
            '#' => self.hash()?,
            '!' => self.bang()?,
            '=' => self.equal()?,
            '<' => self.left_angle()?,
            '>' => self.right_angle()?,
            '/' => self.slash()?,
            '-' => self.dash()?,
            '*' => self.star()?,
            c if c.is_whitespace() => self.whitespace()?,
            _ => self.text()?,
        };
        let lexeme = self.extract_lexeme(token_type)?;
        let token = Token::new(token_type, lexeme, self.line);
        self.advance(token.size(), token.lines())?;
        Ok(token)
    }

    fn single_char(&self, c: char) -> Result<TokenType, LexerError> {
        let token_type = match c {
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '+' => TokenType::Plus,
            ':' => TokenType::Colon,
            '|' => TokenType::Pipe,
            '\'' => TokenType::SingleQuote,
            '"' => TokenType::DoubleQuote,
            '/' => TokenType::Slash,
            '%' => TokenType::Percent,
            _ => {
                return Err(LexerError::UnexpectedCharacter {
                    character: c,
                    line: self.line,
                })
            }
        };
        Ok(token_type)
    }

    fn left_brace(&self) -> Result<TokenType, LexerError> {
        let token_type = match self.peek_next()? {
            '{' => TokenType::DoubleLeftBrace,
            '%' => TokenType::LeftBracePercent,
            '#' => TokenType::LeftBraceHash,
            _ => return self.text(),
        };
        Ok(token_type)
    }

    fn right_brace(&self) -> Result<TokenType, LexerError> {
        let token_type = match self.peek_next()? {
            '}' => TokenType::DoubleRightBrace,
            _ => return self.text(),
        };
        Ok(token_type)
    }

    fn percent(&self) -> Result<TokenType, LexerError> {
        let token_type = match self.peek_next()? {
            '}' => TokenType::PercentRightBrace,
            _ => TokenType::Percent,
        };
        Ok(token_type)
    }

    fn hash(&self) -> Result<TokenType, LexerError> {
        let token_type = match self.peek_next()? {
            '}' => TokenType::HashRightBrace,
            _ => return self.text(),
        };
        Ok(token_type)
    }

    fn bang(&self) -> Result<TokenType, LexerError> {
        let token_type = match self.peek_next()? {
            '=' => TokenType::BangEqual,
            _ => TokenType::Bang,
        };
        Ok(token_type)
    }

    fn equal(&self) -> Result<TokenType, LexerError> {
        let token_type = match self.peek_next()? {
            '=' => TokenType::DoubleEqual,
            _ => TokenType::Equal,
        };
        Ok(token_type)
    }

    fn left_angle(&self) -> Result<TokenType, LexerError> {
        let token_type = match (self.peek_next()?, self.peek_at(2)?, self.peek_at(3)?) {
            ('=', _, _) => TokenType::LeftAngleEqual,
            ('!', '-', '-') => TokenType::LeftAngleBangDashDash,
            (c, _, _) if c.is_whitespace() || c.is_alphabetic() || c == '\0' => {
                TokenType::LeftAngle
            }
            _ => return self.text(),
        };
        Ok(token_type)
    }

    fn right_angle(&self) -> Result<TokenType, LexerError> {
        let token_type = match self.peek_next()? {
            '=' => TokenType::RightAngleEqual,
            _ => TokenType::RightAngle,
        };
        Ok(token_type)
    }

    fn slash(&self) -> Result<TokenType, LexerError> {
        let token_type = match self.peek_next()? {
            '>' => TokenType::SlashRightAngle,
            '/' => TokenType::DoubleSlash,
            '*' => TokenType::SlashStar,
            _ => TokenType::Slash,
        };
        Ok(token_type)
    }

    fn dash(&self) -> Result<TokenType, LexerError> {
        let token_type = match (self.peek_next()?, self.peek_at(2)?) {
            ('-', '>') => TokenType::DashDashRightAngle,
            ('-', _) => return self.text(),
            _ => TokenType::Dash,
        };
        Ok(token_type)
    }

    fn star(&self) -> Result<TokenType, LexerError> {
        let token_type = match self.peek_next()? {
            '/' => TokenType::StarSlash,
            _ => return self.text(),
        };
        Ok(token_type)
    }

    fn whitespace(&mut self) -> Result<TokenType, LexerError> {
        let token_type = TokenType::Whitespace;
        Ok(token_type)
    }

    fn text(&self) -> Result<TokenType, LexerError> {
        let token_type = TokenType::Text;
        Ok(token_type)
    }

    fn extract_lexeme(&self, token_type: TokenType) -> Result<&str, LexerError> {
        let remaining_source = &self.source[self.current..];

        let size = match token_type.size() {
            Ok(size) => size,
            _ => match token_type {
                TokenType::Whitespace => remaining_source
                    .chars()
                    .take_while(|&c| c.is_whitespace() && c != '\0')
                    .map(|c| c.len_utf8())
                    .sum(),
                TokenType::Text => remaining_source
                    .chars()
                    .take_while(|&c| !c.is_whitespace() && c != '\0')
                    .map(|c| c.len_utf8())
                    .sum(),
                _ => return Err(LexerError::UnexpectedTokenType(token_type)),
            },
        };

        let end = size.min(remaining_source.len());
        let result = &remaining_source[..end];
        Ok(result)
    }

    fn advance(&mut self, chars: usize, lines: usize) -> Result<(), LexerError> {
        if self.is_at_end() {
            return Err(LexerError::AtEndOfSource);
        }
        self.start = self.current;
        self.current += chars;
        self.line += lines;
        Ok(())
    }

    fn peek(&self) -> Result<char, LexerError> {
        self.peek_at(0)
    }

    fn peek_next(&self) -> Result<char, LexerError> {
        self.peek_at(1)
    }

    fn peek_previous(&self) -> Result<char, LexerError> {
        self.peek_at(-1)
    }

    fn peek_at(&self, offset: isize) -> Result<char, LexerError> {
        let index = self.current as isize + offset;
        self.item_at(index as usize)
    }

    fn item_at(&self, index: usize) -> Result<char, LexerError> {
        println!("index: {}", index);
        if index >= self.source.len() {
            // Return a null character when past the end, a bit of a departure from
            // idiomatic Rust code, but makes writing the matching above and testing
            // much easier
            Ok('\0')
        } else {
            Ok(self.source.chars().nth(index).unwrap())
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_from_source() {
        let line = 1;

        let test_cases = vec![
            (
                "<",
                Token {
                    token_type: TokenType::LeftAngle,
                    lexeme: "<".to_string(),
                    line,
                },
            ),
            (
                ">",
                Token {
                    token_type: TokenType::RightAngle,
                    lexeme: ">".to_string(),
                    line,
                },
            ),
            (
                ",",
                Token {
                    token_type: TokenType::Comma,
                    lexeme: ",".to_string(),
                    line,
                },
            ),
            (
                ".",
                Token {
                    token_type: TokenType::Dot,
                    lexeme: ".".to_string(),
                    line,
                },
            ),
            (
                "-",
                Token {
                    token_type: TokenType::Dash,
                    lexeme: "-".to_string(),
                    line,
                },
            ),
            (
                "+",
                Token {
                    token_type: TokenType::Plus,
                    lexeme: "+".to_string(),
                    line,
                },
            ),
            (
                ":",
                Token {
                    token_type: TokenType::Colon,
                    lexeme: ":".to_string(),
                    line,
                },
            ),
            (
                "/",
                Token {
                    token_type: TokenType::Slash,
                    lexeme: "/".to_string(),
                    line,
                },
            ),
            (
                "!",
                Token {
                    token_type: TokenType::Bang,
                    lexeme: "!".to_string(),
                    line,
                },
            ),
            (
                "=",
                Token {
                    token_type: TokenType::Equal,
                    lexeme: "=".to_string(),
                    line,
                },
            ),
            (
                "|",
                Token {
                    token_type: TokenType::Pipe,
                    lexeme: "|".to_string(),
                    line,
                },
            ),
            (
                "%",
                Token {
                    token_type: TokenType::Percent,
                    lexeme: "%".to_string(),
                    line,
                },
            ),
            (
                "'",
                Token {
                    token_type: TokenType::SingleQuote,
                    lexeme: "'".to_string(),
                    line,
                },
            ),
            (
                "\"",
                Token {
                    token_type: TokenType::DoubleQuote,
                    lexeme: "\"".to_string(),
                    line,
                },
            ),
            (
                "{{",
                Token {
                    token_type: TokenType::DoubleLeftBrace,
                    lexeme: "{{".to_string(),
                    line,
                },
            ),
            (
                "}}",
                Token {
                    token_type: TokenType::DoubleRightBrace,
                    lexeme: "}}".to_string(),
                    line,
                },
            ),
            (
                "{%",
                Token {
                    token_type: TokenType::LeftBracePercent,
                    lexeme: "{%".to_string(),
                    line,
                },
            ),
            (
                "%}",
                Token {
                    token_type: TokenType::PercentRightBrace,
                    lexeme: "%}".to_string(),
                    line,
                },
            ),
            (
                "{#",
                Token {
                    token_type: TokenType::LeftBraceHash,
                    lexeme: "{#".to_string(),
                    line,
                },
            ),
            (
                "#}",
                Token {
                    token_type: TokenType::HashRightBrace,
                    lexeme: "#}".to_string(),
                    line,
                },
            ),
            (
                "!=",
                Token {
                    token_type: TokenType::BangEqual,
                    lexeme: "!=".to_string(),
                    line,
                },
            ),
            (
                "==",
                Token {
                    token_type: TokenType::DoubleEqual,
                    lexeme: "==".to_string(),
                    line,
                },
            ),
            (
                "<=",
                Token {
                    token_type: TokenType::LeftAngleEqual,
                    lexeme: "<=".to_string(),
                    line,
                },
            ),
            (
                ">=",
                Token {
                    token_type: TokenType::RightAngleEqual,
                    lexeme: ">=".to_string(),
                    line,
                },
            ),
            (
                "<!--",
                Token {
                    token_type: TokenType::LeftAngleBangDashDash,
                    lexeme: "<!--".to_string(),
                    line,
                },
            ),
            (
                "-->",
                Token {
                    token_type: TokenType::DashDashRightAngle,
                    lexeme: "-->".to_string(),
                    line,
                },
            ),
            (
                "/>",
                Token {
                    token_type: TokenType::SlashRightAngle,
                    lexeme: "/>".to_string(),
                    line,
                },
            ),
            (
                "//",
                Token {
                    token_type: TokenType::DoubleSlash,
                    lexeme: "//".to_string(),
                    line,
                },
            ),
            (
                "/*",
                Token {
                    token_type: TokenType::SlashStar,
                    lexeme: "/*".to_string(),
                    line,
                },
            ),
            (
                "*/",
                Token {
                    token_type: TokenType::StarSlash,
                    lexeme: "*/".to_string(),
                    line,
                },
            ),
            (
                " ",
                Token {
                    token_type: TokenType::Whitespace,
                    lexeme: " ".to_string(),
                    line,
                },
            ),
            (
                "\r",
                Token {
                    token_type: TokenType::Whitespace,
                    lexeme: "\r".to_string(),
                    line,
                },
            ),
            (
                "\t",
                Token {
                    token_type: TokenType::Whitespace,
                    lexeme: "\t".to_string(),
                    line,
                },
            ),
            (
                "\n",
                Token {
                    token_type: TokenType::Whitespace,
                    lexeme: "\n".to_string(),
                    line,
                },
            ),
            (
                "  ",
                Token {
                    token_type: TokenType::Whitespace,
                    lexeme: "  ".to_string(),
                    line,
                },
            ),
            (
                " \n",
                Token {
                    token_type: TokenType::Whitespace,
                    lexeme: " \n".to_string(),
                    line,
                },
            ),
            (
                "a",
                Token {
                    token_type: TokenType::Text,
                    lexeme: "a".to_string(),
                    line,
                },
            ),
            (
                "Hello",
                Token {
                    token_type: TokenType::Text,
                    lexeme: "Hello".to_string(),
                    line,
                },
            ),
        ];

        for (input, expected_token) in test_cases {
            println!("Testing input: {:?}", input);

            let mut lexer = Lexer::new(input);

            match lexer.next_token() {
                Ok(token) => {
                    assert_eq!(token, expected_token, "Input: {}", input);
                }
                Err(e) => panic!(
                    "Expected {:?}, but got Err({:?}) for input: {}",
                    expected_token, e, input,
                ),
            }
            println!("---");
        }
    }

    fn assert_token_type(
        test_cases: Vec<(&str, TokenType)>,
        method: fn(&mut Lexer) -> Result<TokenType, LexerError>,
    ) {
        for (input, expected_token_type) in test_cases {
            println!("Testing input: {:?}", input);

            let mut lexer = Lexer::new(input);

            match method(&mut lexer) {
                Ok(token_type) => {
                    println!("Returned token type: {:?}", token_type);
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    panic!(
                        "Expected {:?}, but got Err({:?}) for input: {}",
                        expected_token_type, e, input,
                    );
                }
            }
            println!("---");
        }
    }

    #[test]
    fn test_left_brace() {
        let test_cases = vec![
            ("{{", TokenType::DoubleLeftBrace),
            ("{%", TokenType::LeftBracePercent),
            ("{#", TokenType::LeftBraceHash),
            ("{", TokenType::Text),
        ];

        assert_token_type(test_cases, |lexer| lexer.left_brace());
    }

    #[test]
    fn test_right_brace() {
        let test_cases = vec![("}}", TokenType::DoubleRightBrace), ("}", TokenType::Text)];

        assert_token_type(test_cases, |lexer| lexer.right_brace());
    }

    #[test]
    fn test_percent() {
        let test_cases = vec![
            ("%", TokenType::Percent),
            ("%}", TokenType::PercentRightBrace),
        ];

        assert_token_type(test_cases, |lexer| lexer.percent());
    }

    #[test]
    fn test_bang() {
        let test_cases = vec![("!", TokenType::Bang), ("!=", TokenType::BangEqual)];

        assert_token_type(test_cases, |lexer| lexer.bang());
    }

    #[test]
    fn test_equal() {
        let test_cases = vec![("=", TokenType::Equal), ("==", TokenType::DoubleEqual)];

        assert_token_type(test_cases, |lexer| lexer.equal());
    }

    #[test]
    fn test_left_angle() {
        let test_cases = vec![
            ("<", TokenType::LeftAngle),
            ("<=", TokenType::LeftAngleEqual),
            ("<!--", TokenType::LeftAngleBangDashDash),
            ("<!", TokenType::Text),
            ("<!-", TokenType::Text),
            ("<!---", TokenType::LeftAngleBangDashDash),
        ];

        assert_token_type(test_cases, |lexer| lexer.left_angle());
    }

    #[test]
    fn test_right_angle() {
        let test_cases = vec![
            (">", TokenType::RightAngle),
            (">=", TokenType::RightAngleEqual),
        ];

        assert_token_type(test_cases, |lexer| lexer.right_angle());
    }

    #[test]
    fn test_slash() {
        let test_cases = vec![
            ("/", TokenType::Slash),
            ("/>", TokenType::SlashRightAngle),
            ("//", TokenType::DoubleSlash),
            ("/*", TokenType::SlashStar),
        ];

        assert_token_type(test_cases, |lexer| lexer.slash());
    }

    #[test]
    fn test_dash() {
        let test_cases = vec![
            ("-", TokenType::Dash),
            ("-->", TokenType::DashDashRightAngle),
            ("--", TokenType::Text),
        ];

        assert_token_type(test_cases, |lexer| lexer.dash());
    }

    #[test]
    fn test_star() {
        let test_cases = vec![("*/", TokenType::StarSlash), ("*", TokenType::Text)];

        assert_token_type(test_cases, |lexer| lexer.star());
    }

    #[test]
    fn test_whitespace() {
        let test_cases = vec![
            (" ", TokenType::Whitespace),
            ("  ", TokenType::Whitespace),
            ("\r", TokenType::Whitespace),
            ("\t", TokenType::Whitespace),
            ("\n", TokenType::Whitespace),
            (" \n", TokenType::Whitespace),
        ];

        assert_token_type(test_cases, |lexer| lexer.whitespace());
    }

    #[test]
    fn test_text() {
        let test_cases = vec![
            ("a", TokenType::Text),
            ("1", TokenType::Text),
            ("Hello", TokenType::Text),
        ];

        assert_token_type(test_cases, |lexer| lexer.text());
    }

    #[test]
    fn test_extract_lexeme() {
        let test_cases = vec![
            (",", TokenType::Comma, ","),
            (".", TokenType::Dot, "."),
            ("+", TokenType::Plus, "+"),
            (":", TokenType::Colon, ":"),
            ("|", TokenType::Pipe, "|"),
            ("'", TokenType::SingleQuote, "'"),
            ("\"", TokenType::DoubleQuote, "\""),
            ("{{", TokenType::DoubleLeftBrace, "{{"),
            ("}}", TokenType::DoubleRightBrace, "}}"),
            ("{%", TokenType::LeftBracePercent, "{%"),
            ("%}", TokenType::PercentRightBrace, "%}"),
            ("{#", TokenType::LeftBraceHash, "{#"),
            ("#}", TokenType::HashRightBrace, "#}"),
            ("!=", TokenType::BangEqual, "!="),
            ("==", TokenType::DoubleEqual, "=="),
            ("<=", TokenType::LeftAngleEqual, "<="),
            (">=", TokenType::RightAngleEqual, ">="),
            ("/>", TokenType::SlashRightAngle, "/>"),
            ("//", TokenType::DoubleSlash, "//"),
            ("/*", TokenType::SlashStar, "/*"),
            ("*/", TokenType::StarSlash, "*/"),
            ("-->", TokenType::DashDashRightAngle, "-->"),
            ("<!--", TokenType::LeftAngleBangDashDash, "<!--"),
            (" ", TokenType::Whitespace, " "),
            ("  ", TokenType::Whitespace, "  "),
            ("\t", TokenType::Whitespace, "\t"),
            ("\n", TokenType::Whitespace, "\n"),
            (" \t\n", TokenType::Whitespace, " \t\n"),
            ("hello", TokenType::Text, "hello"),
            ("world!", TokenType::Text, "world!"),
            ("abc123", TokenType::Text, "abc123"),
            ("mixed_case", TokenType::Text, "mixed_case"),
            ("with spaces", TokenType::Text, "with"),
            ("", TokenType::Eof, ""),
        ];

        for (input, token_type, expected_lexeme) in test_cases {
            println!("Testing input: {:?}", input);

            let lexer = Lexer::new(input);

            match lexer.extract_lexeme(token_type) {
                Ok(lexeme) => {
                    println!("Extracted lexeme: {:?}", lexeme);
                    assert_eq!(lexeme, expected_lexeme, "For input: {:?}", input);
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    panic!(
                        "Expected lexeme {:?}, but got Err({:?}) for input: {}",
                        expected_lexeme, e, input,
                    );
                }
            }
            println!("---");
        }
    }

    #[test]
    fn test_advance() {
        let input = "Hello\nWorld\nTest";
        let mut lexer = Lexer::new(input);

        // initial
        assert_eq!(lexer.current, 0);
        assert_eq!(lexer.start, 0);
        assert_eq!(lexer.line, 1);

        // "Hello"
        lexer.advance(5, 0).unwrap();
        assert_eq!(lexer.current, 5);
        assert_eq!(lexer.start, 0);
        assert_eq!(lexer.line, 1);

        // newline
        lexer.advance(1, 1).unwrap();
        assert_eq!(lexer.current, 6);
        assert_eq!(lexer.start, 5);
        assert_eq!(lexer.line, 2);

        // "World"
        lexer.advance(5, 0).unwrap();
        assert_eq!(lexer.current, 11);
        assert_eq!(lexer.start, 6);
        assert_eq!(lexer.line, 2);

        // newline
        lexer.advance(1, 1).unwrap();
        assert_eq!(lexer.current, 12);
        assert_eq!(lexer.start, 11);
        assert_eq!(lexer.line, 3);

        // "Test"
        lexer.advance(4, 0).unwrap();
        assert_eq!(lexer.current, 16);
        assert_eq!(lexer.start, 12);
        assert_eq!(lexer.line, 3);

        // past the end of the input
        let result = lexer.advance(1, 0);
        assert!(result.is_err());
        assert_eq!(lexer.current, 16);
        assert_eq!(lexer.start, 12);
        assert_eq!(lexer.line, 3);
    }

    #[test]
    fn test_peek() {
        let test_cases = vec![
            ("Hello\nWorld", 0, 'H'),
            ("Hello\nWorld", 5, '\n'),
            ("Hello\nWorld", 6, 'W'),
            ("Hello\nWorld", 11, '\0'),
            ("Hello\nWorld", usize::MAX, '\0'),
        ];

        for (input, current, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            lexer.current = current;
            assert_eq!(
                lexer.peek().unwrap(),
                expected,
                "peek() failed for input: {}, current: {}",
                input,
                current
            );
        }
    }

    #[test]
    fn test_peek_next() {
        let test_cases = vec![
            ("Hello\nWorld", 0, 'e'),
            ("Hello\nWorld", 4, '\n'),
            ("Hello\nWorld", 5, 'W'),
            ("Hello\nWorld", 10, '\0'),
            ("Hello\nWorld", 11, '\0'),
            ("Hello\nWorld", usize::MAX - 1, '\0'),
        ];

        for (input, current, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            lexer.current = current;
            assert_eq!(
                lexer.peek_next().unwrap(),
                expected,
                "peek_next() failed for input: {}, current: {}",
                input,
                current
            );
        }
    }

    #[test]
    fn test_peek_previous() {
        let test_cases = vec![
            ("Hello\nWorld", 1, 'H'),
            ("Hello\nWorld", 5, 'o'),
            ("Hello\nWorld", 6, '\n'),
            ("Hello\nWorld", 0, '\0'),
            ("Hello\nWorld", 11, 'd'),
            ("Hello\nWorld", 12, '\0'),
            ("Hello\nWorld", usize::MAX, '\0'),
        ];

        for (input, current, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            lexer.current = current;
            assert_eq!(
                lexer.peek_previous().unwrap(),
                expected,
                "peek_previous() failed for input: {}, current: {}",
                input,
                current
            );
        }
    }

    #[test]
    fn test_peek_at() {
        let test_cases = vec![
            ("Hello\nWorld", 5, -2, 'l'),
            ("Hello\nWorld", 5, -1, 'o'),
            ("Hello\nWorld", 5, 0, '\n'),
            ("Hello\nWorld", 5, 1, 'W'),
            ("Hello\nWorld", 5, 2, 'o'),
            ("Hello\nWorld", 5, -6, '\0'),
            ("Hello\nWorld", 5, 6, '\0'),
            ("Hello\nWorld", 5, usize::MAX as isize - 6, '\0'),
        ];

        for (input, current, offset, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            lexer.current = current;
            assert_eq!(
                lexer.peek_at(offset).unwrap(),
                expected,
                "peek_at() failed for input: {}, current: {}, offset: {}",
                input,
                current,
                offset
            );
        }
    }

    #[test]
    fn test_item_at() {
        let test_cases = vec![
            ("Hello\nWorld", 0, 'H'),
            ("Hello\nWorld", 5, '\n'),
            ("Hello\nWorld", 6, 'W'),
            ("Hello\nWorld", 11, '\0'),
            ("Hello\nWorld", usize::MAX, '\0'),
        ];

        for (input, index, expected) in test_cases {
            let lexer = Lexer::new(input);
            assert_eq!(
                lexer.item_at(index).unwrap(),
                expected,
                "item_at() failed for input: {}, index: {}",
                input,
                index
            );
        }
    }

    #[test]
    fn test_is_at_end() {
        let test_cases = vec![
            // (input, index, expected_char)
            ("Hello\nWorld", 0, false),
            ("Hello\nWorld", 10, false),
            ("Hello\nWorld", 11, true),
            ("Hello\nWorld", usize::MAX, true),
        ];

        for (input, current, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            lexer.current = current;
            assert_eq!(
                lexer.is_at_end(),
                expected,
                "item_at() failed for input: {}, current: {}",
                input,
                current
            );
        }
    }
}
