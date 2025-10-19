use std::time::Instant;
use crate::message_gen::MessageGenerator;

mod message_gen;

fn main() {
    let start = Instant::now();
    let mut gen = match MessageGenerator::from_file("src/test data/bible.txt", MessageGenerator::LOW_PRECISION, 150) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Error building message generator: {:?}", e);
            return;
        }
    };
    println!("Generator built in: {:?}", start.elapsed());
    for _i in 0..1 {
        let msg = gen.next_message();
        println!("{:#^10}\n{}", "", msg);
    }
    println!("{:#^10}", "");

    // for (context, next) in db.context.iter() {
    //     println!("Context: {:?} => Next: {:?}", context, next);
    // }
    // println!("{db:?}");
}

