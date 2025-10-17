use std::fs;
use std::path::Path;
use std::rc::Rc;
use crate::message_gen::MessageGenError;
use crate::message_gen::tokenizer::{TextTokenizer, Token};

#[derive(Debug, Clone)]
pub struct TextDB {
    pub tokens: Rc<[Token]>
}

impl TextDB {
    /// construct a new [`TextDB`] from a string
    pub fn new(src: String) -> Self {
        let tokens: Rc<[Token]> = Rc::from(
            TextTokenizer::new(&src).tokenize()
        );

        Self {
            tokens
        }
    }

    /// construct a new [`TextDB`] from the contents of a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, MessageGenError> {
        use MessageGenError as E;
        let src: String = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => return Err(E::DatabaseError(e.to_string()))
        };
        Ok(Self::new(src))
    }
}