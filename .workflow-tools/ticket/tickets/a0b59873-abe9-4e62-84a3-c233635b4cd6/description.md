## Overview
Create or update spec and validation for the complete tool-metrics → graded-cost-gate → grant → escalation workflow.

## Key Design Points
- **Spec scope**: spans T1 (tool_metrics core), T2 (surfaces + rollup), T3 (graded cost model), T5 (grants), T6 (escalation)
- **Spec**: spec 29ae5f6e "Empirical tool-metrics driven cost-gate classification" (widen to include graded model + grants + escalation)
- **Validation coverage**: 
  - Tool-metrics collection (T1/T2)
  - Graded cost + budget calculation (T3)
  - Grant CRUD + offset resolution (T5)
  - Escalation workflow + async queue (T6)
  - End-to-end: metrics → classification → gate decision with grant → escalation

## Acceptance Criteria
- [ ] Spec 29ae5f6e updated with sections: Graded Cost Scale, Model Budget & Offset, Grant Records, Escalation Workflow
- [ ] Traceability links: T1, T2, T3, T5, T6
- [ ] Validation plan: unit tests for each component, integration test for full workflow
- [ ] Tests linked to spec sections
- [ ] Spec moved to active state when implementation complete

## References
- Depends on: T1, T2, T3, T5, T6
- Target: .spec/specs/29ae5f6e-c202-41f1-ba88-a446aa872993/