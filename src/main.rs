mod globals;
mod search;

use search::Vocabulary;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("to many arguments");
    }

    let mut fruits = Vocabulary::new();
    globals::FRUITS.into_iter().for_each(|v| fruits.insert(v));

    let result: Vec<String>;

    match args.get(1) {
        Some(arg) => {
            if *arg == ".".to_owned() {
                result = fruits.get_all();
            } else {
                result = fruits.search(arg);
            }
        },
        None => result = fruits.get_all()
    }

    if result.is_empty() {
        println!("No results");
    } else {
        println!("{:#?}", result);
    }

}
