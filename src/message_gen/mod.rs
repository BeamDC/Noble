use std::path::Path;
use std::rc::Rc;
use rand::prelude::IteratorRandom;
use crate::message_gen::text_db::TextDB;
use crate::message_gen::tokenizer::Token;

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
    /// how many tokens of context are given when generating the next token
    context: usize,
    stream: Rc<[String]>,
    stream_size: usize,
    stream_pos: usize,
}

impl MessageGenerator {
    const MAX_STREAM_SIZE:  usize = 256;
    pub const LOW_PRECISION:  usize = 2;
    pub const MED_PRECISION:  usize = 4;
    pub const HIGH_PRECISION: usize = 8;

    /// construct a new [`MessageGenerator`] from a given [`TextDB`] and context size
    #[must_use]
    pub fn new(db: TextDB, context: usize, message_len: usize) -> Self {
        let stream = Self::generate_stream(&db.clone(), message_len);
        let stream_size = stream.len();

        Self {
            db,
            context,
            stream,
            stream_size,
            stream_pos: 0,
        }
    }

    #[must_use]
    pub fn from_string(string: String, context: usize, message_len: usize) -> Self {
        let db = TextDB::new(string, context);
        Self::new(db, context, message_len)
    }

    pub fn from_file<P: AsRef<Path>>(path: P, context: usize, message_len: usize) -> Result<Self, MessageGenError> {
        let db = TextDB::from_file(path, context)?;
        Ok(Self::new(db, context, message_len))
    }

    fn generate_stream(db: &TextDB, len: usize) -> Rc<[String]> {
        let mut rng = rand::thread_rng();
        let mut messages = Vec::with_capacity(Self::MAX_STREAM_SIZE);

        for _ in 0..Self::MAX_STREAM_SIZE {
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

            messages.push(
                tokens
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<String>()
            );
        }

        Rc::from(messages)
    }

    pub fn next_message(&mut self) -> String {
        let msg = self.stream[self.stream_pos].to_owned();
        self.stream_pos += 1;
        msg
    }
}