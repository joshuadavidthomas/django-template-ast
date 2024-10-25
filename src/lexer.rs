use crate::error::LexerError;
use crate::token::{Token, TokenStream, TokenType};

pub struct Lexer {
    source: String,
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: String::from(source),
            chars: source.chars().collect(),
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
        let token_type = self.match_token_type(c)?;
        let lexeme = self.extract_lexeme(&token_type)?;
        Ok(Token {
            token_type,
            lexeme,
            line: self.line,
        })
    }

    fn match_token_type(&self, c: char) -> Result<TokenType, LexerError> {
        let next_char = self.peek_next()?;
        let token_type = match c {
            '{' => match next_char {
                '%' => TokenType::Block,
                '{' => TokenType::Variable,
                '#' => TokenType::Comment,
                _ => TokenType::Text,
            },
            '<' => match next_char {
                '/' => TokenType::TagClose,
                _ => {
                    if self.peek_until("/>")? {
                        TokenType::TagVoid
                    } else {
                        TokenType::TagOpen
                    }
                }
            },
            _ => TokenType::Text,
        };
        Ok(token_type)
    }

    fn extract_lexeme(&mut self, token_type: &TokenType) -> Result<String, LexerError> {
        let lexeme = match token_type {
            TokenType::Block
            | TokenType::Variable
            | TokenType::Comment
            | TokenType::TagOpen
            | TokenType::TagVoid
            | TokenType::TagClose => {
                let (start, end) = token_type.delimiters();
                self.consume(start)?;
                let lexeme = self.consume_until(end)?;
                self.consume(end)?;
                lexeme
            }
            TokenType::Text => {
                const TOKEN_BOUNDARIES: &[char] = &['<', '>', '%', '{', '}'];
                let start = self.current;
                while !self.is_at_end() && !TOKEN_BOUNDARIES.contains(&self.peek()?) {
                    self.advance()?;
                }
                let end = self.current;
                self.source[start..end].trim().to_string()
            }
            _ => return Err(LexerError::UnexpectedTokenType(token_type.to_owned())),
        };
        Ok(lexeme)
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

    fn peek_until(&self, end: &str) -> Result<bool, LexerError> {
        let mut index = self.current;
        let end_chars: Vec<char> = end.chars().collect();

        while index < self.chars.len() {
            if self.chars[index..].starts_with(&end_chars) {
                return Ok(true);
            }
            index += 1;
        }
        Ok(false)
    }

    fn peek_at(&self, offset: isize) -> Result<char, LexerError> {
        let index = self.current as isize + offset;
        self.item_at(index as usize)
    }

    fn item_at(&self, index: usize) -> Result<char, LexerError> {
        if index >= self.chars.len() {
            // Return a null character when past the end, a bit of a departure from
            // idiomatic Rust code, but makes writing the matching above and testing
            // much easier
            Ok('\0')
        } else {
            Ok(self.chars[index])
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn advance(&mut self) -> Result<(), LexerError> {
        if self.is_at_end() {
            return Err(LexerError::AtEndOfSource);
        }
        self.start = self.current;
        self.current += 1;
        match self.peek_previous()? {
            '\n' => self.line += 1,
            '\r' => {
                self.line += 1;
                if self.peek()? == '\n' {
                    self.current += 1;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn consume(&mut self, s: &str) -> Result<char, LexerError> {
        for c in s.chars() {
            if c != self.peek()? {
                return Err(LexerError::UnexpectedCharacter(c, self.line));
            }
            self.advance()?;
        }
        self.peek_previous()
    }

    fn consume_until(&mut self, s: &str) -> Result<String, LexerError> {
        let start = self.current;
        while !self.is_at_end() {
            if self.chars[self.current..self.chars.len()]
                .starts_with(s.chars().collect::<Vec<_>>().as_slice())
            {
                let end = self.current;
                return Ok(self.source[start..end].trim().to_string());
            }
            self.advance()?;
        }
        Err(LexerError::UnexpectedEndOfInput)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let test_cases = vec![
            ("<html>", vec![TokenType::TagOpen, TokenType::Eof]),
            ("</body>", vec![TokenType::TagClose, TokenType::Eof]),
            (
                "<img src='https://example.com/image.jpg' />",
                vec![TokenType::TagVoid, TokenType::Eof],
            ),
            ("{{ variable }}", vec![TokenType::Variable, TokenType::Eof]),
            ("{% if condition %}", vec![TokenType::Block, TokenType::Eof]),
            ("{# A comment #}", vec![TokenType::Comment, TokenType::Eof]),
            (
                "{{ value|default:'default' }}",
                vec![TokenType::Variable, TokenType::Eof],
            ),
            (
                r#"'{% url "api:index" %}'"#,
                vec![
                    TokenType::Text,
                    TokenType::Block,
                    TokenType::Text,
                    TokenType::Eof,
                ],
            ),
            ("This is text.", vec![TokenType::Text, TokenType::Eof]),
            (
                "This is text.\nThis is a new line.",
                vec![TokenType::Text, TokenType::Eof],
            ),
        ];

        for (input, expected_token_types) in test_cases {
            println!("Testing input: {:?}", input);

            let mut lexer = Lexer::new(input);
            let tokens = lexer.tokenize().unwrap();

            println!("tokens: {:?}", tokens);

            assert_eq!(
                tokens.len(),
                expected_token_types.len(),
                "Number of tokens doesn't match for input: {}",
                input
            );

            for (token, expected_type) in tokens.iter().zip(expected_token_types.iter()) {
                assert_eq!(
                    token.token_type, *expected_type,
                    "Token type mismatch for input: {}",
                    input
                );
            }

            println!("---");
        }
    }
}
