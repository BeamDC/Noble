use std::iter::Peekable;
use std::str::Chars;
use crate::make_token;

#[derive(Debug, Clone)]
pub enum Token {
    Word(String),
    Number(String),
    Symbol(char),
    Punctuation(char),
    Whitespace(String),
}

pub struct TextTokenizer<'tl> {
    text: Peekable<Chars<'tl>>,
    start: usize,
    current: usize,
}

impl<'tl> TextTokenizer<'tl> {
    pub fn new(text: &'tl str) -> Self {
        Self {
            text: text.chars().peekable(),
            start: 0,
            current: 0,
        }
    }

    fn next(&mut self) -> Option<char> {
        self.current += 1;
        self.text.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.text.peek()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while let Some(c) = self.next() {
            match c {
                '0'..='9' => {
                    tokens.push(Token::Number(make_token!(c, self, '0'..='9')));
                }
                'a'..='z' | 'A'..='Z' => {
                    tokens.push(Token::Word(
                        make_token!(c, self, 'a'..='z' | 'A'..='Z')
                    ));
                }
                ' ' | '\t' | '\n' | '\r' => {
                    tokens.push(Token::Whitespace(
                        make_token!(c, self, ' ' | '\t' | '\n' | '\r')
                    ));
                }
                '.' | ',' | '!' | '?' | ';' | ':' => {
                    tokens.push(Token::Punctuation(c));
                }
                _ => {
                    tokens.push(Token::Symbol(c));
                }
            }
            self.start = self.current;
        }
        tokens
    }
}