#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "semver",
# ]
# ///
"""
Version management for the diom monorepo.

Usage:
  uv run tools/bump_version.py check   # verify all tracked files match .version
  uv run tools/bump_version.py bump    # propagate .version into all tracked files
"""

import argparse
import re
import subprocess
import sys
from pathlib import Path
from dataclasses import dataclass, field

import semver

REPO_ROOT = Path(__file__).parent.parent


@dataclass
class VersionFile:
    path: str
    patterns: list[str]
    pattern_flags: int = 0


POST_BUMP_COMMANDS = [
    # Python
    "cd z-clients/python/ && uv sync",
    # Rust
    "cargo update --workspace",
    # JavaScript
    "cd z-clients/javascript && npm i --package-lock-only --ignore-scripts"
]

VERSION_FILES = [
    VersionFile(
        "Cargo.toml",
        [
            r'(\[workspace\.package\][\s\S]*?\nversion\s*=\s*")([^"]*)(")',
        ],
    ),
    VersionFile(
        "z-clients/javascript/package.json",
        [r'("version"\s*:\s*")([^"]*)(")'],
    ),
    VersionFile(
        "z-clients/javascript/src/request.ts",
        [
            r'(export const LIB_VERSION\s*=\s*")([^"]*)(")',
        ],
    ),
    VersionFile(
        "z-clients/python/pyproject.toml",
        [r'(^version\s*=\s*")([^"]*)(")'],
        re.MULTILINE,
    ),
    VersionFile(
        "z-clients/python/diom/client_base.py",
        [r'("user-agent":\s*"svix-libs/)([^/]*)(\/python")'],
    ),
    VersionFile(
        "z-clients/java/pom.xml",
        [r"(<artifactId>diom</artifactId>\s*<version>)([^<]*)(</version>)"],
    ),
    VersionFile(
        "z-clients/java/src/main/java/com/svix/diom/Version.java",
        [
            r'(public static final String VERSION\s*=\s*")([^"]*)(")',
        ],
    ),
    VersionFile(
        "z-clients/java/README.md",
        [
            r"(<version>)([^<]*)(</version>)",
            r'(com\.svix:diom:)([^"]*)(")',
        ],
    ),
    VersionFile(
        "z-clients/go/client.go",
        [r'("diom-sdks/)([^/]*)(\/go")'],
    ),
    VersionFile(
        "openapi.json",
        [r'("info"[\s\S]*?"version"\s*:\s*")([^"]*)(")', ],
    ),
]


def read_canonical_version() -> str:
    return (REPO_ROOT / ".version").read_text().strip()


def cmd_check() -> int:
    expected = read_canonical_version()
    errors = 0
    for vf in VERSION_FILES:
        content = (REPO_ROOT / vf.path).read_text()
        for pattern in vf.patterns:
            m = re.search(pattern, content, vf.pattern_flags)
            if not m:
                print(f"NO MATCH pattern '{pattern}' in {vf.path}", file=sys.stderr)
                errors += 1
                continue
            actual = m.group(2)
            if actual != expected:
                print(
                    f"MISMATCH {vf.path}: got '{actual}', expected '{expected}'",
                    file=sys.stderr,
                )
                errors += 1

    if errors:
        print(
            f"\nERROR: {errors} version mismatch(es) found. All versions must match .version ({expected}).",
            file=sys.stderr,
        )
        return 1

    print(f"All versions match: {expected}")
    return 0


def cmd_bump(new_version: str) -> int:
    if not semver.Version.is_valid(new_version):
        print(f"ERROR: '{new_version}' is not a valid semver", file=sys.stderr)
        return 1

    old_version = read_canonical_version()

    if semver.Version.is_valid(old_version) and semver.Version.parse(
        new_version
    ) <= semver.Version.parse(old_version):
        print(
            f"ERROR: new version '{new_version}' is not greater than current '{old_version}'",
            file=sys.stderr,
        )
        return 1

    (REPO_ROOT / ".version").write_text(new_version + "\n")
    version = new_version

    for vf in VERSION_FILES:
        abs_path = REPO_ROOT / vf.path
        content = abs_path.read_text()
        for pattern in vf.patterns:
            m = re.search(pattern, content, vf.pattern_flags)
            if not m:
                print(
                    f"ERROR: no match for pattern '{pattern}' in {vf.path}",
                    file=sys.stderr,
                )
                return 1

            content = re.sub(
                pattern,
                lambda mo: f"{mo.group(1)}{version}{mo.group(3)}",
                content,
                flags=vf.pattern_flags,
            )
        abs_path.write_text(content)

    print(f"Updated all versions to: {version}")
    if rc := cmd_check():
        return rc

    for cmd in POST_BUMP_COMMANDS:
        print(f"\n$ {cmd}")
        result = subprocess.run(cmd, shell=True, cwd=REPO_ROOT)
        if result.returncode != 0:
            print(f"ERROR: command failed: {cmd}", file=sys.stderr)
            return result.returncode

    return 0


def main() -> None:
    parser = argparse.ArgumentParser(description="diom monorepo version management")
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("check", help="verify all tracked files match .version")
    bump_parser = sub.add_parser(
        "bump", help="set new version in .version and propagate to all tracked files"
    )
    bump_parser.add_argument("version", help="new semver version to set (e.g. 1.2.3)")

    args = parser.parse_args()
    sys.exit(cmd_check() if args.command == "check" else cmd_bump(args.version))


if __name__ == "__main__":
    main()
