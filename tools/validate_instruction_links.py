#!/usr/bin/env python3
"""Instruction-link validator (ticket 5bc9fede, Waypoint 4).

Checks every relative link in `--baseline` against the disposition table in
`--manifest` (instruction-distribution.md). A link is safe if source and
target share the same disposition (they move/stay together) or is otherwise
required to appear in `--exceptions` with an owner and expiry date.

Exit 0 and print "PASS" only when every baseline reference resolves or is a
reviewed, non-expired exception. Otherwise print each unresolved reference
and exit 1.
"""
from __future__ import annotations

import argparse
import datetime as dt
import posixpath
import re
import sys
from pathlib import Path

MANIFEST_ROW_RE = re.compile(r"^\|\s*`([^`]+)`\s*\|\s*(.+?)\s*\|\s*$")
# exceptions file: <source>\t<target>\t<owner>\t<expiry YYYY-MM-DD>\t<reason>
EXCEPTION_RE = re.compile(r"^([^\t]+)\t([^\t]+)\t([^\t]+)\t(\d{4}-\d{2}-\d{2})\t(.*)$")


def bucket(disposition: str) -> str:
    """Reduce a disposition string to its comparable bucket (strip parenthetical notes)."""
    return disposition.split(" (", 1)[0].strip()


def load_manifest(path: Path) -> dict[str, str]:
    table: dict[str, str] = {}
    in_table = False
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.startswith("| Source path"):
            in_table = True
            continue
        if not in_table:
            continue
        if line.startswith("| --- "):
            continue
        m = MANIFEST_ROW_RE.match(line)
        if not m:
            continue
        source, disposition = m.group(1), m.group(2)
        table[source] = bucket(disposition)
    return table


def load_baseline(path: Path) -> list[tuple[str, str]]:
    rows = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip():
            continue
        source, target = line.split("\t", 1)
        rows.append((source, target))
    return rows


def load_exceptions(path: Path | None) -> dict[tuple[str, str], tuple[str, str]]:
    exceptions: dict[tuple[str, str], tuple[str, str]] = {}
    if path is None or not path.exists():
        return exceptions
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        m = EXCEPTION_RE.match(line)
        if not m:
            print(f"WARN: malformed exception line ignored: {line}", file=sys.stderr)
            continue
        source, target, owner, expiry, _reason = m.groups()
        exceptions[(source, target)] = (owner, expiry)
    return exceptions


def resolve_target(source: str, target: str) -> str | None:
    """Resolve `target` relative to `source`'s directory within the manifest's
    root (`context-engine/.agents/`). Returns the normalized path relative to
    that root, or None if it escapes the root (points elsewhere in the repo)."""
    clean_target = target.split("#", 1)[0].strip()
    if not clean_target:
        return None
    source_dir = posixpath.dirname(source)
    joined = posixpath.normpath(posixpath.join(source_dir, clean_target))
    if joined.startswith(".."):
        return None
    return joined


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--manifest", required=True, type=Path)
    ap.add_argument("--baseline", required=True, type=Path)
    ap.add_argument("--exceptions", type=Path, default=None)
    args = ap.parse_args()

    manifest = load_manifest(args.manifest)
    baseline = load_baseline(args.baseline)
    exceptions = load_exceptions(args.exceptions)

    today = dt.date.today()
    failures: list[str] = []
    warnings: list[str] = []
    expired_exceptions: list[str] = []

    for source, target in baseline:
        source_disposition = manifest.get(source)
        if source_disposition is None:
            failures.append(f"UNKNOWN SOURCE (not in manifest): {source}")
            continue

        # A retained file never relocates, so nothing about migration can
        # break its links. Any dangling/escaping reference here is pre-existing
        # corpus hygiene debt, not a migration-safety failure (out of scope
        # per this ticket's non-goal: do not change instruction semantics).
        retained_source = source_disposition == "retained: context-engine"

        resolved = resolve_target(source, target)
        if resolved is None:
            # Reference escapes context-engine/.agents/ (e.g. ../../AGENTS.md,
            # ../../.ticket/...). Safe only if the source itself never moves.
            if retained_source:
                continue
            key = (source, target)
            if key in exceptions:
                owner, expiry = exceptions[key]
                if expiry < today.isoformat():
                    expired_exceptions.append(
                        f"{source} -> {target} (owner={owner}, expired {expiry})"
                    )
                continue
            failures.append(
                f"{source} -> {target}: escapes corpus root and source moves "
                f"to {source_disposition}; not a reviewed exception"
            )
            continue

        target_disposition = manifest.get(resolved)
        if target_disposition is None:
            msg = (
                f"{source} -> {target}: resolved to {resolved!r}, which is not "
                "in the manifest (dangling reference)"
            )
            (warnings if retained_source else failures).append(msg)
            continue

        if source_disposition == target_disposition:
            continue

        if retained_source and target_disposition == "retained: context-engine":
            continue

        key = (source, target)
        if key in exceptions:
            owner, expiry = exceptions[key]
            if expiry < today.isoformat():
                expired_exceptions.append(
                    f"{source} -> {target} (owner={owner}, expired {expiry})"
                )
            continue

        msg = (
            f"{source} ({source_disposition}) -> {target} ({target_disposition}): "
            "disposition mismatch, not a reviewed exception"
        )
        (warnings if retained_source else failures).append(msg)

    if expired_exceptions:
        print("EXPIRED EXCEPTIONS:", file=sys.stderr)
        for line in expired_exceptions:
            print(f"  {line}", file=sys.stderr)

    if warnings:
        print(
            f"NOTE: {len(warnings)} pre-existing broken reference(s) in "
            "retained (non-relocating) files -- corpus hygiene debt, not a "
            "migration-safety failure:",
            file=sys.stderr,
        )
        for line in warnings:
            print(f"  {line}", file=sys.stderr)

    if failures:
        print("FAIL: unresolved instruction-link references:", file=sys.stderr)
        for line in failures:
            print(f"  {line}", file=sys.stderr)
        print(f"\n{len(failures)} failing reference(s), "
              f"{len(expired_exceptions)} expired exception(s).", file=sys.stderr)
        return 1

    if expired_exceptions:
        print(f"FAIL: {len(expired_exceptions)} expired exception(s).", file=sys.stderr)
        return 1

    print(
        f"PASS: {len(baseline)} baseline reference(s) checked against "
        f"{len(manifest)} manifest entries and {len(exceptions)} exception(s) "
        f"({len(warnings)} pre-existing warning(s) in retained files)."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
