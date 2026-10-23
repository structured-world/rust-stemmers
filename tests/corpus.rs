// Source-only regression corpora: every supplied vocabulary/result pair must
// match the independent Snowball Python backend at our pinned revision.
extern crate structured_stemmers;
use structured_stemmers::{Algorithm, Stemmer};

fn check_corpus(code: &str, algorithm: Algorithm) {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let mut words =
        BufReader::new(File::open(format!("test_data/voc_{}.txt", code)).unwrap()).lines();
    let mut expected =
        BufReader::new(File::open(format!("test_data/res_{}.txt", code)).unwrap()).lines();
    let stemmer = Stemmer::create(algorithm);
    let mut count = 0;
    loop {
        match (words.next(), expected.next()) {
            (Some(word), Some(stem)) => {
                let word = word.unwrap();
                assert_eq!(stemmer.stem(&word), stem.unwrap(), "{}: {}", code, word);
                count += 1;
            }
            (None, None) => break,
            _ => panic!(
                "{} corpus has unequal input/output lengths at row {}",
                code,
                count + 1
            ),
        }
    }
    // A truncated or empty file must never turn the release gate green.
    assert!(count > 0, "{} corpus is empty", code);
}

// Each language remains independently reported by nextest; the shared helper
// enforces complete coverage and identifies the offending word on failure.
#[test]
fn german_test() {
    check_corpus("ger", Algorithm::German);
}
#[test]
fn english_test() {
    check_corpus("en", Algorithm::English);
}
#[test]
fn french_test() {
    check_corpus("fr", Algorithm::French);
}
#[test]
fn spanish_test() {
    check_corpus("es", Algorithm::Spanish);
}
#[test]
fn portuguese_test() {
    check_corpus("pt", Algorithm::Portuguese);
}
#[test]
fn italian_test() {
    check_corpus("it", Algorithm::Italian);
}
#[test]
fn romanian_test() {
    check_corpus("ro", Algorithm::Romanian);
}
#[test]
fn russian_test() {
    check_corpus("ru", Algorithm::Russian);
}
#[test]
fn arabic_test() {
    check_corpus("ar", Algorithm::Arabic);
}
#[test]
fn finnish_test() {
    check_corpus("fi", Algorithm::Finnish);
}
#[test]
fn greek_test() {
    check_corpus("el", Algorithm::Greek);
}
#[test]
fn ukrainian_test() {
    check_corpus("uk", Algorithm::Ukrainian);
}
#[test]
fn norwegian_test() {
    check_corpus("no", Algorithm::Norwegian);
}

// Official Ukrainian main-corpus cases which the previous generated algorithm
// mishandled: short words, adjective stripping, and derivational noun suffixes.
#[test]
fn ukrainian_upstream_main_regressions() {
    let stemmer = Stemmer::create(Algorithm::Ukrainian);
    for &(word, expected) in &[
        ("аа", "аа"),
        ("абаксіально", "абаксіал"),
        ("абатства", "абат"),
    ] {
        assert_eq!(stemmer.stem(word), expected, "{}", word);
    }
}
