// Source-only regression corpora: every supplied vocabulary/result pair must
// preserve the upstream stemming result, including the Ukrainian additions.
extern crate structured_stemmers;
use structured_stemmers::{Stemmer, Algorithm};

fn stemms_to(lhs: &str, rhs: &str, stemmer: Algorithm) {
    assert_eq!(Stemmer::create(stemmer).stem(lhs), rhs);
}

#[test]
fn german_test() {
    use std::fs;
    use std::io;
    use std::io::BufRead;

    let vocab = io::BufReader::new(fs::File::open("test_data/voc_ger.txt").unwrap());
    let result = io::BufReader::new(fs::File::open("test_data/res_ger.txt").unwrap());

    let lines = vocab.lines().zip(result.lines());

    for (voc, res) in lines {
        stemms_to(voc.unwrap().as_str(),
                  res.unwrap().as_str(),
                  Algorithm::German);
    }
}

#[test]
fn english_test() {
    use std::fs;
    use std::io;
    use std::io::BufRead;

    let vocab = io::BufReader::new(fs::File::open("test_data/voc_en.txt").unwrap());
    let result = io::BufReader::new(fs::File::open("test_data/res_en.txt").unwrap());

    let lines = vocab.lines().zip(result.lines());

    for (voc, res) in lines {
        stemms_to(voc.unwrap().as_str(),
                  res.unwrap().as_str(),
                  Algorithm::English);
    }
}

#[test]
fn french_test() {
    use std::fs;
    use std::io;
    use std::io::BufRead;

    let vocab = io::BufReader::new(fs::File::open("test_data/voc_fr.txt").unwrap());
    let result = io::BufReader::new(fs::File::open("test_data/res_fr.txt").unwrap());

    let lines = vocab.lines().zip(result.lines());

    for (voc, res) in lines {
        stemms_to(voc.unwrap().as_str(),
                  res.unwrap().as_str(),
                  Algorithm::French);
    }
}

#[test]
fn spanish_test() {
    use std::fs;
    use std::io;
    use std::io::BufRead;

    let vocab = io::BufReader::new(fs::File::open("test_data/voc_es.txt").unwrap());
    let result = io::BufReader::new(fs::File::open("test_data/res_es.txt").unwrap());

    let lines = vocab.lines().zip(result.lines());

    for (voc, res) in lines {
        stemms_to(voc.unwrap().as_str(),
                  res.unwrap().as_str(),
                  Algorithm::Spanish);
    }
}

#[test]
fn portuguese_test() {
    use std::fs;
    use std::io;
    use std::io::BufRead;

    let vocab = io::BufReader::new(fs::File::open("test_data/voc_pt.txt").unwrap());
    let result = io::BufReader::new(fs::File::open("test_data/res_pt.txt").unwrap());

    let lines = vocab.lines().zip(result.lines());

    for (voc, res) in lines {
        stemms_to(voc.unwrap().as_str(),
                  res.unwrap().as_str(),
                  Algorithm::Portuguese);
    }
}

#[test]
fn italian_test() {
    use std::fs;
    use std::io;
    use std::io::BufRead;

    let vocab = io::BufReader::new(fs::File::open("test_data/voc_it.txt").unwrap());
    let result = io::BufReader::new(fs::File::open("test_data/res_it.txt").unwrap());

    let lines = vocab.lines().zip(result.lines());

    for (voc, res) in lines {
        stemms_to(voc.unwrap().as_str(),
                  res.unwrap().as_str(),
                  Algorithm::Italian);
    }
}

#[test]
fn romanian_test() {
    use std::fs;
    use std::io;
    use std::io::BufRead;

    let vocab = io::BufReader::new(fs::File::open("test_data/voc_ro.txt").unwrap());
    let result = io::BufReader::new(fs::File::open("test_data/res_ro.txt").unwrap());

    let lines = vocab.lines().zip(result.lines());

    for (voc, res) in lines {
        stemms_to(voc.unwrap().as_str(),
                  res.unwrap().as_str(),
                  Algorithm::Romanian);
    }
}

#[test]
fn russian_test() {
    use std::fs;
    use std::io;
    use std::io::BufRead;

    let vocab = io::BufReader::new(fs::File::open("test_data/voc_ru.txt").unwrap());
    let result = io::BufReader::new(fs::File::open("test_data/res_ru.txt").unwrap());

    let lines = vocab.lines().zip(result.lines());

    for (voc, res) in lines {
        stemms_to(voc.unwrap().as_str(),
                  res.unwrap().as_str(),
                  Algorithm::Russian);
    }
}

#[test]
fn arabic_test() {
    use std::fs;
    use std::io;
    use std::io::BufRead;

    let vocab = io::BufReader::new(fs::File::open("test_data/voc_ar.txt").unwrap());
    let result = io::BufReader::new(fs::File::open("test_data/res_ar.txt").unwrap());

    let lines = vocab.lines().zip(result.lines());

    for (voc, res) in lines {
        stemms_to(voc.unwrap().as_str(),
                  res.unwrap().as_str(),
                  Algorithm::Arabic);
    }
}

#[test]
fn finnish_test() {
    use std::fs;
    use std::io;
    use std::io::BufRead;

    let vocab = io::BufReader::new(fs::File::open("test_data/voc_fi.txt").unwrap());
    let result = io::BufReader::new(fs::File::open("test_data/res_fi.txt").unwrap());

    let lines = vocab.lines().zip(result.lines());

    for (voc, res) in lines {
        stemms_to(voc.unwrap().as_str(),
                  res.unwrap().as_str(),
                  Algorithm::Finnish);
    }
}

#[test]
fn greek_test() {
    use std::fs;
    use std::io;
    use std::io::BufRead;

    let vocab = io::BufReader::new(fs::File::open("test_data/voc_el.txt").unwrap());
    let result = io::BufReader::new(fs::File::open("test_data/res_el.txt").unwrap());

    let lines = vocab.lines().zip(result.lines());

    for (voc, res) in lines {
        stemms_to(voc.unwrap().as_str(),
                  res.unwrap().as_str(),
                  Algorithm::Greek);
    }
}

#[test]
fn ukrainian_test() {
    use std::fs;
    use std::io;
    use std::io::BufRead;

    let vocab = io::BufReader::new(fs::File::open("test_data/voc_uk.txt").unwrap());
    let result = io::BufReader::new(fs::File::open("test_data/res_uk.txt").unwrap());

    let lines = vocab.lines().zip(result.lines());

    for (voc, res) in lines {
        stemms_to(voc.unwrap().as_str(),
                  res.unwrap().as_str(),
                  Algorithm::Ukrainian);
    }
}

#[test]
fn norwegian_test() {
    use std::fs;
    use std::io;
    use std::io::BufRead;

    let vocab = io::BufReader::new(fs::File::open("test_data/voc_no.txt").unwrap());
    let result = io::BufReader::new(fs::File::open("test_data/res_no.txt").unwrap());

    let lines = vocab.lines().zip(result.lines());

    for (voc, res) in lines {
        stemms_to(voc.unwrap().as_str(),
                  res.unwrap().as_str(),
                  Algorithm::Norwegian);
    }
}
