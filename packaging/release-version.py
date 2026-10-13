"""Resolve and validate a release version without evaluating shell expressions."""
import os
import re
import tomllib
from pathlib import Path


def resolve(value, manifest):
    version = value or manifest["workspace"]["package"]["version"]
    if not re.fullmatch(r"(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-[0-9A-Za-z]+(?:[.-][0-9A-Za-z]+)*)?", version):
        raise ValueError("Expected a version such as 0.7.1 or 0.7.1-rc.1")
    return version


if __name__ == "__main__":
    root = Path(__file__).resolve().parents[1]
    version = resolve(os.environ.get("NEXOPDF_VERSION_INPUT", ""), tomllib.loads((root / "Cargo.toml").read_text()))
    output = os.environ.get("GITHUB_OUTPUT")
    if output:
        with open(output, "a", encoding="utf-8") as handle:
            handle.write(f"version={version}\n")
    print(version)
