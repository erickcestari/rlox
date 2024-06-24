use std::iter::Peekable;
use std::str::Chars;

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

    fn peekNext(&mut self) -> char {
        if (self.is_at_end()) {
            return '\0';
        }

        self.current.next().unwrap_or('\0')
    }
}
