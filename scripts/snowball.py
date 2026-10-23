#!/usr/bin/env python3
"""Reproduce Rust sources and verify independent, pinned Snowball corpora.

The C Snowball compiler runs only in this maintainer tool, never in a consumer
build or runtime. Python-generated stemmers provide an independent backend for
the small checked-in regression corpora; official data verifies all languages.
"""
import argparse
import gzip
import importlib.util
import itertools
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def run(*args, cwd=ROOT, **kwargs):
    return subprocess.run(args, cwd=cwd, check=True, **kwargs)


def snapshot(repo, revision, destination, local):
    """Export committed objects only; never modify or read a donor's worktree."""
    if local:
        repository = Path(local).resolve()
    else:
        repository = destination / (repo + "-git")
        run("git", "init", "--quiet", str(repository))
        run("git", "fetch", "--quiet", "--depth=1",
            f"https://github.com/snowballstem/{repo}.git", revision, cwd=repository)
    actual = run("git", "rev-parse", revision + "^{commit}", cwd=repository,
                 capture_output=True, text=True).stdout.strip()
    if actual != revision:
        raise RuntimeError(f"unexpected {repo} revision {actual}")
    archive = destination / (repo + ".tar")
    run("git", "archive", "--format=tar", "--output=" + str(archive), revision,
        cwd=repository)
    target = destination / repo
    target.mkdir()
    with tarfile.open(archive) as source:
        source.extractall(target, filter="data")
    return target


def generate(source, destination, languages):
    make = shutil.which("gmake") or "make"
    run(make, "-s", "snowball", cwd=source)
    compiler = str(source / "snowball")
    for language, local_corpus in languages.items():
        sbl = source / "algorithms" / (language + ".sbl")
        algorithm = destination / "algorithms" / sbl.name
        algorithm.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(sbl, algorithm)
        generated = destination / "src/snowball/algorithms" / language
        generated.parent.mkdir(parents=True, exist_ok=True)
        run(compiler, str(sbl), "-rust", "-o", str(generated))
        if local_corpus:
            # A different code generator is the oracle, never the Rust output
            # under test. These inputs are our existing regression vocabulary.
            python_module = source / "oracle" / language
            run(compiler, str(sbl), "-python", "-eprefix", "_", "-name", "OracleStemmer", "-o", str(python_module))
            spec = importlib.util.spec_from_file_location("oracle." + language, str(python_module) + ".py")
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            stemmer = module.OracleStemmer()
            expected = destination / "test_data" / ("res_" + local_corpus + ".txt")
            expected.parent.mkdir(exist_ok=True)
            with (ROOT / "test_data" / ("voc_" + local_corpus + ".txt")).open(encoding="utf-8") as words, expected.open("w", encoding="utf-8", newline="\n") as out:
                for word in words:
                    out.write(stemmer.stemWord(word.rstrip("\n")) + "\n")
    for name in ("among.rs", "snowball_env.rs"):
        shutil.copyfile(source / "rust/src/snowball" / name, destination / "src/snowball" / name)
    shutil.copyfile(source / "COPYING", destination / "LICENSE-SNOWBALL")


def reference_file(directory, name):
    path = directory / name
    if path.exists():
        return path
    compressed = path.with_suffix(path.suffix + ".gz")
    with gzip.open(compressed, "rb") as src, path.open("wb") as dst:
        shutil.copyfileobj(src, dst)
    return path


def corpus(data, languages):
    run("cargo", "build", "--example", "corpus")
    metadata = json.loads(run("cargo", "metadata", "--no-deps", "--format-version=1",
                              capture_output=True, text=True).stdout)
    executable = Path(metadata["target_directory"]) / "debug/examples/corpus"
    failed = False
    for language in languages:
        directory = data / language
        vocabulary = reference_file(directory, "voc.txt")
        reference = reference_file(directory, "output.txt")
        actual = directory / "rust-actual.txt"
        with vocabulary.open("rb") as words, actual.open("wb") as out:
            run(str(executable), language, stdin=words, stdout=out)
        mismatches = count = 0
        with vocabulary.open(encoding="utf-8") as words, reference.open(encoding="utf-8") as expected, actual.open(encoding="utf-8") as output:
            for count, (word, wanted, got) in enumerate(itertools.zip_longest(words, expected, output), 1):
                if word is None or wanted != got or wanted is None:
                    if mismatches < 3:
                        print(f"{language}:{count}: {word!r}: expected {wanted!r}, got {got!r}")
                    mismatches += 1
        if count == 0:
            raise RuntimeError(f"empty {language} corpus")
        print(f"{language}: {count} words, {mismatches} mismatches", flush=True)
        failed |= mismatches != 0
    if failed:
        raise RuntimeError("Rust disagrees with the pinned official Snowball corpus")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=("check", "regenerate", "corpus"))
    parser.add_argument("--source-repository", help="optional existing Git object cache")
    parser.add_argument("--data-repository", help="optional existing Git object cache")
    args = parser.parse_args()
    lock = json.loads((ROOT / "snowball.lock.json").read_text())
    with tempfile.TemporaryDirectory(prefix="stemmers-upstream-") as temporary:
        work = Path(temporary)
        if args.mode != "corpus":
            source = snapshot("snowball", lock["source"], work, args.source_repository)
            # Generated Python imports its sibling runtime relatively.
            (source / "oracle").mkdir()
            (source / "oracle/__init__.py").touch()
            shutil.copyfile(source / "python/snowballstemmer/basestemmer.py", source / "oracle/basestemmer.py")
            shutil.copyfile(source / "python/snowballstemmer/among.py", source / "oracle/among.py")
            sys.path.insert(0, str(source))
            generated = work / "generated"
            generated.mkdir()
            generate(source, generated, lock["algorithms"])
            changed = []
            for path in sorted(generated.rglob("*")):
                if not path.is_file():
                    continue
                relative = path.relative_to(generated)
                target = ROOT / relative
                if not target.exists() or path.read_bytes() != target.read_bytes():
                    changed.append(str(relative))
                    if args.mode == "regenerate":
                        target.parent.mkdir(parents=True, exist_ok=True)
                        shutil.copyfile(path, target)
            if changed:
                print("Changed generated files:\n" + "\n".join(changed))
                if args.mode == "check":
                    raise RuntimeError("sources do not reproduce from snowball.lock.json")
        if args.mode != "regenerate":
            data = snapshot("snowball-data", lock["data"], work, args.data_repository)
            corpus(data, lock["algorithms"])


if __name__ == "__main__":
    main()
