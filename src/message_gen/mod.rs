use crate::message_gen::text_db::TextDB;

pub(crate) mod text_db;
mod tokenizer;
mod util;

pub enum MessageGenError {
    TokenizationError(String),
    DatabaseError(String),
}

pub struct MessageGenerator {
    pub db: TextDB,
}