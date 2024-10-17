use crate::error::TokenError;
use std::fmt::Debug;

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
    fn single_char(c: char) -> Result<(Self, usize), TokenError> {
        let token_type;
        let size = 1;

        token_type = match c {
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

        Ok((token_type, size))
    }

    fn left_brace(s: &str) -> Result<(Self, usize), TokenError> {
        let token_type;
        let size;

        if s.starts_with("{{") {
            token_type = Self::DoubleLeftBrace;
            size = 2;
        } else if s.starts_with("{%") {
            token_type = Self::LeftBracePercent;
            size = 2;
        } else if s.starts_with("{#") {
            token_type = Self::LeftBraceHash;
            size = 2;
        } else {
            token_type = Self::Text;
            size = 1;
        }

        Ok((token_type, size))
    }

    fn right_brace(s: &str) -> Result<(Self, usize), TokenError> {
        let token_type;
        let size;

        if s.starts_with("}}") {
            token_type = Self::DoubleRightBrace;
            size = 2;
        } else {
            token_type = Self::Text;
            size = 1;
        }

        Ok((token_type, size))
    }

    fn percent(s: &str) -> Result<(Self, usize), TokenError> {
        let token_type;
        let size;

        if s.starts_with("%}") {
            token_type = Self::PercentRightBrace;
            size = 2;
        } else {
            token_type = Self::Percent;
            size = 1;
        }

        Ok((token_type, size))
    }

    fn hash(s: &str) -> Result<(Self, usize), TokenError> {
        let token_type;
        let size;

        if s.starts_with("#}") {
            token_type = Self::HashRightBrace;
            size = 2;
        } else {
            token_type = Self::Text;
            size = 1;
        }

        Ok((token_type, size))
    }

    fn bang(s: &str) -> Result<(Self, usize), TokenError> {
        let token_type;
        let size;

        if s.starts_with("!=") {
            token_type = Self::BangEqual;
            size = 2;
        } else {
            token_type = Self::Bang;
            size = 1;
        }

        Ok((token_type, size))
    }

    fn equal(s: &str) -> Result<(Self, usize), TokenError> {
        let token_type;
        let size;

        if s.starts_with("==") {
            token_type = Self::DoubleEqual;
            size = 2;
        } else {
            token_type = Self::Equal;
            size = 1;
        }

        Ok((token_type, size))
    }

    fn left_angle(s: &str) -> Result<(Self, usize), TokenError> {
        let token_type;
        let size;

        if s.starts_with("<=") {
            token_type = Self::LeftAngleEqual;
            size = 2;
        } else if s.starts_with("<!--") {
            token_type = Self::LeftAngleBangDashDash;
            size = 5;
        } else {
            token_type = Self::LeftAngle;
            size = 1;
        }

        Ok((token_type, size))
    }

    fn right_angle(s: &str) -> Result<(Self, usize), TokenError> {
        let token_type;
        let size;

        if s.starts_with(">=") {
            token_type = Self::RightAngleEqual;
            size = 2;
        } else {
            token_type = Self::RightAngle;
            size = 1;
        }

        Ok((token_type, size))
    }

    fn slash(s: &str) -> Result<(Self, usize), TokenError> {
        let token_type;
        let size;

        if s.starts_with("/>") {
            token_type = Self::SlashRightAngle;
            size = 2;
        } else if s.starts_with("//") {
            token_type = Self::DoubleSlash;
            size = 2;
        } else if s.starts_with("/*") {
            token_type = Self::SlashStar;
            size = 2;
        } else if s.starts_with("*/") {
            token_type = Self::StarSlash;
            size = 2;
        } else {
            token_type = Self::Slash;
            size = 1;
        }

        Ok((token_type, size))
    }

    fn dash(s: &str) -> Result<(Self, usize), TokenError> {
        let token_type;
        let size;

        if let Some(rest) = s.strip_prefix("--") {
            if rest.starts_with(">") {
                token_type = Self::DashDashRightAngle;
                size = 3;
            } else {
                token_type = Self::Text;
                size = 2;
            }
        } else {
            token_type = Self::Dash;
            size = 1;
        }

        Ok((token_type, size))
    }

    fn star(s: &str) -> Result<(Self, usize), TokenError> {
        let token_type;
        let size;

        if s.starts_with("*/") {
            token_type = Self::StarSlash;
            size = 2;
        } else {
            token_type = Self::Text;
            size = 1;
        }

        Ok((token_type, size))
    }

    fn whitespace(s: &str) -> Result<(Self, usize, usize), TokenError> {
        let mut size = 0;
        let mut lines = 0;
        let mut chars = s.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                ' ' | '\t' => {}
                '\n' => {
                    lines += 1;
                }
                '\r' => {
                    chars.next();
                    if chars.peek() == Some(&'\n') {
                        chars.next();
                    }
                    lines += 1;
                }
                _ => break,
            }

            size += c.len_utf8();
            chars.next();
        }

        if size > 0 {
            Ok((Self::Whitespace, size, lines))
        } else {
            Err(TokenError::NoTokenMatch)
        }
    }

    fn text(s: &str) -> Result<(Self, usize), TokenError> {
        let mut size = 0;

        for (i, c) in s.chars().enumerate() {
            if Self::is_token_boundary(c) {
                break;
            }
            size = i + 1;
        }

        if size > 0 {
            Ok((Self::Text, size))
        } else {
            Err(TokenError::NoTokenMatch)
        }
    }

    fn is_token_boundary(c: char) -> bool {
        const TOKEN_BOUNDARIES: &[char] = &[
            '(', ')', '[', ']', '{', '}', ',', '.', '-', '+', ':', ';', '*', '|', '%', '#', '!',
            '=', '<', '>', '/', ' ', '\r', '\t', '\n', '"', '\'',
        ];

        TOKEN_BOUNDARIES.contains(&c)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }

    pub fn from_input(input: &'a str, line: usize) -> Result<(Self, usize, usize), TokenError> {
        let c = input.chars().next().ok_or(TokenError::NoTokenMatch)?;

        if c.is_whitespace() {
            let (token_type, size, lines_consumed) = TokenType::whitespace(input)?;
            return Ok((
                Self::new(token_type, &input[..size.min(input.len())], line),
                size,
                lines_consumed,
            ));
        }

        let (token_type, size) = match c {
            ',' | '.' | '+' | ':' | '|' | '\'' | '"' => TokenType::single_char(c)?,
            '{' => TokenType::left_brace(input)?,
            '}' => TokenType::right_brace(input)?,
            '%' => TokenType::percent(input)?,
            '#' => TokenType::hash(input)?,
            '!' => TokenType::bang(input)?,
            '=' => TokenType::equal(input)?,
            '<' => TokenType::left_angle(input)?,
            '>' => TokenType::right_angle(input)?,
            '/' => TokenType::slash(input)?,
            '-' => TokenType::dash(input)?,
            '*' => TokenType::star(input)?,
            _ => TokenType::text(input)?,
        };

        Ok((
            Self::new(token_type, &input[..size.min(input.len())], line),
            size,
            0,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_token_instance<F>(test_cases: Vec<(&str, TokenType)>, method: F)
    where
        F: Fn(&str) -> Result<(Token<'_>, usize, usize), TokenError>,
    {
        for (input, expected_token_type) in test_cases {
            println!("Testing input: {:?}", input);

            // Call the token-based method
            match method(input) {
                Ok((token, _size_consumed, _lines_consumed)) => {
                    assert_eq!(token.token_type, expected_token_type, "Input: {}", input);
                }
                Err(e) => panic!(
                    "Expected {:?}, but got Err({:?}) for input: {}",
                    expected_token_type, e, input,
                ),
            }
        }
    }

    #[test]
    fn test_match_token() {
        let test_cases = vec![
            ("<", TokenType::LeftAngle),
            (">", TokenType::RightAngle),
            (",", TokenType::Comma),
            (".", TokenType::Dot),
            ("-", TokenType::Dash),
            ("+", TokenType::Plus),
            (":", TokenType::Colon),
            ("/", TokenType::Slash),
            ("!", TokenType::Bang),
            ("=", TokenType::Equal),
            ("|", TokenType::Pipe),
            ("%", TokenType::Percent),
            ("'", TokenType::SingleQuote),
            ("\"", TokenType::DoubleQuote),
            ("{{", TokenType::DoubleLeftBrace),
            ("}}", TokenType::DoubleRightBrace),
            ("{%", TokenType::LeftBracePercent),
            ("%}", TokenType::PercentRightBrace),
            ("{#", TokenType::LeftBraceHash),
            ("#}", TokenType::HashRightBrace),
            ("!=", TokenType::BangEqual),
            ("==", TokenType::DoubleEqual),
            ("<=", TokenType::LeftAngleEqual),
            (">=", TokenType::RightAngleEqual),
            ("<!--", TokenType::LeftAngleBangDashDash),
            ("-->", TokenType::DashDashRightAngle),
            ("/>", TokenType::SlashRightAngle),
            ("//", TokenType::DoubleSlash),
            ("/*", TokenType::SlashStar),
            ("*/", TokenType::StarSlash),
            (" ", TokenType::Whitespace),
            ("\r", TokenType::Whitespace),
            ("\t", TokenType::Whitespace),
            ("\n", TokenType::Whitespace),
            ("  ", TokenType::Whitespace),
            (" \n", TokenType::Whitespace),
            ("a", TokenType::Text),
            ("1", TokenType::Text),
            ("Hello", TokenType::Text),
        ];

        assert_token_instance(test_cases, |input| Token::from_input(input, 0));
    }

    fn assert_token_type<F>(test_cases: Vec<(&str, TokenType)>, method: F)
    where
        F: Fn(&str) -> Result<(TokenType, usize), TokenError>,
    {
        for (input, expected_token_type) in test_cases {
            println!("Testing input: {:?}", input);

            match method(input) {
                Ok((token_type, _size_consumed)) => {
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

    #[test]
    fn test_text() {
        let test_cases = vec![
            ("a", TokenType::Text),
            ("1", TokenType::Text),
            ("Hello", TokenType::Text),
        ];

        assert_token_type(test_cases, TokenType::text);
    }

    fn assert_whitespace_token_type<F>(test_cases: Vec<(&str, usize)>, method: F)
    where
        F: Fn(&str) -> Result<(TokenType, usize, usize), TokenError>,
    {
        for (input, expected_lines) in test_cases {
            println!("Testing input: {:?}", input);

            // Call the token matcher
            match method(input) {
                Ok((token_type, _size_consumed, lines_consumed)) => {
                    assert_eq!(token_type, TokenType::Whitespace, "Input: {}", input);
                    assert_eq!(lines_consumed, expected_lines, "Input: {}", input);
                }
                Err(e) => panic!(
                    "Expected Whitespace, but got Err({:?}) for input: {}",
                    e, input
                ),
            }
        }
    }
    #[test]
    fn test_whitespace_token_type() {
        let test_cases = vec![
            (" ", 0),
            ("\n", 1),
            ("\t", 0),
            ("\r", 1),
            (" \n", 1),
            ("\r\n", 1),
        ];

        assert_whitespace_token_type(test_cases, TokenType::whitespace);
    }
}
