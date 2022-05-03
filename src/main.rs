mod globals;
mod search;

use search::Vocabulary;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("to many arguments");
    }

    let repeat = 1_000_000;

    // ARRAY
    let fruits = Vec::from(globals::WORDS);

    let vector_time = Instant::now();

    for _ in 0..repeat {
        match args.get(1) {
            Some(arg) => &fruits
                .clone()
                .into_iter()
                .filter(|&el| el.starts_with(arg))
                .collect(),
            None => &fruits,
        };
    }

    println!(
        "ARRAY {}s",
        vector_time.elapsed().as_millis() as f32 / 1000.0
    );

    // THREE
    let mut fruits = Vocabulary::new();
    globals::WORDS.into_iter().for_each(|v| fruits.insert(v));

    let vocabulary_time = Instant::now();

    (0..repeat).into_iter().for_each(|_| {
        fruits.search(args.get(1).unwrap_or(&"".to_owned()));
    });

    println!(
        "THREE {}s",
        vocabulary_time.elapsed().as_millis() as f32 / 1000.0
    );

    let result = fruits.search(args.get(1).unwrap_or(&"".to_owned()));

    match result.is_empty() {
        true => println!("No results"),
        false => println!("{:#?}", result),
    }

    // println!("{:#?}", fruits.len());
}
