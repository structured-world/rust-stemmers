# Structured Stemmers

The structured-world fork of [rust-stemmers](https://github.com/CurrySoftware/rust-stemmers),
published as `structured-stemmers`. It includes Ukrainian stemming and updated
Snowball-generated implementations. It has no C/C++ build or runtime dependency.
The minimum supported Rust version is 1.92.

This crate implements some stemmer algorithms found in the [snowball project](http://snowballstem.org/) which are compiled to rust using the rust-backend of the [snowball compiler](https://github.com/snowballstem/snowball).

# Supported Algorithms

-   Arabic
-   Armenian
-   Danish
-   Dutch
-   English
-   Finnish
-   French
-   German
-   Greek
-   Hungarian
-   Italian
-   Norwegian
-   Portuguese
-   Romanian
-   Russian
-   Spanish
-   Swedish
-   Tamil
-   Turkish
-   Ukrainian


# Usage

```toml
[dependencies]
structured-stemmers = "1.2"
```

```rust
extern crate structured_stemmers;
use structured_stemmers::{Algorithm, Stemmer};

// Create a stemmer for the english language
let en_stemmer = Stemmer::create(Algorithm::English);

// Stemm the word "fruitlessly"
// Please be aware that all algorithms expect their input to only contain lowercase characters.
assert_eq!(en_stemmer.stem("fruitlessly"), "fruitless");
```

# Related Projects

-   The [stemmer](https://github.com/lise-henry/stemmer-rs) crate provides bindings to the C Snowball implementation.

# License

The Rust wrapper retains its upstream MIT license in `LICENSE`. Snowball-derived
code retains the BSD-3-Clause license in `LICENSE-SNOWBALL`; both notices ship in
the package. See `NOTICE` for provenance.

# Releases

CI checks Rust 1.92 and stable, runs every supplied vocabulary corpus, and
builds and runs an independent consumer of the prepared package archive.
Test corpora (including the separately licensed Arabic vocabulary) stay in
the source repository; they are not distributed in the library archive.

`release-plz` prepares release PRs. Merging one publishes through the `Release`
workflow using crates.io Trusted Publishing, bound to this repository,
`release.yml`, and the `crates-io` GitHub environment. Configure that environment
to accept only the `main` branch.

For the first publication of the new name, an owner runs this same workflow on
`main` with `bootstrap=true`. Its temporary `CRATES_IO_BOOTSTRAP_TOKEN` secret
must be restricted to `structured-stemmers`, with `publish-new`, `publish-update`
and `trusted-publishing` permissions and a short expiration. CI publishes using
release-plz and registers and verifies the Trusted Publisher. After success,
revoke the temporary token and delete the secret. Normal releases do not read it.

