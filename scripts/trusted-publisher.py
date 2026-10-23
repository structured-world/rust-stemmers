"""Idempotently register this release workflow after the first CI publication.

API contract: rust-lang/crates.io, src/controllers/trustpub/github_configs.
The temporary token needs publish-new and trusted-publishing
scopes for structured-stemmers only. No token or response body is logged.
"""
import json
import os
import urllib.error
import urllib.parse
import urllib.request


def configure(request):
    expected = {
        "crate": "structured-stemmers",
        "repository_owner": "structured-world",
        "repository_name": "rust-stemmers",
        "workflow_filename": "release.yml",
        "environment": "crates-io",
    }
    query = "?" + urllib.parse.urlencode({"crate": expected["crate"]})

    def exists():
        configs = request("GET", query)["github_configs"]
        return any(all(config.get(k) == v for k, v in expected.items()) for config in configs)

    if not exists():
        # On an uncertain response, fail. A retry first reads the persisted
        # configuration, so it cannot blindly create a second publisher.
        request("POST", "", {"github_config": expected})
    if not exists():
        raise RuntimeError("Trusted Publisher registration was not verified")


def main():
    if os.environ.get("GITHUB_REPOSITORY") != "structured-world/rust-stemmers":
        raise RuntimeError("Unexpected repository")
    if os.environ.get("GITHUB_REF") != "refs/heads/main":
        raise RuntimeError("Trusted Publisher setup is restricted to main")
    token = os.environ["CARGO_REGISTRY_TOKEN"]
    if not token:
        raise RuntimeError("Missing bootstrap token")

    def request(method, query, body=None):
        req = urllib.request.Request(
            "https://crates.io/api/v1/trusted_publishing/github_configs" + query,
            data=None if body is None else json.dumps(body).encode(),
            headers={"Authorization": token, "Content-Type": "application/json",
                     "User-Agent": "structured-stemmers-release (github.com/structured-world/rust-stemmers)"},
            method=method,
        )
        try:
            with urllib.request.urlopen(req, timeout=30) as response:
                return json.load(response)
        except urllib.error.HTTPError as error:
            raise RuntimeError(f"Trusted Publisher API returned HTTP {error.code}") from None
        except urllib.error.URLError:
            raise RuntimeError("Trusted Publisher API unavailable; reconcile by rerunning setup") from None

    configure(request)
    print("Verified Trusted Publisher: structured-world/rust-stemmers, release.yml, crates-io")


if __name__ == "__main__":
    main()
