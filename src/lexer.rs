use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    LEFT_PAREN,          // (
    RIGHT_PAREN,         // )
    LEFT_BRACE,          // {
    RIGHT_BRACE,         // }
    LEFT_BRACKET,        // [
    RIGHT_BRACKET,       // ]
    LEFT_ANGLE,          // <
    RIGHT_ANGLE,         // >
    COMMA,               // ,
    DOT,                 // .
    MINUS,               // -
    PLUS,                // +
    COLON,               // :
    SEMICOLON,           // ;
    SLASH,               // /
    STAR,                // *
    BANG,                // !
    EQUAL,               // =
    PIPE,                // |
    PERCENT,             // %
    HASH,                // #
    SINGLE_QUOTE,        // '
    DOUBLE_QUOTE,        // "
    DOUBLE_LEFT_BRACE,   // {{
    DOUBLE_RIGHT_BRACE,  // }}
    LEFT_BRACE_PERCENT,  // {%
    PERCENT_RIGHT_BRACE, // %}
    LEFT_BRACE_HASH,     // {#
    HASH_RIGHT_BRACE,    // #}
    BANG_EQUAL,          // !=
    DOUBLE_EQUAL,        // ==
    LEFT_ANGLE_EQUAL,    // <=
    RIGHT_ANGLE_EQUAL,   // =>
    TEXT,
    EOF,
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
            '(' => self.add_token(LEFT_PAREN),
            ')' => self.add_token(RIGHT_PAREN),
            '[' => self.add_token(LEFT_BRACKET),
            ']' => self.add_token(RIGHT_BRACKET),
            ',' => self.add_token(COMMA),
            '.' => self.add_token(DOT),
            '-' => self.add_token(MINUS),
            '+' => self.add_token(PLUS),
            ':' => self.add_token(COLON),
            ';' => self.add_token(SEMICOLON),
            '*' => self.add_token(STAR),
            '|' => self.add_token(PIPE),
            '\'' => self.add_token(SINGLE_QUOTE),
            '"' => self.add_token(DOUBLE_QUOTE),
            '{' => {
                let token_type = if self.match_char('{') {
                    DOUBLE_LEFT_BRACE
                } else if self.match_char('%') {
                    LEFT_BRACE_PERCENT
                } else if self.match_char('#') {
                    LEFT_BRACE_HASH
                } else {
                    LEFT_BRACE
                };
                self.add_token(token_type);
            }
            '}' => {
                let token_type = if self.match_char('}') {
                    DOUBLE_RIGHT_BRACE
                } else {
                    RIGHT_BRACE
                };
                self.add_token(token_type);
            }
            '%' => {
                let token_type = if self.match_char('}') {
                    PERCENT_RIGHT_BRACE
                } else {
                    PERCENT
                };
                self.add_token(token_type);
            }
            '#' => {
                let token_type = if self.match_char('}') {
                    HASH_RIGHT_BRACE
                } else {
                    HASH
                };
                self.add_token(token_type);
            }
            '!' => {
                let token_type = if self.match_char('=') {
                    BANG_EQUAL
                } else {
                    BANG
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    DOUBLE_EQUAL
                } else {
                    EQUAL
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    LEFT_ANGLE_EQUAL
                } else {
                    LEFT_ANGLE
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    RIGHT_ANGLE_EQUAL
                } else {
                    RIGHT_ANGLE
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(SLASH);
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
            self.add_token_literal(TokenType::TEXT, Some(text));
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
        assert_eq!(tokens[0].token_type, LEFT_ANGLE);
        assert_eq!(tokens[1].token_type, TEXT);
        assert_eq!(tokens[1].lexeme, "html");
        assert_eq!(tokens[2].token_type, RIGHT_ANGLE);
    }

    #[test]
    fn test_closing_tag() {
        let tokens = tokenize("</body>");
        assert_eq!(tokens[0].token_type, LEFT_ANGLE);
        assert_eq!(tokens[1].token_type, SLASH);
        assert_eq!(tokens[2].token_type, TEXT);
        assert_eq!(tokens[2].lexeme, "body");
        assert_eq!(tokens[3].token_type, RIGHT_ANGLE);
    }

    #[test]
    fn test_html_attribute() {
        let tokens = tokenize(r#"<a href="link">"#);
        assert_eq!(tokens[0].token_type, LEFT_ANGLE);
        assert_eq!(tokens[1].token_type, TEXT);
        assert_eq!(tokens[1].lexeme, "a");
        assert_eq!(tokens[2].token_type, TEXT);
        assert_eq!(tokens[2].lexeme, "href");
        assert_eq!(tokens[3].token_type, EQUAL);
        assert_eq!(tokens[4].token_type, DOUBLE_QUOTE);
        assert_eq!(tokens[5].token_type, TEXT);
        assert_eq!(tokens[5].lexeme, "link");
        assert_eq!(tokens[6].token_type, DOUBLE_QUOTE);
        assert_eq!(tokens[7].token_type, RIGHT_ANGLE);
    }

    #[test]
    fn test_django_variable() {
        let tokens = tokenize("{{ variable }}");
        assert_eq!(tokens[0].token_type, DOUBLE_LEFT_BRACE);
        assert_eq!(tokens[1].token_type, TEXT);
        assert_eq!(tokens[1].lexeme, "variable");
        assert_eq!(tokens[2].token_type, DOUBLE_RIGHT_BRACE);
    }

    #[test]
    fn test_django_templatetag() {
        let tokens = tokenize("{% if condition %}");
        assert_eq!(tokens[0].token_type, LEFT_BRACE_PERCENT);
        assert_eq!(tokens[1].token_type, TEXT);
        assert_eq!(tokens[1].lexeme, "if");
        assert_eq!(tokens[2].token_type, TEXT);
        assert_eq!(tokens[2].lexeme, "condition");
        assert_eq!(tokens[3].token_type, PERCENT_RIGHT_BRACE);
    }

    #[test]
    fn test_django_comment() {
        let tokens = tokenize("{# This is a comment #}");
        assert_eq!(tokens[0].token_type, LEFT_BRACE_HASH);
        assert_eq!(tokens[1].token_type, TEXT);
        assert_eq!(tokens[1].lexeme, "This");
        assert_eq!(tokens[2].token_type, TEXT);
        assert_eq!(tokens[2].lexeme, "is");
        assert_eq!(tokens[3].token_type, TEXT);
        assert_eq!(tokens[3].lexeme, "a");
        assert_eq!(tokens[4].token_type, TEXT);
        assert_eq!(tokens[4].lexeme, "comment");
        assert_eq!(tokens[5].token_type, HASH_RIGHT_BRACE);
    }

    #[test]
    fn test_django_filter() {
        let tokens = tokenize("{{ value|default:'default' }}");
        assert_eq!(tokens[0].token_type, DOUBLE_LEFT_BRACE);
        assert_eq!(tokens[1].token_type, TEXT);
        assert_eq!(tokens[1].lexeme, "value");
        assert_eq!(tokens[2].token_type, PIPE);
        assert_eq!(tokens[3].token_type, TEXT);
        assert_eq!(tokens[3].lexeme, "default");
        assert_eq!(tokens[4].token_type, COLON);
        assert_eq!(tokens[5].token_type, SINGLE_QUOTE);
        assert_eq!(tokens[6].token_type, TEXT);
        assert_eq!(tokens[6].lexeme, "default");
        assert_eq!(tokens[7].token_type, SINGLE_QUOTE);
        assert_eq!(tokens[8].token_type, DOUBLE_RIGHT_BRACE);
    }

    #[test]
    fn test_quoted_django_templatetag() {
        let tokens = tokenize(r#"'{% url "api:index" %}'"#);
        assert_eq!(tokens[0].token_type, SINGLE_QUOTE);
        assert_eq!(tokens[1].token_type, LEFT_BRACE_PERCENT);
        assert_eq!(tokens[2].token_type, TEXT);
        assert_eq!(tokens[2].lexeme, "url");
        assert_eq!(tokens[3].token_type, DOUBLE_QUOTE);
        assert_eq!(tokens[4].token_type, TEXT);
        assert_eq!(tokens[4].lexeme, "api");
        assert_eq!(tokens[5].token_type, COLON);
        assert_eq!(tokens[6].token_type, TEXT);
        assert_eq!(tokens[6].lexeme, "index");
        assert_eq!(tokens[7].token_type, DOUBLE_QUOTE);
        assert_eq!(tokens[8].token_type, PERCENT_RIGHT_BRACE);
        assert_eq!(tokens[9].token_type, SINGLE_QUOTE);
    }
}
