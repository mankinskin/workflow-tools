#!/usr/bin/env python3
"""Prompt-replay skill-discovery matrix (CH-F).

Asserts that an agent using by-description loading resolves the correct skill
for a representative prompt in each target domain. The scorer mirrors
by-description matching: it scores each skill's `name` + `description`
frontmatter against the prompt's keyword set and requires the expected skill to
win outright.

This is a deterministic, dependency-free stand-in for live agent replay: it
proves the vendored/authored skill descriptions are discriminative enough that
the intended skill is the top match per domain. Run:

    python3 .agents/skills/tests/skill_discovery_matrix.py

Exit code 0 = all domains resolved correctly; non-zero = at least one miss.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

SKILLS_ROOT = Path(__file__).resolve().parents[1]

# Domain -> (representative prompt, expected skill folder/name).
# Covers every target domain from the skill-infrastructure spec (a9b7ef39).
MATRIX: list[tuple[str, str, str]] = [
    ("rust-async",
     "help me build an async Rust service with Tokio tasks, channels and streams",
     "rust-async-patterns"),
    ("rust-best-practices",
     "review this Rust code for idiomatic ownership, borrowing vs cloning and Result error handling",
     "rust-best-practices"),
    ("playwright-cli",
     "automate the browser and drive a page using the playwright-cli tool",
     "playwright-cli"),
    ("playwright-best-practices",
     "my Playwright end-to-end test is flaky, help me debug it and set up a page object model in CI",
     "playwright-best-practices"),
    ("webgpu-typegpu",
     "write a type-safe WebGPU compute pipeline in TypeScript using tgpu.fn and buffers with TypeGPU",
     "typegpu"),
    ("webgpu-threejs-tsl",
     "set up a Three.js WebGPU renderer with TSL node materials and a compute shader",
     "webgpu-threejs-tsl"),
    ("interviewing",
     "how do I conduct high-impact customer interviews to uncover root user pain and triggers",
     "customer-interviews"),
    ("authoring",
     "guide me through co-authoring a technical design doc / proposal / spec",
     "doc-coauthoring"),
    ("dioxus",
     "help me write a Dioxus component with use_signal and wire up trunk serve for the viewer-api frontend",
     "dioxus"),
    ("token-optimization",
     "make my coding agent more token efficient and preserve the context window with compaction",
     "token-optimized-agentic-engineering"),
    ("find-skills",
     "is there a skill for this, how do I find and install a skill to extend capabilities",
     "find-skills"),
]

STOP = {
    "the", "a", "an", "and", "or", "to", "of", "for", "in", "on", "with", "my",
    "me", "i", "help", "how", "do", "is", "this", "up", "using", "use", "set",
    "build", "make", "write", "guide", "through", "it", "am", "are", "your",
}


def tokenize(text: str) -> set[str]:
    return {t for t in re.findall(r"[a-z0-9]+", text.lower()) if t not in STOP and len(t) > 1}


def read_frontmatter(skill_md: Path) -> tuple[str, str]:
    """Return (name, description) from a SKILL.md YAML frontmatter block."""
    text = skill_md.read_text(encoding="utf-8")
    m = re.match(r"^---\s*\n(.*?)\n---\s*\n", text, re.DOTALL)
    if not m:
        raise ValueError(f"missing frontmatter: {skill_md}")
    fm = m.group(1)
    name = re.search(r"^name:\s*(.+)$", fm, re.MULTILINE)
    # description may be a folded/literal block spanning multiple lines.
    desc = re.search(r"^description:\s*(.*)$", fm, re.MULTILINE)
    if not name:
        raise ValueError(f"missing name in frontmatter: {skill_md}")
    name_val = name.group(1).strip().strip('"').strip("'")
    # Gather the description text: the value on the description line plus any
    # subsequent indented continuation lines (handles '>' and '|' YAML blocks).
    desc_lines: list[str] = []
    if desc:
        head = desc.group(1).strip().lstrip(">|").strip().strip('"').strip("'")
        if head:
            desc_lines.append(head)
        after = fm[desc.end():]
        for line in after.splitlines():
            if re.match(r"^\S", line):  # next top-level key ends the block
                break
            desc_lines.append(line.strip())
    return name_val, " ".join(desc_lines)


def load_skills() -> dict[str, str]:
    """folder name -> (name + description) searchable text."""
    out: dict[str, str] = {}
    for skill_md in sorted(SKILLS_ROOT.glob("*/SKILL.md")):
        folder = skill_md.parent.name
        name, desc = read_frontmatter(skill_md)
        out[folder] = f"{name} {name.replace('-', ' ')} {desc}"
    return out


def score(prompt: str, skill_text: str) -> int:
    return len(tokenize(prompt) & tokenize(skill_text))


def main() -> int:
    skills = load_skills()
    print(f"discovered {len(skills)} skills: {', '.join(sorted(skills))}\n")

    failures = 0
    for domain, prompt, expected in MATRIX:
        if expected not in skills:
            print(f"FAIL [{domain}] expected skill '{expected}' not found on disk")
            failures += 1
            continue
        ranked = sorted(
            ((score(prompt, txt), folder) for folder, txt in skills.items()),
            reverse=True,
        )
        top_score, top_skill = ranked[0]
        ok = top_skill == expected and top_score > 0
        # Detect ties at the top that include a wrong skill.
        tied = [f for s, f in ranked if s == top_score]
        if ok and len(tied) > 1:
            ok = expected in tied and len(tied) == 1
        status = "PASS" if ok else "FAIL"
        if not ok:
            failures += 1
        runner = ", ".join(f"{f}:{s}" for s, f in ranked[:3])
        print(f"{status} [{domain}] -> {top_skill} (expected {expected}) top3: {runner}")

    print()
    total = len(MATRIX)
    print(f"result: {total - failures}/{total} domains resolved correctly")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
