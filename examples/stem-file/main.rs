extern crate structured_stemmers;

use structured_stemmers::{Algorithm, Stemmer};

fn main() {
    let en_stemmer = Stemmer::create(Algorithm::English);
    assert_eq!(en_stemmer.stem("fruitlessly"), "fruitless");    
}
