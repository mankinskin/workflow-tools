## Problem

`--grep` with regex alternation (`\|`) reports no match even when individual alternatives do match:

```
$ peek store.rs --grep "fn list\|pub fn"
peek: no match for "fn list\|pub fn" in store.rs
```

Searching for either pattern individually returns results. The tool does not document whether `--grep` accepts plain substrings or regex, causing confusion.

## Expected behaviour

Either:
1. Accept full Rust `regex` syntax (including `|` alternation) and document it clearly, OR
2. Accept plain fixed-string patterns only and document it clearly (and reject/warn on patterns that look like regex).

Option 1 is preferred - it matches user expectations for a grep-style flag.

## Scope

- Decide and document whether `--grep` is regex or fixed-string.
- If regex: switch the matching code to use the `regex` crate and support `|` alternation.
- Update `--help` to clarify the accepted pattern syntax.

## Acceptance criteria

- `peek file.rs --grep "fn list|pub fn"` returns all lines matching either alternative.
- Help text states whether the pattern is a regex or a fixed string.