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

## Reproducing the algorithms

The algorithms and Rust runtime are generated from Snowball **main** at
[`6389c79f`](https://github.com/snowballstem/snowball/commit/6389c79f91cd67ed2de3ebdcf84ec8c2d2d23c11).
This includes the Ukrainian algorithm merged upstream in
[PR #277](https://github.com/snowballstem/snowball/pull/277), which is absent from
the `v3.1.1` tag. The source identifies itself as Snowball 3.2.0; the exact commit,
not a moving branch or a presumed release, defines this package's behavior.

`snowball.lock.json` pins the source and official `snowball-data` commits.
Maintainers run `python3 -B scripts/snowball.py regenerate` after deliberately
updating these pins. The tool uses the upstream compiler and copies its matching
Rust runtime. It regenerates the small local regression outputs with Snowball's
independent Python backend. `python3 -B scripts/snowball.py check` checks exact
reproduction and all 20 languages against the pinned official reference corpus,
including input/output lengths. Both commands accept `--source-repository` and
`--data-repository` Git object caches; uncommitted donor files are never used.

Regeneration requires Python 3.11.8+ (safe tar extraction), Git, GNU Make and a C99
compiler. These are **maintainer tools only**: installing or building the crate
requires Rust, with no Snowball generator, C/C++ compiler or network codegen.
The full upstream reference data is downloaded into a temporary directory and
removed after checking; its component licenses stay with that test data.

**Index upgrades:** stemming output changes between algorithm revisions. Rebuild
persisted full-text indexes and encrypted stem tokens under the new revision;
do not query old postings/tokens with a new analyzer or mix revisions within a
generation. A database must bind analyzer identity to its index generation and
preserve that interpretation for retained reads until replacement is ready.

## Publication

CI checks Rust 1.92 and stable, verifies reproducible generation and the official
corpora for every supported language, runs the local regression corpora, and
builds and runs an independent consumer of the prepared package archive.
Test corpora (including the separately licensed Arabic vocabulary) stay in
the source repository; they are not distributed in the library archive.

`release-plz` prepares release PRs. Merging one publishes through the `Release`
workflow using crates.io Trusted Publishing, bound to this repository,
`release.yml`, and the `crates-io` GitHub environment. Configure that environment
to accept only the `main` branch.

An owner configures the Trusted Publisher in the crate's settings on crates.io.
The workflow only consumes that OIDC binding: it does not register publishers
or read a stored registry token. For a new crate name, publish the first
qualified archive outside CI, configure its Trusted Publisher, and retry the
release job. Normal releases need neither a bootstrap mode nor a registry-token
secret.
