use std::str::Chars;

pub struct Cursor<'a> {
    chars: Chars<'a>,
    prev: char
}

impl <'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor { chars: input.chars(), prev: '\0' }
    }

    pub fn bump(&mut self) -> Option<char> {
        let res = self.chars.next()?;
        self.prev = res;
        Some(res)
    }

    pub fn is_eof(&mut self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub fn first(&mut self) -> char {
        self.chars.clone().next().unwrap_or('\0')
    }

    pub fn second(&mut self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or('\0')
    }

    pub fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }
}