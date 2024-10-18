use crate::error::TokenError;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftAngle,             // <
    RightAngle,            // >
    Comma,                 // ,
    Dot,                   // .
    Dash,                  // -
    Plus,                  // +
    Colon,                 // :
    Slash,                 // /
    Bang,                  // !
    Equal,                 // =
    Pipe,                  // |
    Percent,               // %
    SingleQuote,           // '
    DoubleQuote,           // "
    DoubleLeftBrace,       // {{
    DoubleRightBrace,      // }}
    LeftBracePercent,      // {%
    PercentRightBrace,     // %}
    LeftBraceHash,         // {#
    HashRightBrace,        // #}
    BangEqual,             // !=
    DoubleEqual,           // ==
    LeftAngleEqual,        // <=
    RightAngleEqual,       // =>
    LeftAngleBangDashDash, // <!--
    DashDashRightAngle,    // -->
    SlashRightAngle,       // />
    DoubleSlash,           // //
    SlashStar,             // /*
    StarSlash,             // */
    Whitespace,            // special token to account for whitespace
    Text,
    Eof,
}

impl TokenType {
    fn from_char(c: char, source: &str) -> Result<Self, TokenError> {
        let token_type = match c {
            ',' | '.' | '+' | ':' | '|' | '\'' | '"' => TokenType::single_char(c)?,
            '{' => TokenType::left_brace(source)?,
            '}' => TokenType::right_brace(source)?,
            '%' => TokenType::percent(source)?,
            '#' => TokenType::hash(source)?,
            '!' => TokenType::bang(source)?,
            '=' => TokenType::equal(source)?,
            '<' => TokenType::left_angle(source)?,
            '>' => TokenType::right_angle(source)?,
            '/' => TokenType::slash(source)?,
            '-' => TokenType::dash(source)?,
            '*' => TokenType::star(source)?,
            c if c.is_whitespace() => TokenType::Whitespace,
            _ => TokenType::Text,
        };
        Ok(token_type)
    }

    fn single_char(c: char) -> Result<Self, TokenError> {
        let token_type = match c {
            ',' => Self::Comma,
            '.' => Self::Dot,
            '+' => Self::Plus,
            ':' => Self::Colon,
            '|' => Self::Pipe,
            '\'' => Self::SingleQuote,
            '"' => Self::DoubleQuote,
            '/' => Self::Slash,
            '%' => Self::Percent,
            _ => return Err(TokenError::UnexpectedCharacter { character: c }),
        };
        Ok(token_type)
    }

    fn left_brace(s: &str) -> Result<Self, TokenError> {
        let token_type = if s.starts_with("{{") {
            Self::DoubleLeftBrace
        } else if s.starts_with("{%") {
            Self::LeftBracePercent
        } else if s.starts_with("{#") {
            Self::LeftBraceHash
        } else {
            Self::Text
        };
        Ok(token_type)
    }

    fn right_brace(s: &str) -> Result<Self, TokenError> {
        let token_type = if s.starts_with("}}") {
            Self::DoubleRightBrace
        } else {
            Self::Text
        };
        Ok(token_type)
    }

    fn percent(s: &str) -> Result<Self, TokenError> {
        let token_type = if s.starts_with("%}") {
            Self::PercentRightBrace
        } else {
            Self::Percent
        };
        Ok(token_type)
    }

    fn hash(s: &str) -> Result<Self, TokenError> {
        let token_type = if s.starts_with("#}") {
            Self::HashRightBrace
        } else {
            Self::Text
        };
        Ok(token_type)
    }

    fn bang(s: &str) -> Result<Self, TokenError> {
        let token_type = if s.starts_with("!=") {
            Self::BangEqual
        } else {
            Self::Bang
        };
        Ok(token_type)
    }

    fn equal(s: &str) -> Result<Self, TokenError> {
        let token_type = if s.starts_with("==") {
            Self::DoubleEqual
        } else {
            Self::Equal
        };
        Ok(token_type)
    }

    fn left_angle(s: &str) -> Result<Self, TokenError> {
        let token_type = if s.starts_with("<=") {
            Self::LeftAngleEqual
        } else if s.starts_with("<!--") {
            Self::LeftAngleBangDashDash
        } else {
            Self::LeftAngle
        };
        Ok(token_type)
    }

    fn right_angle(s: &str) -> Result<Self, TokenError> {
        let token_type = if s.starts_with(">=") {
            Self::RightAngleEqual
        } else {
            Self::RightAngle
        };
        Ok(token_type)
    }

    fn slash(s: &str) -> Result<Self, TokenError> {
        let token_type = if s.starts_with("/>") {
            Self::SlashRightAngle
        } else if s.starts_with("//") {
            Self::DoubleSlash
        } else if s.starts_with("/*") {
            Self::SlashStar
        } else if s.starts_with("*/") {
            Self::StarSlash
        } else {
            Self::Slash
        };
        Ok(token_type)
    }

    fn dash(s: &str) -> Result<Self, TokenError> {
        let token_type = if let Some(rest) = s.strip_prefix("--") {
            if rest.starts_with(">") {
                Self::DashDashRightAngle
            } else {
                Self::Text
            }
        } else {
            Self::Dash
        };
        Ok(token_type)
    }

    fn star(s: &str) -> Result<Self, TokenError> {
        let token_type = if s.starts_with("*/") {
            Self::StarSlash
        } else {
            Self::Text
        };
        Ok(token_type)
    }

    pub fn size(&self, s: Option<&str>) -> Result<usize, TokenError> {
        let size = match self {
            Self::Eof => 0,
            Self::LeftAngle
            | Self::RightAngle
            | Self::Comma
            | Self::Dot
            | Self::Dash
            | Self::Plus
            | Self::Colon
            | Self::Slash
            | Self::Bang
            | Self::Equal
            | Self::Pipe
            | Self::Percent
            | Self::SingleQuote
            | Self::DoubleQuote => 1,
            Self::DoubleLeftBrace
            | Self::DoubleRightBrace
            | Self::LeftBracePercent
            | Self::PercentRightBrace
            | Self::LeftBraceHash
            | Self::HashRightBrace
            | Self::BangEqual
            | Self::DoubleEqual
            | Self::LeftAngleEqual
            | Self::RightAngleEqual
            | Self::SlashRightAngle
            | Self::DoubleSlash
            | Self::SlashStar
            | Self::StarSlash => 2,
            Self::DashDashRightAngle => 3,
            Self::LeftAngleBangDashDash => 4,
            Self::Whitespace => s
                .expect("must provide source to get size")
                .chars()
                .take_while(|&c| c.is_whitespace())
                .map(|c| c.len_utf8())
                .sum(),
            Self::Text => {
                let mut size = 0;
                const TOKEN_BOUNDARIES: &[char] = &[
                    '(', ')', '[', ']', '{', '}', ',', '.', '-', '+', ':', ';', '*', '|', '%', '#',
                    '!', '=', '<', '>', '/', ' ', '\r', '\t', '\n', '"', '\'',
                ];
                for (i, c) in s
                    .expect("must provide source to get size")
                    .chars()
                    .enumerate()
                {
                    if TOKEN_BOUNDARIES.contains(&c) {
                        break;
                    }
                    size = i + 1;
                }
                size
            }
        };
        Ok(size)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl<'a> Token {
    pub fn new(token_type: TokenType, lexeme: &'a str, line: usize) -> Self {
        Token {
            token_type,
            lexeme: String::from(lexeme),
            line,
        }
    }

    pub fn from_source(source: &'a str, current_line: usize) -> Result<Self, TokenError> {
        let c = source.chars().next().ok_or(TokenError::NoTokenMatch)?;
        let token_type = TokenType::from_char(c, source)?;
        let size = match token_type {
            TokenType::Whitespace | TokenType::Text => token_type.size(Some(source))?,
            _ => token_type.size(None)?,
        };
        let lexeme = &source[..size.min(source.len())];
        Ok(Token::new(token_type, lexeme, current_line))
    }

    pub fn eof(line: usize) -> Self {
        Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            line,
        }
    }

    pub fn size(&self) -> usize {
        self.lexeme.chars().count()
    }

    pub fn lines(&self) -> usize {
        match self.token_type {
            TokenType::Whitespace => self
                .lexeme
                .chars()
                .filter(|&c| c == '\n' || c == '\r')
                .count(),
            _ => 0,
        }
    }

    pub fn is_throwaway(&self) -> bool {
        matches!(self.token_type, TokenType::Whitespace)
    }
}

#[derive(Clone, Debug)]
pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new() -> Self {
        TokenStream { tokens: Vec::new() }
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn finalize(&mut self, last_line: usize) -> TokenStream {
        let eof_token = Token::eof(last_line);
        self.add_token(eof_token);
        self.clone()
    }
}

impl Default for TokenStream {
    fn default() -> Self {
        TokenStream::new()
    }
}

impl AsRef<[Token]> for TokenStream {
    fn as_ref(&self) -> &[Token] {
        &self.tokens
    }
}

impl Deref for TokenStream {
    type Target = Vec<Token>;

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl DerefMut for TokenStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tokens
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl<'a> IntoIterator for &'a TokenStream {
    type Item = &'a Token;
    type IntoIter = std::slice::Iter<'a, Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_token_type<F>(test_cases: Vec<(&str, TokenType)>, method: F)
    where
        F: Fn(&str) -> Result<TokenType, TokenError>,
    {
        for (input, expected_token_type) in test_cases {
            println!("Testing input: {:?}", input);

            match method(input) {
                Ok(token_type) => {
                    assert_eq!(token_type, expected_token_type, "Input: {}", input);
                }
                Err(e) => panic!(
                    "Expected {:?}, but got Err({:?}) for input: {}",
                    expected_token_type, e, input,
                ),
            }
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

        assert_token_type(test_cases, TokenType::left_brace);
    }

    #[test]
    fn test_right_brace() {
        let test_cases = vec![("}}", TokenType::DoubleRightBrace), ("}", TokenType::Text)];

        assert_token_type(test_cases, TokenType::right_brace);
    }

    #[test]
    fn test_percent() {
        let test_cases = vec![
            ("%", TokenType::Percent),
            ("%}", TokenType::PercentRightBrace),
        ];

        assert_token_type(test_cases, TokenType::percent);
    }

    #[test]
    fn test_bang() {
        let test_cases = vec![("!", TokenType::Bang), ("!=", TokenType::BangEqual)];

        assert_token_type(test_cases, TokenType::bang);
    }

    #[test]
    fn test_equal() {
        let test_cases = vec![("=", TokenType::Equal), ("==", TokenType::DoubleEqual)];

        assert_token_type(test_cases, TokenType::equal);
    }

    #[test]
    fn test_left_angle() {
        let test_cases = vec![
            ("<", TokenType::LeftAngle),
            ("<=", TokenType::LeftAngleEqual),
            ("<!--", TokenType::LeftAngleBangDashDash),
            ("<!", TokenType::LeftAngle),
            ("<!-", TokenType::LeftAngle),
            ("<!---", TokenType::LeftAngleBangDashDash),
        ];

        assert_token_type(test_cases, TokenType::left_angle);
    }

    #[test]
    fn test_right_angle() {
        let test_cases = vec![
            (">", TokenType::RightAngle),
            (">=", TokenType::RightAngleEqual),
        ];

        assert_token_type(test_cases, TokenType::right_angle);
    }

    #[test]
    fn test_slash() {
        let test_cases = vec![
            ("/", TokenType::Slash),
            ("/>", TokenType::SlashRightAngle),
            ("//", TokenType::DoubleSlash),
            ("/*", TokenType::SlashStar),
        ];

        assert_token_type(test_cases, TokenType::slash);
    }

    #[test]
    fn test_dash() {
        let test_cases = vec![
            ("-", TokenType::Dash),
            ("-->", TokenType::DashDashRightAngle),
            ("--", TokenType::Text),
        ];

        assert_token_type(test_cases, TokenType::dash);
    }

    #[test]
    fn test_star() {
        let test_cases = vec![("*/", TokenType::StarSlash), ("*", TokenType::Text)];

        assert_token_type(test_cases, TokenType::star);
    }

    fn assert_token_instance<F>(test_cases: Vec<(&str, Token)>, method: F)
    where
        F: Fn(&str) -> Result<Token, TokenError>,
    {
        for (input, expected_token) in test_cases {
            println!("Testing input: {:?}", input);

            match method(input) {
                Ok(token) => {
                    assert_eq!(token, expected_token, "Input: {}", input);
                }
                Err(e) => panic!(
                    "Expected {:?}, but got Err({:?}) for input: {}",
                    expected_token, e, input,
                ),
            }
        }
    }

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
                "line",
                Token {
                    token_type: TokenType::Text,
                    lexeme: "line".to_string(),
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

        assert_token_instance(test_cases, |input| Token::from_source(input, line));
    }
}
