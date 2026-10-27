# Requirements — typed draft persistence

Implement W2 after W1. `mission-api` owns strict v2 `source.json` and one typed `parts/<waypoint_id>.json` per waypoint, all path resolution, UUID/revision allocation, CAS persistence, history, and Markdown projection. Clients submit inline structured values and expected revision only; prompt and Part document remain distinct.

A waypoint has `validation: [{command, cwd?}]`, `commit_checkpoint: {message}`, optional `non_goals`, and ownership references. `command` and `message` are strings; a present `cwd` is a nonblank execution-directory string and **may be absolute**. It is execution metadata, not a Part/source reference. When supplied, every `non_goals` entry is nonblank. Only managed source paths must be safe relative paths below the Mission root.

Structurally valid incomplete drafts, including an empty draft, save atomically. Publish requires ordered waypoints plus at least one nonblank validation command and a nonblank checkpoint message for every waypoint, derives immediate predecessors, and rejects malformed nodes, unsafe source paths, stale revisions, and partial writes without changing accepted data. `non_goals` is never a publish gate. Legacy revision N and its history stay byte-identical; later migration appends v2 N+1.

Add and run once `cargo test -p mission-api --test typed_source`; reserve full API suites for W5. Checkpoint: `feat(mission-api): persist typed draft sources`. No transports or migration apply.
