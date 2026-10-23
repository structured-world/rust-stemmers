//! This library provides rust implementations for some stemmer algorithms
//! written in the [snowball language](https://snowballstem.org/).
//!
//!
//! All algorithms expect the input to already be lowercased.
//!
//! # Usage
//! ```toml
//! [dependencies]
//! structured-stemmers = "1.2"
//! ```
//!
//! ```rust
//! extern crate structured_stemmers;
//!
//! use structured_stemmers::{Algorithm, Stemmer};
//!
//! fn main() {
//!    let en_stemmer = Stemmer::create(Algorithm::English);
//!    assert_eq!(en_stemmer.stem("fruitlessly"), "fruitless");
//! }
//! ```
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::borrow::Cow;

mod snowball;

use snowball::SnowballEnv;
use snowball::algorithms;

/// Enum of all supported algorithms.
/// Check the [Snowball-Website](https://snowballstem.org/) for details.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum Algorithm {
    Arabic,
    Armenian,
    Danish,
    Dutch,
    English,
    Finnish,
    French,
    German,
    Greek,
    Hungarian,
    Italian,
    Norwegian,
    Portuguese,
    Romanian,
    Russian,
    Spanish,
    Swedish,
    Tamil,
    Turkish,
    Ukrainian,
}

/// Wrapps a usable interface around the actual stemmer implementation
pub struct Stemmer {
    stemmer: fn(&mut SnowballEnv) -> bool,
}

impl Stemmer {
    /// Create a new stemmer from an algorithm
    pub fn create(lang: Algorithm) -> Self {
        match lang {
            Algorithm::Arabic => Stemmer { stemmer: algorithms::arabic::stem },
            Algorithm::Armenian => Stemmer { stemmer: algorithms::armenian::stem },
            Algorithm::Danish => Stemmer { stemmer: algorithms::danish::stem },
            Algorithm::Dutch => Stemmer { stemmer: algorithms::dutch::stem },
            Algorithm::English => Stemmer { stemmer: algorithms::english::stem },
            Algorithm::Finnish => Stemmer { stemmer: algorithms::finnish::stem },
            Algorithm::French => Stemmer { stemmer: algorithms::french::stem },
            Algorithm::German => Stemmer { stemmer: algorithms::german::stem },
            Algorithm::Greek => Stemmer { stemmer: algorithms::greek::stem },
            Algorithm::Hungarian => Stemmer { stemmer: algorithms::hungarian::stem },
            Algorithm::Italian => Stemmer { stemmer: algorithms::italian::stem },
            Algorithm::Norwegian => Stemmer { stemmer: algorithms::norwegian::stem },
            Algorithm::Portuguese => Stemmer { stemmer: algorithms::portuguese::stem },
            Algorithm::Romanian => Stemmer { stemmer: algorithms::romanian::stem },
            Algorithm::Russian => Stemmer { stemmer: algorithms::russian::stem },
            Algorithm::Spanish => Stemmer { stemmer: algorithms::spanish::stem },
            Algorithm::Swedish => Stemmer { stemmer: algorithms::swedish::stem },
            Algorithm::Tamil => Stemmer { stemmer: algorithms::tamil::stem },
            Algorithm::Turkish => Stemmer { stemmer: algorithms::turkish::stem },
            Algorithm::Ukrainian => Stemmer { stemmer: algorithms::ukrainian::stem },
        }
    }

    /// Stem a single word
    /// Please note, that the input is expected to be all lowercase (if that is applicable).
    pub fn stem<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let mut env = SnowballEnv::create(input);
        (self.stemmer)(&mut env);
        env.get_current()
    }
}
