use std::iter::Peekable;
use std::str::Chars;
use crate::{consume_while};

#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Token {
    Word(String),
    Number(String),
    Symbol(char),
    Punctuation(char),
    Whitespace(String),
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Word(s) => s.clone(),
            Token::Number(s) => s.clone(),
            Token::Symbol(c) => c.to_string(),
            Token::Punctuation(c) => c.to_string(),
            Token::Whitespace(s) => s.clone(),
        }
    }
}

pub struct TextTokenizer<'tt> {
    text: Peekable<Chars<'tt>>,
    start: usize,
    current: usize,
}

impl<'tt> TextTokenizer<'tt> {
    pub fn new(text: &'tt str) -> Self {
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
                    tokens.push(Token::Number(consume_while!(c, self, '0'..='9')));
                }
                'a'..='z' | 'A'..='Z' => {
                    tokens.push(Token::Word(
                        consume_while!(c, self, 'a'..='z' | 'A'..='Z' | '_' | '-' | '\'')
                    ));
                }
                ' ' | '\t' | '\n' | '\r' => {
                    tokens.push(Token::Whitespace(
                        consume_while!(c, self, ' ' | '\t' | '\n' | '\r')
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