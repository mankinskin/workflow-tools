## Scope
Batches the low-architecture-risk guidance-corpus tickets that can be executed without waiting on the Planner/Worker spec track. Closes when its three children are done.

## Children (depends_on)
- Extend entity disambiguation protocol to all agent responses (quick win, do first)
- Simplify Agent: audit and condense instruction/guidance corpus
- Scoped/dynamic guidance injection by ticket domain tag (benefits from the condensed corpus produced above)

## Sequencing note
Priority order within this epic (non-blocking `linked` edges, see ticket graph): entity disambiguation -> Simplify Agent -> scoped guidance injection.
Rollup: children 95403a71 (applyTo frontmatter + README convention), 6426c891, and 9cd886d5 (spec 1b654f30 tightening) are all done.