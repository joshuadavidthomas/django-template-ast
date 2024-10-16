use crate::error::LexerError;
use crate::scanner::{LexerState, Scanner};
use crate::token::{Token, TokenType, Tokenizer};

pub struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    state: LexerState,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            tokens: Vec::new(),
            state: LexerState::new(),
        }
    }

    fn match_token_type(&mut self, c: char) -> Result<TokenType, LexerError> {
        match c {
            '(' | ')' | '[' | ']' | ',' | '-' | '+' | ':' | ';' | '*' | '|' | '\'' | '"' => {
                self.single_char(c)
            }
            '{' => self.left_brace(),
            '}' => self.right_brace(),
            '%' => self.percent(),
            '#' => self.hash(),
            '!' => self.bang(),
            '=' => self.equal(),
            '<' => self.left_angle(),
            '>' => self.right_angle(),
            '/' => self.slash(),
            '.' => self.dot(),
            ' ' | '\r' | '\t' | '\n' => self.whitespace(c),
            _ => self.text(),
        }
    }

    fn single_char(&mut self, c: char) -> Result<TokenType, LexerError> {
        let token_type = match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            ',' => TokenType::Comma,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ':' => TokenType::Colon,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            '|' => TokenType::Pipe,
            '\'' => TokenType::SingleQuote,
            '"' => TokenType::DoubleQuote,
            _ => {
                return Err(LexerError::UnexpectedCharacter {
                    character: c,
                    line: self.state.line,
                })
            }
        };
        Ok(token_type)
    }

    fn left_brace(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('{')? {
            TokenType::DoubleLeftBrace
        } else if self.advance_if_matches('%')? {
            TokenType::LeftBracePercent
        } else if self.advance_if_matches('#')? {
            TokenType::LeftBraceHash
        } else {
            TokenType::LeftBrace
        };
        Ok(token_type)
    }

    fn right_brace(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('}')? {
            TokenType::DoubleRightBrace
        } else {
            TokenType::RightBrace
        };
        Ok(token_type)
    }

    fn percent(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('}')? {
            TokenType::PercentRightBrace
        } else {
            TokenType::Percent
        };
        Ok(token_type)
    }

    fn hash(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('}')? {
            TokenType::HashRightBrace
        } else {
            TokenType::Hash
        };
        Ok(token_type)
    }

    fn bang(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('=')? {
            TokenType::BangEqual
        } else {
            TokenType::Bang
        };
        Ok(token_type)
    }

    fn equal(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('=')? {
            TokenType::DoubleEqual
        } else {
            TokenType::Equal
        };
        Ok(token_type)
    }

    fn left_angle(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('=')? {
            TokenType::LeftAngleEqual
        } else if self.advance_if_matches('!')? {
            let start_pos = self.state.current;
            self.advance_while(|c| c == '-')?;

            if self.state.current - start_pos >= 2 {
                TokenType::LeftAngleBangMinusMinus
            } else {
                self.state.current = start_pos;
                TokenType::LeftAngle
            }
        } else {
            TokenType::LeftAngle
        };

        Ok(token_type)
    }

    fn right_angle(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('=')? {
            TokenType::RightAngleEqual
        } else {
            TokenType::RightAngle
        };
        Ok(token_type)
    }

    fn slash(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('>')? {
            TokenType::SlashRightAngle
        } else if self.advance_if_matches('/')? {
            TokenType::DoubleSlash
        } else {
            TokenType::Slash
        };
        Ok(token_type)
    }

    fn dot(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('.')? {
            TokenType::DoubleDot
        } else {
            TokenType::Dot
        };
        Ok(token_type)
    }

    fn whitespace(&mut self, mut c: char) -> Result<TokenType, LexerError> {
        while !self.is_at_end() && self.peek()?.is_whitespace() {
            match c {
                '\n' => {
                    self.state.line += 1;
                }
                '\r' if self.peek()? == '\n' => {
                    self.advance()?;
                    self.state.line += 1;
                }
                ' ' | '\t' | '\r' => {}
                _ => {
                    return Err(LexerError::UnexpectedCharacter {
                        character: c,
                        line: self.state.line,
                    })
                }
            }
            c = self.advance()?;
        }
        Ok(TokenType::Whitespace)
    }

    fn text(&mut self) -> Result<TokenType, LexerError> {
        self.advance_while(|c| !Self::is_token_boundary(c))?;
        Ok(TokenType::Text)
    }

    fn advance_if_matches(&mut self, expected: char) -> Result<bool, LexerError> {
        if self.is_at_end() || self.peek()? != expected {
            Ok(false)
        } else {
            self.state.current += 1;
            Ok(true)
        }
    }

    fn advance_while<F>(&mut self, condition: F) -> Result<(), LexerError>
    where
        F: Fn(char) -> bool,
    {
        while !self.is_at_end() {
            let current_char = self.peek()?;
            if !condition(current_char) {
                break;
            }
            if current_char == '\n' {
                self.state.line += 1;
            }
            self.advance()?;
        }
        Ok(())
    }

    fn is_token_boundary(c: char) -> bool {
        const TOKEN_BOUNDARIES: &[char] = &[
            '(', ')', '[', ']', '{', '}', ',', '.', '-', '+', ':', ';', '*', '|', '%', '#', '!',
            '=', '<', '>', '/', ' ', '\r', '\t', '\n', '"', '\'',
        ];

        TOKEN_BOUNDARIES.contains(&c)
    }
}

impl<'a> Scanner for Lexer<'a> {
    type Item = char;
    type Error = LexerError;

    fn advance(&mut self) -> Result<Self::Item, Self::Error> {
        let current_char = self.peek()?;
        self.state.current += 1;
        Ok(current_char)
    }

    fn peek(&self) -> Result<Self::Item, Self::Error> {
        self.item_at(self.state.current)
    }

    fn peek_next(&self) -> Result<Self::Item, Self::Error> {
        self.item_at(self.state.current + 1)
    }

    fn peek_previous(&self) -> Result<Self::Item, Self::Error> {
        self.item_at(self.state.current - 1)
    }

    fn item_at(&self, index: usize) -> Result<Self::Item, Self::Error> {
        if let Some(ch) = self.source.chars().nth(index) {
            Ok(ch)
        } else {
            let error = if self.source.is_empty() {
                LexerError::EmptySource
            } else if index < self.state.current {
                LexerError::AtBeginningOfSource
            } else if index >= self.source.len() {
                LexerError::AtEndOfSource
            } else {
                LexerError::InvalidCharacterAccess
            };
            Err(error)
        }
    }

    fn is_at_end(&self) -> bool {
        self.state.current >= self.source.len()
    }
}

impl<'a> Tokenizer<'a> for Lexer<'a> {
    type Token = Token<'a>;
    type TokenType = TokenType;

    fn tokenize(&mut self) -> Result<Vec<Self::Token>, Self::Error> {
        while !self.is_at_end() {
            self.state.start = self.state.current;
            let (token_type, text) = self.next_token()?;
            self.add_token(token_type, text);
        }

        self.add_token(TokenType::Eof, "");
        Ok(self.tokens.clone())
    }

    fn next_token(&mut self) -> Result<(Self::TokenType, &'a str), Self::Error> {
        let c = self.advance()?;
        let token_type = self.match_token_type(c)?;
        let text = &self.source[self.state.start..self.state.current];
        Ok((token_type, text))
    }

    fn add_token(&mut self, token_type: Self::TokenType, text: &'a str) {
        if token_type != TokenType::Whitespace {
            self.tokens
                .push(Token::new(token_type, text, self.state.line));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod lexer {
        use super::*;

        #[test]
        fn test_lexer_new() {
            let lexer = Lexer::new("");

            assert_eq!(lexer.source, "");
            assert_eq!(lexer.tokens.len(), 0);
            assert_eq!(lexer.state.start, 0);
            assert_eq!(lexer.state.current, 0);
            assert_eq!(lexer.state.line, 1);
        }

        fn assert_token_type<F>(test_cases: Vec<(&str, TokenType)>, method: F)
        where
            F: Fn(&mut Lexer, Option<char>) -> Result<TokenType, LexerError>,
        {
            for (input, expected) in test_cases {
                println!("Testing input: {:?}", input);
                let mut chars = input.chars();
                let first_char = chars.next().unwrap();
                let rest: String = chars.collect();

                let mut lexer = Lexer::new(&rest);

                match method(&mut lexer, Some(first_char)) {
                    Ok(token_type) => assert_eq!(token_type, expected, "Input: {}", input),
                    Err(e) => panic!(
                        "Expected {:?}, but got Err({:?}) for input: {}",
                        expected, e, input
                    ),
                }
            }
        }

        #[test]
        fn test_match_token_type() {
            let test_cases = vec![
                ("(", TokenType::LeftParen),
                (")", TokenType::RightParen),
                ("{", TokenType::LeftBrace),
                ("}", TokenType::RightBrace),
                ("[", TokenType::LeftBracket),
                ("]", TokenType::RightBracket),
                ("<", TokenType::LeftAngle),
                (">", TokenType::RightAngle),
                (",", TokenType::Comma),
                (".", TokenType::Dot),
                ("-", TokenType::Minus),
                ("+", TokenType::Plus),
                (":", TokenType::Colon),
                (";", TokenType::Semicolon),
                ("/", TokenType::Slash),
                ("*", TokenType::Star),
                ("!", TokenType::Bang),
                ("=", TokenType::Equal),
                ("|", TokenType::Pipe),
                ("%", TokenType::Percent),
                ("#", TokenType::Hash),
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
                ("..", TokenType::DoubleDot),
                ("<!--", TokenType::LeftAngleBangMinusMinus),
                ("/>", TokenType::SlashRightAngle),
                ("//", TokenType::DoubleSlash),
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

            assert_token_type(test_cases, |lexer, c| lexer.match_token_type(c.unwrap()));
        }

        #[test]
        fn test_left_brace() {
            let test_cases = vec![
                ("{", TokenType::LeftBrace),
                ("{{", TokenType::DoubleLeftBrace),
                ("{%", TokenType::LeftBracePercent),
                ("{#", TokenType::LeftBraceHash),
                ("{a", TokenType::LeftBrace),
            ];

            assert_token_type(test_cases, |lexer, _| lexer.left_brace());
        }

        #[test]
        fn test_right_brace() {
            let test_cases = vec![
                ("}", TokenType::RightBrace),
                ("}}", TokenType::DoubleRightBrace),
            ];

            assert_token_type(test_cases, |lexer, _| lexer.right_brace());
        }

        #[test]
        fn test_percent() {
            let test_cases = vec![
                ("%", TokenType::Percent),
                ("%}", TokenType::PercentRightBrace),
            ];

            assert_token_type(test_cases, |lexer, _| lexer.percent());
        }

        #[test]
        fn test_hash() {
            let test_cases = vec![("#", TokenType::Hash), ("#}", TokenType::HashRightBrace)];

            assert_token_type(test_cases, |lexer, _| lexer.hash());
        }

        #[test]
        fn test_bang() {
            let test_cases = vec![("!", TokenType::Bang), ("!=", TokenType::BangEqual)];

            assert_token_type(test_cases, |lexer, _| lexer.bang());
        }

        #[test]
        fn test_equal() {
            let test_cases = vec![("=", TokenType::Equal), ("==", TokenType::DoubleEqual)];

            assert_token_type(test_cases, |lexer, _| lexer.equal());
        }

        #[test]
        fn test_left_angle() {
            let test_cases = vec![
                ("<", TokenType::LeftAngle),
                ("<=", TokenType::LeftAngleEqual),
                ("<!--", TokenType::LeftAngleBangMinusMinus),
                ("<!", TokenType::LeftAngle),
                ("<! ", TokenType::LeftAngle),
                ("<!-", TokenType::LeftAngle),
                ("<!- ", TokenType::LeftAngle),
                ("<!---", TokenType::LeftAngleBangMinusMinus), // Extra test case
            ];

            assert_token_type(test_cases, |lexer, _| lexer.left_angle());
        }

        #[test]
        fn test_right_angle() {
            let test_cases = vec![
                (">", TokenType::RightAngle),
                (">=", TokenType::RightAngleEqual),
            ];

            assert_token_type(test_cases, |lexer, _| lexer.right_angle());
        }

        #[test]
        fn test_slash() {
            let test_cases = vec![
                ("/", TokenType::Slash),
                ("/>", TokenType::SlashRightAngle),
                ("//", TokenType::DoubleSlash),
            ];

            assert_token_type(test_cases, |lexer, _| lexer.slash());
        }

        #[test]
        fn test_dot() {
            let test_cases = vec![(".", TokenType::Dot), ("..", TokenType::DoubleDot)];

            assert_token_type(test_cases, |lexer, _| lexer.dot());
        }

        #[test]
        fn test_whitespace() {
            let test_cases = vec![
                (" ", TokenType::Whitespace),
                ("\r", TokenType::Whitespace),
                ("\t", TokenType::Whitespace),
                ("\n", TokenType::Whitespace),
                ("  ", TokenType::Whitespace),
                (" \n", TokenType::Whitespace),
            ];

            assert_token_type(test_cases, |lexer, c| lexer.whitespace(c.unwrap()));
        }

        #[test]
        fn test_text() {
            let test_cases = vec![
                ("a", TokenType::Text),
                ("1", TokenType::Text),
                ("Hello", TokenType::Text),
            ];

            assert_token_type(test_cases, |lexer, _| lexer.text());
        }
    }

    fn tokenize(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input);
        match lexer.tokenize() {
            Ok(tokens) => {
                // Debug print all tokens
                for token in tokens.iter() {
                    println!("{:?}", token)
                }

                tokens
            }
            Err(e) => {
                eprintln!("Tokenization error: {:?}", e);
                eprintln!("Input that caused the error: {}", input);
                panic!("Tokenization failed. See error output above.");
            }
        }
    }

    #[test]
    fn test_opening_tag() {
        let tokens = tokenize("<html>");
        assert_eq!(tokens[0].token_type, TokenType::LeftAngle);
        assert_eq!(tokens[1].token_type, TokenType::Text);
        assert_eq!(tokens[2].token_type, TokenType::RightAngle);
    }

    #[test]
    fn test_closing_tag() {
        let tokens = tokenize("</body>");
        assert_eq!(tokens[0].token_type, TokenType::LeftAngle);
        assert_eq!(tokens[1].token_type, TokenType::Slash);
        assert_eq!(tokens[2].token_type, TokenType::Text);
        assert_eq!(tokens[3].token_type, TokenType::RightAngle);
    }

    #[test]
    fn test_html_attribute() {
        let tokens = tokenize(r#"<a href="link">"#);
        assert_eq!(tokens[0].token_type, TokenType::LeftAngle);
        assert_eq!(tokens[1].token_type, TokenType::Text);
        assert_eq!(tokens[2].token_type, TokenType::Text);
        assert_eq!(tokens[3].token_type, TokenType::Equal);
        assert_eq!(tokens[4].token_type, TokenType::DoubleQuote);
        assert_eq!(tokens[5].token_type, TokenType::Text);
        assert_eq!(tokens[6].token_type, TokenType::DoubleQuote);
        assert_eq!(tokens[7].token_type, TokenType::RightAngle);
    }

    #[test]
    fn test_django_variable() {
        let tokens = tokenize("{{ variable }}");
        assert_eq!(tokens[0].token_type, TokenType::DoubleLeftBrace);
        assert_eq!(tokens[1].token_type, TokenType::Text);
        assert_eq!(tokens[2].token_type, TokenType::DoubleRightBrace);
    }

    #[test]
    fn test_django_templatetag() {
        let tokens = tokenize("{% if condition %}");
        assert_eq!(tokens[0].token_type, TokenType::LeftBracePercent);
        assert_eq!(tokens[1].token_type, TokenType::Text);
        assert_eq!(tokens[2].token_type, TokenType::Text);
        assert_eq!(tokens[3].token_type, TokenType::PercentRightBrace);
    }

    #[test]
    fn test_django_comment() {
        let tokens = tokenize("{# This is a comment #}");
        assert_eq!(tokens[0].token_type, TokenType::LeftBraceHash);
        assert_eq!(tokens[1].token_type, TokenType::Text);
        assert_eq!(tokens[2].token_type, TokenType::Text);
        assert_eq!(tokens[3].token_type, TokenType::Text);
        assert_eq!(tokens[4].token_type, TokenType::Text);
        assert_eq!(tokens[5].token_type, TokenType::HashRightBrace);
    }

    #[test]
    fn test_django_filter() {
        let tokens = tokenize("{{ value|default:'default' }}");
        assert_eq!(tokens[0].token_type, TokenType::DoubleLeftBrace);
        assert_eq!(tokens[1].token_type, TokenType::Text);
        assert_eq!(tokens[2].token_type, TokenType::Pipe);
        assert_eq!(tokens[3].token_type, TokenType::Text);
        assert_eq!(tokens[4].token_type, TokenType::Colon);
        assert_eq!(tokens[5].token_type, TokenType::SingleQuote);
        assert_eq!(tokens[6].token_type, TokenType::Text);
        assert_eq!(tokens[7].token_type, TokenType::SingleQuote);
        assert_eq!(tokens[8].token_type, TokenType::DoubleRightBrace);
    }

    #[test]
    fn test_quoted_django_templatetag() {
        let tokens = tokenize(r#"'{% url "api:index" %}'"#);
        assert_eq!(tokens[0].token_type, TokenType::SingleQuote);
        assert_eq!(tokens[1].token_type, TokenType::LeftBracePercent);
        assert_eq!(tokens[2].token_type, TokenType::Text);
        assert_eq!(tokens[3].token_type, TokenType::DoubleQuote);
        assert_eq!(tokens[4].token_type, TokenType::Text);
        assert_eq!(tokens[5].token_type, TokenType::Colon);
        assert_eq!(tokens[6].token_type, TokenType::Text);
        assert_eq!(tokens[7].token_type, TokenType::DoubleQuote);
        assert_eq!(tokens[8].token_type, TokenType::PercentRightBrace);
        assert_eq!(tokens[9].token_type, TokenType::SingleQuote);
    }

    #[test]
    fn test_multiline_template() {
        let template = r#"\
        {% if user.is_authenticated %}
            Hello, {{ user.name }}!
        {% else %}
            Please log in.
        {% endif %}
    "#;
        let tokens = tokenize(template);
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[6].line, 2);
        assert_eq!(tokens[14].line, 3);
        assert_eq!(tokens[17].line, 4);
        assert_eq!(tokens[21].line, 5);
    }
}
