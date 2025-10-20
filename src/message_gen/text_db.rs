use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use crate::message_gen::MessageGenError;
use crate::message_gen::tokenizer::{TextTokenizer, Token};

#[derive(Debug, Clone)]
pub struct TextDB {
    source: Arc<str>,
    tokens: Arc<[Token]>,
    pub context_size: usize,
    pub context: Arc<HashMap<Vec<Token>, Vec<Token>>>,
}

impl TextDB {
    /// construct a new [`TextDB`] from a string
    pub fn new(src: String, context_size: usize) -> Self {
        let tokens: Arc<[Token]> = Arc::from(
            TextTokenizer::new(&src).tokenize()
        );

        let mut context_map: HashMap<Vec<Token>, Vec<Token>> = HashMap::new();
        let mut keys: Vec<Arc<[Token]>> = vec![];
        let mut values: Vec<Token> = vec![];


        for (i, w) in tokens.windows(context_size).enumerate() {
            if let Some(next_token) = tokens.get(i + context_size) {
                keys.push(Arc::from(w.to_vec()));
                values.push(next_token.clone());
            }
        }

        for (k, v) in keys.into_iter().zip(values.into_iter()) {
            context_map
                .entry((*k).to_vec())
                .or_insert_with(|| vec![])
                .push(v.clone());
        }

        Self {
            source: Arc::from(src.as_str()),
            tokens,
            context_size,
            context: Arc::new(context_map),
        }
    }

    /// add `new` to the source string of this database,
    /// and set the context size to `context_size`.
    pub fn update<S: ToString>(&mut self, new: S, context_size: usize) {
        let source = self.source.to_string() + new.to_string().as_str();
        let db = Self::new(source, context_size);
        self.source = db.source;
        self.tokens = db.tokens;
        self.context_size = db.context_size;
        self.context = db.context;
    }

    /// construct a new [`TextDB`] from the contents of a file
    pub fn from_file<P: AsRef<Path>>(path: P, context_size: usize) -> Result<Self, MessageGenError> {
        use MessageGenError as E;
        let src: String = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => return Err(E::DatabaseError(e.to_string()))
        };
        Ok(Self::new(src, context_size))
    }
}