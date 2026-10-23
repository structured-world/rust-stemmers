"""Verify retry safety and exact workflow/environment binding without credentials."""
import importlib.util
import pathlib
import unittest

spec = importlib.util.spec_from_file_location(
    "publisher", pathlib.Path(__file__).with_name("trusted-publisher.py"))
publisher = importlib.util.module_from_spec(spec)
spec.loader.exec_module(publisher)


class PublisherTest(unittest.TestCase):
    def test_retry_after_unknown_outcome_does_not_create_a_second_binding(self):
        # The server commits before the client loses its response. A rerun
        # must discover that exact binding, not repeat the mutation.
        configs = []
        writes = []

        def request(method, query, body=None):
            if method == "POST":
                writes.append(body)
                configs.append(body["github_config"])
                raise RuntimeError("connection lost")
            return {"github_configs": configs}

        with self.assertRaisesRegex(RuntimeError, "connection lost"):
            publisher.configure(request)
        publisher.configure(request)
        self.assertEqual(len(writes), 1)
        self.assertEqual(configs[0]["environment"], "crates-io")
        self.assertEqual(configs[0]["workflow_filename"], "release.yml")

    def test_different_environment_does_not_satisfy_the_binding(self):
        # An existing credential for another environment is not evidence
        # that this workflow has the intended permission.
        configs = [{"crate": "structured-stemmers", "environment": "other"}]

        def request(method, query, body=None):
            if method == "POST":
                configs.append(body["github_config"])
                return {}
            return {"github_configs": configs}

        publisher.configure(request)
        self.assertEqual(len(configs), 2)
        self.assertEqual(configs[1]["repository_name"], "rust-stemmers")


if __name__ == "__main__":
    unittest.main()
