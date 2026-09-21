Define domain extension contract crates and implement first provider/consumer pair.

Scope:
- create domain extension traits where trigger-domain ownership applies
- implement one dependency-domain provider for a trigger-domain contract
- validate no domain crate cycle is introduced

Acceptance criteria:
- at least one extension contract path is wired and tested
- ownership boundaries are clear and documented
