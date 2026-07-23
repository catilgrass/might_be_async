"""
Generate `*_expand.rs` and `*_async_expand.rs` files in `doc/usage/` by running
`cargo expand` on each usage example, once for sync and once for async.

Requires `cargo-expand` to be installed:
    cargo install cargo-expand
"""

import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TEMP = ROOT / ".temp"
USAGE = ROOT / "doc" / "usage"

EXPAND_BEGIN = "use might_be_async::*;\nconst EXPAND_BEGIN: () = ();"
EXPAND_END = "const EXPAND_END: () = ();"

CARGO_TOML = """\
[package]
name = "expander"
version = "0.0.0"
edition = "2024"

[workspace]

[features]
async = []

[dependencies]
might_be_async = { path = "../.." }
"""


def check_cargo_expand() -> None:
    """Ensure cargo-expand is installed."""
    try:
        subprocess.run(
            ["cargo", "expand", "--help"],
            capture_output=True,
            check=True,
        )
    except (FileNotFoundError, subprocess.CalledProcessError):
        print("error: `cargo expand` not found. Install it with:")
        print("    cargo install cargo-expand")
        sys.exit(1)


def prepare_temp() -> None:
    """Create .temp/sync/ and .temp/async/ skeletons."""
    if TEMP.exists():
        shutil.rmtree(TEMP)

    for variant in ("sync", "async"):
        (TEMP / variant / "src").mkdir(parents=True)
        (TEMP / variant / "Cargo.toml").write_text(CARGO_TOML)


def write_lib_rs(variant: str, source: str) -> None:
    """Wrap `source` with EXPAND markers and write to .temp/{variant}/src/lib.rs."""
    (TEMP / variant / "src" / "lib.rs").write_text(f"""\
#![allow(unused_imports, dead_code)]

{EXPAND_BEGIN}
{source}
{EXPAND_END}
""")


def run_expand(variant: str) -> str:
    """Run `cargo expand` in .temp/{variant}/, optionally with --features async."""
    cmd = ["cargo", "expand"]
    if variant == "async":
        cmd.append("--features")
        cmd.append("async")

    try:
        result = subprocess.run(
            cmd,
            cwd=TEMP / variant,
            capture_output=True,
            text=True,
            check=True,
        )
        return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"error: `cargo expand` failed in .temp/{variant}/:", file=sys.stderr)
        print(e.stderr, file=sys.stderr)
        sys.exit(1)


def extract_body(expanded: str) -> str:
    """Extract the text between the EXPAND_BEGIN and EXPAND_END markers."""
    lines = expanded.splitlines()
    cleaned = "\n".join(line for line in lines if not line.startswith("#!"))

    begin_idx = cleaned.find(EXPAND_BEGIN)
    end_idx = cleaned.find(EXPAND_END)

    if begin_idx == -1 or end_idx == -1:
        print("error: could not locate EXPAND markers in output", file=sys.stderr)
        print("=== expanded output (first 60 lines) ===")
        print("\n".join(cleaned.splitlines()[:60]))
        sys.exit(1)

    start = begin_idx + len(EXPAND_BEGIN)
    body = cleaned[start:end_idx]
    return body.strip()


def write_expand(name: str, suffix: str, body: str) -> None:
    """Write the expanded body to doc/usage/{name}_{suffix}expand.rs."""
    dest = USAGE / f"{name}_{suffix}expand.rs"
    dest.write_text(body + "\n")
    print(f"  → {dest.name}")


def main() -> None:
    check_cargo_expand()
    prepare_temp()

    # Find all input .rs files that are NOT already expanded
    inputs = sorted(
        p
        for p in USAGE.glob("*.rs")
        if not p.name.endswith("_expand.rs") and not p.name.endswith("_async_expand.rs")
    )

    if not inputs:
        print("No usage examples found in doc/usage/")
        sys.exit(0)

    print(f"Expanding {len(inputs)} example(s) with cargo-expand …\n")

    for src in inputs:
        stem = src.stem  # "func", "invoke", "select"
        code = src.read_text()

        for variant, suffix in [("sync", ""), ("async", "async_")]:
            write_lib_rs(variant, code)
            expanded = run_expand(variant)
            body = extract_body(expanded)
            write_expand(stem, suffix, body)

    print("\nDone.")


if __name__ == "__main__":
    main()
