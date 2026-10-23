//! Source-development corpus driver; reference data is never packaged.
extern crate structured_stemmers;

use std::io::{self, BufRead, Write};
use structured_stemmers::{Algorithm, Stemmer};

fn main() -> io::Result<()> {
    let language = std::env::args().nth(1).expect("language is required");
    let algorithm = match language.as_str() {
        "arabic" => Algorithm::Arabic,
        "armenian" => Algorithm::Armenian,
        "danish" => Algorithm::Danish,
        "dutch" => Algorithm::Dutch,
        "english" => Algorithm::English,
        "finnish" => Algorithm::Finnish,
        "french" => Algorithm::French,
        "german" => Algorithm::German,
        "greek" => Algorithm::Greek,
        "hungarian" => Algorithm::Hungarian,
        "italian" => Algorithm::Italian,
        "norwegian" => Algorithm::Norwegian,
        "portuguese" => Algorithm::Portuguese,
        "romanian" => Algorithm::Romanian,
        "russian" => Algorithm::Russian,
        "spanish" => Algorithm::Spanish,
        "swedish" => Algorithm::Swedish,
        "tamil" => Algorithm::Tamil,
        "turkish" => Algorithm::Turkish,
        "ukrainian" => Algorithm::Ukrainian,
        _ => panic!("unknown corpus language: {}", language),
    };
    let stemmer = Stemmer::create(algorithm);
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    for word in stdin.lock().lines() {
        writeln!(stdout, "{}", stemmer.stem(&word?))?;
    }
    stdout.flush()
}
