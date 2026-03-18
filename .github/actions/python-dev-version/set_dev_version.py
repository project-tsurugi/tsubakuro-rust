from pathlib import Path
import re
import sys
import tempfile


SECTION_RE = re.compile(r"^\s*\[(?P<name>[^\[\]]+)\]\s*(?:#.*)?$")
VERSION_RE = re.compile(r'^(?P<prefix>[ \t]*version\s*=\s*")(?P<version>[^"]+)(?P<suffix>".*)$')


def split_line_ending(line: str) -> tuple[str, str]:
    if line.endswith("\r\n"):
        return line[:-2], "\r\n"
    if line.endswith("\n"):
        return line[:-1], "\n"
    if line.endswith("\r"):
        return line[:-1], "\r"
    return line, ""


def find_section_bounds(lines: list[str], section_name: str, target_file: Path) -> tuple[int, int]:
    in_target_section = False
    section_start = None

    for index, line in enumerate(lines):
        match = SECTION_RE.match(line)
        if not match:
            continue

        current_section = match.group("name").strip()
        if in_target_section:
            return section_start, index

        if current_section == section_name:
            in_target_section = True
            section_start = index + 1

    if in_target_section:
        return section_start, len(lines)

    raise RuntimeError(f"Section [{section_name}] not found in {target_file}")


def append_suffix_to_version(lines: list[str], start: int, end: int, suffix: str, target_file: Path) -> bool:
    for index in range(start, end):
        line_body, line_ending = split_line_ending(lines[index])
        match = VERSION_RE.match(line_body)
        if not match:
            continue

        current_version = match.group("version")
        if current_version.endswith(suffix):
            return False

        lines[index] = (
            f"{match.group('prefix')}{current_version}{suffix}{match.group('suffix')}"
            f"{line_ending}"
        )
        return True

    raise RuntimeError(f"version entry not found in [package] section of {target_file}")


def update_manifest(target_file: Path, suffix: str) -> bool:
    with target_file.open("r", encoding="utf-8", newline="") as file:
        lines = file.readlines()

    package_start, package_end = find_section_bounds(lines, "package", target_file)
    changed = append_suffix_to_version(lines, package_start, package_end, suffix, target_file)

    if changed:
        with target_file.open("w", encoding="utf-8", newline="") as file:
            file.writelines(lines)

    return changed


def assert_equal(actual: object, expected: object, message: str) -> None:
    if actual != expected:
        raise AssertionError(f"{message}: expected {expected!r}, got {actual!r}")


def assert_raises(function, expected_message: str) -> None:
    try:
        function()
    except RuntimeError as error:
        if expected_message not in str(error):
            raise AssertionError(
                f"unexpected error message: expected to contain {expected_message!r}, got {str(error)!r}"
            ) from error
        return

    raise AssertionError("expected RuntimeError was not raised")


def run_self_test() -> int:
    suffix = "-dev1234"
    valid_manifest = """[package]
name = "example"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1"
"""

    with tempfile.TemporaryDirectory() as temp_dir:
        manifest_path = Path(temp_dir) / "Cargo.toml"

        manifest_path.write_text(
            valid_manifest,
            encoding="utf-8",
            newline="",
        )
        changed = update_manifest(manifest_path, suffix)
        updated_manifest = manifest_path.read_text(encoding="utf-8")

        assert_equal(changed, True, "first update should modify the manifest")
        assert_equal(
            'version = "0.1.0-dev1234"' in updated_manifest,
            True,
            "version suffix should be appended in [package]",
        )
        assert_equal(
            'version = "0.1.0-dev1234"\nedition = "2021"' in updated_manifest,
            True,
            "updated version line should preserve the following newline",
        )
        assert_equal(
            'serde = "1"' in updated_manifest,
            True,
            "entries outside [package] should remain untouched",
        )

        changed = update_manifest(manifest_path, suffix)
        assert_equal(changed, False, "second update should be idempotent")

        missing_section_path = Path(temp_dir) / "missing-section.toml"
        missing_section_path.write_text(
            """[dependencies]\nserde = \"1\"\n""",
            encoding="utf-8",
            newline="",
        )
        assert_raises(
            lambda: update_manifest(missing_section_path, suffix),
            "Section [package] not found",
        )

        missing_version_path = Path(temp_dir) / "missing-version.toml"
        missing_version_path.write_text(
            """[package]\nname = \"example\"\n""",
            encoding="utf-8",
            newline="",
        )
        assert_raises(
            lambda: update_manifest(missing_version_path, suffix),
            "version entry not found",
        )

    print("self-test passed")
    return 0


def main(argv: list[str] | None = None) -> int:
    args = list(sys.argv[1:] if argv is None else argv)

    if args == ["--self-test"]:
        return run_self_test()

    if len(args) != 2:
        raise SystemExit("usage: set_dev_version.py <manifest_path> <suffix> | --self-test")

    update_manifest(Path(args[0]), args[1])
    return 0


if __name__ == "__main__":
    raise SystemExit(main())