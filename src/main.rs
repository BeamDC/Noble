use std::time::Instant;
use crate::message_gen::text_db::TextDB;

mod message_gen;

fn main() {
    let start = Instant::now();
    let db = TextDB::new("Hello, World!\n12.34".to_string());
    println!("Database built in: {:?}", start.elapsed());
    println!("{db:#?}");
}

