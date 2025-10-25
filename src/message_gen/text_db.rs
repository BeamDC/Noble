use crate::message_gen::tokenizer::{TextTokenizer, Token};
use crate::message_gen::MessageGenError;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TextDB {
    source: Arc<str>,
    pub context_size: usize,
}

impl TextDB {
    /// the maximum allowed length of a source string
    const LENGTH_LIMIT: usize = 2<<18; // 512KiB

    /// construct a new [`TextDB`] from a string
    pub fn new(src: String, context_size: usize) -> Self {
        let src = Self::validate_string(src);

        Self {
            source: Arc::from(src.as_str()),
            context_size,
        }
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

    /// ensures that a string does not take up too much memory,
    /// if it does, the string is truncated to the size limit
    fn validate_string(s: String) -> String {
        let cut = match s.len() {
            0..Self::LENGTH_LIMIT => 0,
            _ => {
                let start = s.len() - Self::LENGTH_LIMIT - 1;
                s[start..]
                    .find(|c| char::is_whitespace(c))
                    .unwrap_or(start)
            }
        };

        s[cut..].to_owned()
    }

    /// add `new` to the source string of this database,
    /// and set the context size to `context_size`.
    pub fn update<S: ToString>(&mut self, new: S, context_size: usize) {
        let source = self.source.to_string() + new.to_string().as_str();
        let source = Self::validate_string(source);

        self.source = Arc::from(source);
        self.context_size = context_size;
    }

    #[inline(always)]
    pub fn get_tokens(&self) -> Vec<Token> {
        TextTokenizer::new(&self.source).tokenize()
    }

    pub fn get_context_map(&self) -> HashMap<Vec<Token>, Vec<Token>> {
        let tokens = self.get_tokens();
        let mut context_map: HashMap<Vec<Token>, Vec<Token>> = HashMap::new();
        let mut keys: Vec<&[Token]> = vec![];
        let mut values: Vec<Token> = vec![];

        for (i, w) in tokens.windows(self.context_size).enumerate() {
            if let Some(next_token) = tokens.get(i + self.context_size) {
                keys.push(w.iter().as_slice());
                values.push(next_token.clone());
            }
        }

        for (k, v) in keys.into_iter().zip(values.into_iter()) {
            context_map
                .entry((*k).to_vec())
                .or_insert_with(|| vec![])
                .push(v.clone());
        };

        context_map
    }
}