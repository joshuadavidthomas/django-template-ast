use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    LeftParen,         // (
    RightParen,        // )
    LeftBrace,         // {
    RightBrace,        // }
    LeftBracket,       // [
    RightBracket,      // ]
    LeftAngle,         // <
    RightAngle,        // >
    Comma,             // ,
    Dot,               // .
    Minus,             // -
    Plus,              // +
    Colon,             // :
    Semicolon,         // ;
    Slash,             // /
    Star,              // *
    Bang,              // !
    Equal,             // =
    Pipe,              // |
    Percent,           // %
    Hash,              // #
    SingleQuote,       // '
    DoubleQuote,       // "
    DoubleLeftBrace,   // {{
    DoubleRightBrace,  // }}
    LeftBracePercent,  // {%
    PercentRightBrace, // %}
    LeftBraceHash,     // {#
    HashRightBrace,    // #}
    BangEqual,         // !=
    DoubleEqual,       // ==
    LeftAngleEqual,    // <=
    RightAngleEqual,   // =>
    Text,
    Eof,
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.lexeme,
            if let Some(literal) = &self.literal {
                format!(" ({})", literal)
            } else {
                String::new()
            }
        )
    }
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line));
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        use TokenType::*;
        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '[' => self.add_token(LeftBracket),
            ']' => self.add_token(RightBracket),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ':' => self.add_token(Colon),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '|' => self.add_token(Pipe),
            '\'' => self.add_token(SingleQuote),
            '"' => self.add_token(DoubleQuote),
            '{' => {
                let token_type = if self.match_char('{') {
                    DoubleLeftBrace
                } else if self.match_char('%') {
                    LeftBracePercent
                } else if self.match_char('#') {
                    LeftBraceHash
                } else {
                    LeftBrace
                };
                self.add_token(token_type);
            }
            '}' => {
                let token_type = if self.match_char('}') {
                    DoubleRightBrace
                } else {
                    RightBrace
                };
                self.add_token(token_type);
            }
            '%' => {
                let token_type = if self.match_char('}') {
                    PercentRightBrace
                } else {
                    Percent
                };
                self.add_token(token_type);
            }
            '#' => {
                let token_type = if self.match_char('}') {
                    HashRightBrace
                } else {
                    Hash
                };
                self.add_token(token_type);
            }
            '!' => {
                let token_type = if self.match_char('=') {
                    BangEqual
                } else {
                    Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    DoubleEqual
                } else {
                    Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    LeftAngleEqual
                } else {
                    LeftAngle
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    RightAngleEqual
                } else {
                    RightAngle
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            _ => self.text(),
        }
    }

    fn text(&mut self) {
        while !self.is_at_end() && !self.is_delimiter(self.peek()) {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        if !text.is_empty() {
            self.add_token_literal(TokenType::Text, Some(text));
        }
    }

    fn is_delimiter(&self, c: char) -> bool {
        matches!(
            c,
            '(' | ')'
                | '['
                | ']'
                | '{'
                | '}'
                | ','
                | '.'
                | '-'
                | '+'
                | ':'
                | ';'
                | '*'
                | '|'
                | '%'
                | '#'
                | '!'
                | '='
                | '<'
                | '>'
                | '/'
                | ' '
                | '\r'
                | '\t'
                | '\n'
                | '"'
                | '\''
        )
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap_or('\0')
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap_or('\0')
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let current_char = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        current_char
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenType::*;

    fn tokenize(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input.to_string());
        lexer.tokenize()
    }

    #[test]
    fn test_opening_tag() {
        let tokens = tokenize("<html>");
        assert_eq!(tokens[0].token_type, LeftAngle);
        assert_eq!(tokens[1].token_type, Text);
        assert_eq!(tokens[1].lexeme, "html");
        assert_eq!(tokens[2].token_type, RightAngle);
    }

    #[test]
    fn test_closing_tag() {
        let tokens = tokenize("</body>");
        assert_eq!(tokens[0].token_type, LeftAngle);
        assert_eq!(tokens[1].token_type, Slash);
        assert_eq!(tokens[2].token_type, Text);
        assert_eq!(tokens[2].lexeme, "body");
        assert_eq!(tokens[3].token_type, RightAngle);
    }

    #[test]
    fn test_html_attribute() {
        let tokens = tokenize(r#"<a href="link">"#);
        assert_eq!(tokens[0].token_type, LeftAngle);
        assert_eq!(tokens[1].token_type, Text);
        assert_eq!(tokens[1].lexeme, "a");
        assert_eq!(tokens[2].token_type, Text);
        assert_eq!(tokens[2].lexeme, "href");
        assert_eq!(tokens[3].token_type, Equal);
        assert_eq!(tokens[4].token_type, DoubleQuote);
        assert_eq!(tokens[5].token_type, Text);
        assert_eq!(tokens[5].lexeme, "link");
        assert_eq!(tokens[6].token_type, DoubleQuote);
        assert_eq!(tokens[7].token_type, RightAngle);
    }

    #[test]
    fn test_django_variable() {
        let tokens = tokenize("{{ variable }}");
        assert_eq!(tokens[0].token_type, DoubleLeftBrace);
        assert_eq!(tokens[1].token_type, Text);
        assert_eq!(tokens[1].lexeme, "variable");
        assert_eq!(tokens[2].token_type, DoubleRightBrace);
    }

    #[test]
    fn test_django_templatetag() {
        let tokens = tokenize("{% if condition %}");
        assert_eq!(tokens[0].token_type, LeftBracePercent);
        assert_eq!(tokens[1].token_type, Text);
        assert_eq!(tokens[1].lexeme, "if");
        assert_eq!(tokens[2].token_type, Text);
        assert_eq!(tokens[2].lexeme, "condition");
        assert_eq!(tokens[3].token_type, PercentRightBrace);
    }

    #[test]
    fn test_django_comment() {
        let tokens = tokenize("{# This is a comment #}");
        assert_eq!(tokens[0].token_type, LeftBraceHash);
        assert_eq!(tokens[1].token_type, Text);
        assert_eq!(tokens[1].lexeme, "This");
        assert_eq!(tokens[2].token_type, Text);
        assert_eq!(tokens[2].lexeme, "is");
        assert_eq!(tokens[3].token_type, Text);
        assert_eq!(tokens[3].lexeme, "a");
        assert_eq!(tokens[4].token_type, Text);
        assert_eq!(tokens[4].lexeme, "comment");
        assert_eq!(tokens[5].token_type, HashRightBrace);
    }

    #[test]
    fn test_django_filter() {
        let tokens = tokenize("{{ value|default:'default' }}");
        assert_eq!(tokens[0].token_type, DoubleLeftBrace);
        assert_eq!(tokens[1].token_type, Text);
        assert_eq!(tokens[1].lexeme, "value");
        assert_eq!(tokens[2].token_type, Pipe);
        assert_eq!(tokens[3].token_type, Text);
        assert_eq!(tokens[3].lexeme, "default");
        assert_eq!(tokens[4].token_type, Colon);
        assert_eq!(tokens[5].token_type, SingleQuote);
        assert_eq!(tokens[6].token_type, Text);
        assert_eq!(tokens[6].lexeme, "default");
        assert_eq!(tokens[7].token_type, SingleQuote);
        assert_eq!(tokens[8].token_type, DoubleRightBrace);
    }

    #[test]
    fn test_quoted_django_templatetag() {
        let tokens = tokenize(r#"'{% url "api:index" %}'"#);
        assert_eq!(tokens[0].token_type, SingleQuote);
        assert_eq!(tokens[1].token_type, LeftBracePercent);
        assert_eq!(tokens[2].token_type, Text);
        assert_eq!(tokens[2].lexeme, "url");
        assert_eq!(tokens[3].token_type, DoubleQuote);
        assert_eq!(tokens[4].token_type, Text);
        assert_eq!(tokens[4].lexeme, "api");
        assert_eq!(tokens[5].token_type, Colon);
        assert_eq!(tokens[6].token_type, Text);
        assert_eq!(tokens[6].lexeme, "index");
        assert_eq!(tokens[7].token_type, DoubleQuote);
        assert_eq!(tokens[8].token_type, PercentRightBrace);
        assert_eq!(tokens[9].token_type, SingleQuote);
    }
}
