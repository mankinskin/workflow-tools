# Native-routing validation

Current W8 uses original G7 and MN-W7-01 through MN-W7-05 identities.
Exact command, from meta-workspace root:

```text
cargo test --manifest-path workflow-tools/Cargo.toml --locked -p mission --all-features --test mission_native_routing -j 1
```

Passed six tests, zero failed/ignored/filtered, 17.15 seconds. The bounded
worker observed an initial red assertion showing the selector accepted
the original nineteen tools without the recorder; the green route requires
all twenty tools and stops visibly without fallback on incomplete discovery.

Tests cover classic, mission-backed and native-new-run selection; fresh named
Session intake and complete waypoint definition; named cold/formal reviews
and challenge preparation; missing receipt/spoofed requester rejection;
both consumer-root and initialized-product disposable installation layouts,
source hashes, rewrite parity, transitive link closure, aliases and idempotence.

Owning store: workflow-tools/.workflow-tools/test.
Validation spec: `mission-native-w8-routing-g7`.
Execution: `mission-native-w8-routing-g7-green-04b166f9`.
Same-root domain read-back confirms passed outcome and this Ticket/Spec links.
No Spec was automatically verified (`no-guards`).

Historical planning guard was the same command with no product run.
Product checkpoint: `feat(mission): add native guidance installation coverage`;
then parent prompts/gitlink:
`feat(workflow): default new intake to Mission-native planning`.
W9 installed real-requester pilot is excluded from the current authorization.
