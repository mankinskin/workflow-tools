## Objective
Implement the Track 1 directed inherited schema lifecycle engine defined by the completed research and planning records.

## Scope
Add strict single-parent schema resolution, directed lifecycle transition validation, plan/act/verify category-path validation, and atomic registry-generation reload behavior. Preserve the legacy-compatible representation needed by later loader and migration tracks.

## Done
The lifecycle engine is covered by focused tests, preserves prior valid registry generations on invalid reload, and provides the resolved model required by the downstream loader track.