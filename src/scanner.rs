use std::iter::Peekable;
use std::str::Chars;

enum TokenType {
    // Single-character tokens.
    TOKEN_LEFT_PAREN,
    TOKEN_RIGHT_PAREN,
    TOKEN_LEFT_BRACE,
    TOKEN_RIGHT_BRACE,
    TOKEN_COMMA,
    TOKEN_DOT,
    TOKEN_MINUS,
    TOKEN_PLUS,
    TOKEN_SEMICOLON,
    TOKEN_SLASH,
    TOKEN_STAR,
    // One or two character tokens.
    TOKEN_BANG,
    TOKEN_BANG_EQUAL,
    TOKEN_EQUAL,
    TOKEN_EQUAL_EQUAL,
    TOKEN_GREATER,
    TOKEN_GREATER_EQUAL,
    TOKEN_LESS,
    TOKEN_LESS_EQUAL,
    // Literals.
    TOKEN_IDENTIFIER,
    TOKEN_STRING,
    TOKEN_NUMBER,
    // Keywords.
    TOKEN_AND,
    TOKEN_CLASS,
    TOKEN_ELSE,
    TOKEN_FALSE,
    TOKEN_FOR,
    TOKEN_FUN,
    TOKEN_IF,
    TOKEN_NIL,
    TOKEN_OR,
    TOKEN_PRINT,
    TOKEN_RETURN,
    TOKEN_SUPER,
    TOKEN_THIS,
    TOKEN_TRUE,
    TOKEN_VAR,
    TOKEN_WHILE,

    TOKEN_ERROR,
    TOKEN_EOF,
}

struct Scanner<'a> {
    start: Peekable<Chars<'a>>,
    current: Peekable<Chars<'a>>,
    line: usize,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        Scanner {
            start: source.chars().peekable(),
            current: source.chars().peekable(),
            line: 1,
        }
    }

    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_at_end(&mut self) -> bool {
        self.current.peek().is_none()
    }

    fn advance(&mut self) -> char {
        self.current.next().unwrap_or('\0')
    }

    fn peek(&mut self) -> &char {
        self.current.peek().unwrap_or(&'\0')
    }

    fn peek_next(&mut self) -> char {
        let mut iter_clone = self.current.clone();
        iter_clone.next(); // Advance clone to next character
        *iter_clone.peek().unwrap_or(&'\0')
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => match self.peek_next() {
                    '/' => {
                        while *self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    }
                    '*' => {
                        self.advance();
                        self.advance();
                        loop {
                            if *self.peek() == '*' && self.peek_next() == '/' {
                                self.advance();
                                self.advance();
                                break;
                            } else if self.is_at_end() {
                                break;
                            } else {
                                self.advance();
                            }
                        }
                    }
                    _ => {
                        break;
                    }
                },
                _ => {
                    break;
                }
            }
        }
    }
}

#[test]
fn test_new_scanner() {
    let scanner = Scanner::new("test");
    assert_eq!(scanner.line, 1);
}

#[test]
fn test_is_alpha() {
    assert_eq!(Scanner::is_alpha('a'), true);
    assert_eq!(Scanner::is_alpha('A'), true);
    assert_eq!(Scanner::is_alpha('_'), true);
    assert_eq!(Scanner::is_alpha('1'), false);
}

#[test]
fn test_is_digit() {
    assert_eq!(Scanner::is_digit('1'), true);
    assert_eq!(Scanner::is_digit('a'), false);
}

#[test]
fn test_is_at_end() {
    let mut scanner = Scanner::new("test");
    assert_eq!(scanner.is_at_end(), false);
    for _ in 0..4 {
        scanner.advance();
    }
    assert_eq!(scanner.is_at_end(), true);
}

#[test]
fn test_peeks() {
    let mut scanner = Scanner::new("123");
    assert_eq!(*scanner.peek(), '1');
    assert_eq!(scanner.peek_next(), '2');
    scanner.advance();
    assert_eq!(*scanner.peek(), '2');
    assert_eq!(scanner.peek_next(), '3');
}

#[test]
fn test_skip_whitespace() {
    let mut scanner = Scanner::new("  // comment\n  /* comment */");
    scanner.skip_whitespace();
    assert_eq!(scanner.line, 2);
}
