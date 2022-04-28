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

    let mut fruits = Vocabulary::new();
    globals::FRUITS.into_iter().for_each(|v| fruits.insert(v));

    let repeat = 100_000;

    // ARRAY
    let vector_time = Instant::now();

    (0..repeat).into_iter().for_each(|_| {
        let result: Vec<&str>;

        match args.get(1) {
            Some(arg) => match arg.eq(".") {
                true => result = Vec::from(globals::FRUITS),
                false => {
                    result = globals::FRUITS
                        .into_iter()
                        .filter(|&el| el.starts_with(arg))
                        .collect()
                }
            },
            None => result = Vec::from(globals::FRUITS),
        }

        // if result.is_empty() {
        //     println!("No results");
        // } else {
        //     println!("{:#?}", result);
        // }
    });

    println!("ARRAY {}", vector_time.elapsed().as_millis() as f32 / 1000.0);

    // THREE
    let vocabulary_time = Instant::now();

    (0..repeat).into_iter().for_each(|_| {
        let result: Vec<String>;

        match args.get(1) {
            Some(arg) => match arg.eq(".") {
                true => result = fruits.get_all(),
                false => result = fruits.search(arg),
            },
            None => result = fruits.get_all(),
        }

        // if result.is_empty() {
        //     println!("No results");
        // } else {
        //     println!("{:#?}", result);
        // }
    });

    println!("THREE {}", vocabulary_time.elapsed().as_millis() as f32 / 1000.0)
}
