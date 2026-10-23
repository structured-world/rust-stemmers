"""Verify the delivered archive and exercise it from an independent consumer."""
import json
import pathlib
import subprocess
import tarfile
import tempfile


def run(*args, cwd=None):
    subprocess.run(args, cwd=cwd, check=True)


metadata = json.loads(subprocess.check_output(
    ["cargo", "metadata", "--no-deps", "--format-version=1"]))
package = metadata["packages"][0]
assert package["name"] == "structured-stemmers"
version = package["version"]
run("cargo", "package", "--allow-dirty")
archive = pathlib.Path(metadata["target_directory"]) / "package" / f"structured-stemmers-{version}.crate"
with tempfile.TemporaryDirectory(prefix="stemmers-consumer-") as tmp:
    root = pathlib.Path(tmp)
    with tarfile.open(archive) as tar:
        members = {pathlib.PurePosixPath(x.name).parts[1:] for x in tar.getmembers()}
        for required in ("LICENSE", "LICENSE-SNOWBALL", "NOTICE"):
            assert (required,) in members, required
        assert not any(p and p[0] == "test_data" for p in members)
        tar.extractall(root, filter="data")
    crate = root / f"structured-stemmers-{version}"
    consumer = root / "consumer"
    (consumer / "src").mkdir(parents=True)
    (consumer / "Cargo.toml").write_text(
        '[package]\nname="package-consumer"\nversion="0.0.0"\nedition="2021"\n'
        '[dependencies]\nstructured-stemmers={path=' + json.dumps(str(crate)) + '}\n')
    (consumer / "src/main.rs").write_text('''use structured_stemmers::{Algorithm, Stemmer};
fn main() {
    assert_eq!(Stemmer::create(Algorithm::English).stem("fruitlessly"), "fruitless");
    assert_eq!(Stemmer::create(Algorithm::Ukrainian).stem("книга"), "книг");
}
''')
    run("cargo", "run", "--manifest-path", str(consumer / "Cargo.toml"), cwd=consumer)
print("Prepared archive and independent consumer passed")
