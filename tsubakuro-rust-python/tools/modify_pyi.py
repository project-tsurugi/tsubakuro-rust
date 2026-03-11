#!/usr/bin/env python3
import argparse
import re
from pathlib import Path


def process_file(path: Path) -> None:
    pattern = re.compile(r"builtins\.(?!Exception\b)([A-Za-z]*(?:Exception|Error))")

    original = path.read_text(encoding="utf-8")
    lines = original.splitlines(keepends=True)

    new_lines = []
    changed = False

    for lineno, line in enumerate(lines, 1):
        new_line = pattern.sub(r"\1", line)
        if new_line != line:
            print(f"{path}:{lineno}: {line}", end="")
            changed = True
        new_lines.append(new_line)

    if changed:
        path.write_text("".join(new_lines), encoding="utf-8")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-d", "--directory", required=True, help="Target directory")
    args = parser.parse_args()

    root = Path(args.directory)

    for pyi_file in root.rglob("*.pyi"):
        # Skip paths with directories starting with a period
        if any(part.startswith(".") for part in pyi_file.relative_to(root).parts[:-1]):
            continue
        process_file(pyi_file)


if __name__ == "__main__":
    main()
