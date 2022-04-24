mod Search;
mod globals;

use Search::Vocabulary;

fn main() {
    let mut fruits = Vocabulary::new();

    globals::FRUITS.into_iter().for_each(|v| fruits.insert(v));
    let result = fruits.search("Ap");

    println!("{:#?}", result);
}
