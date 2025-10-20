use crate::message_gen::text_db::TextDB;
use crate::message_gen::tokenizer::Token;
use rand::prelude::IteratorRandom;
use std::path::Path;

pub mod text_db;
pub mod tokenizer;
mod util;

#[derive(Debug)]
pub enum MessageGenError {
    TokenizationError(String),
    DatabaseError(String),
}

pub struct MessageGenerator {
    /// the database containing the reference text
    db: TextDB,
    /// the number of tokens used to compose a message
    message_len: usize,
}

impl MessageGenerator {
    /// a context size of 2 words
    pub const LOW_PRECISION:  usize = 2;
    /// a context size of 4 words
    pub const MED_PRECISION:  usize = 4;
    /// a context size of 8 words
    pub const HIGH_PRECISION: usize = 8;

    /// construct a new [`MessageGenerator`] from a given
    /// [`TextDB`], context size, and message length.
    #[must_use]
    pub fn new(db: TextDB, context: usize, message_len: usize) -> Self {
        // todo : some data may not be able to have a context size as large what what is specified,
        // in this case we will either need to clamp it to a lower value, or return an error.
        Self {
            db,
            message_len,
        }
    }

    /// constructs a new [`MessageGenerator`] from a [`String`].
    #[must_use]
    pub fn from_string(string: String, context: usize, message_len: usize) -> Self {
        let db = TextDB::new(string, context);
        Self::new(db, context, message_len)
    }

    /// constructs a new [`MessageGenerator`] from a file path.
    pub fn from_file<P: AsRef<Path>>(path: P, context: usize, message_len: usize) -> Result<Self, MessageGenError> {
        let db = TextDB::from_file(path, context)?;
        Ok(Self::new(db, context, message_len))
    }

    pub fn update<S: ToString>(&mut self, new: S, context_size: Option<usize>, message_len: Option<usize>) {
        if let Some(context_size) = context_size {
            self.db.context_size = context_size;
        }
        if let Some(message_len) = message_len {
            self.message_len = message_len;
        }

        self.db.update(new, self.db.context_size);
    }

    fn generate_message(db: &TextDB, len: usize) -> String {
        let mut rng = rand::thread_rng();
            let mut tokens = Vec::with_capacity(len);

            // choose random start
            let mut seed = db.context
                .keys()
                .choose(&mut rng)
                .unwrap()
                .to_owned();

            tokens.extend(seed.clone());

            // generate the rest of the message
            for _ in 0..len.saturating_sub(db.context_size) {
                let next: Token = match db.context.get(&seed) {
                    Some(toks) => {
                        toks.into_iter()
                            .choose(&mut rng)
                            .unwrap()
                            .to_owned()
                    }
                    None => break,
                };
                tokens.push(next.clone());

                seed.remove(0);
                seed.push(next);
            }

        tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<String>()
    }

    /// returns the next message in the stream,
    /// if the stream has been fully used, a new one will be generated.
    pub fn next_message(&self) -> String {
        Self::generate_message(&self.db, self.message_len)
    }
}