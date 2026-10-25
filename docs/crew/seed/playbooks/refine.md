---
id: refine
version: 1
session_type: refine
output: "Story Ready stories"
done_when:
  - "Each story meets Story Ready: observable acceptance criteria, explicit scope_out, assumptions confirmed by the Human"
  - "The readiness check by a different author returns PASS, or the Human accepted each concern"
pr: none
seed:
  - "Crew protocol: brief 4.3, NEED 11, 12, 19, 20, 21, 22, 39"
  - "mattpocock wayfinder and grilling (this repo: .devin/skills): decision map with fog of war; rounds of questions"
  - "@fission-ai/openspec@1.15.0 schemas/spec-driven: delta spec"
  - "bmad-method@6.12.4 src/bmm-skills/plan/bmad-create-epics-and-stories and bmad-sprint-planning/references/readiness-gate.md"
  - "get-shit-done-cc@1.42.3 get-shit-done/templates/SECURITY.md: trust boundaries and threat register"
  - "dropped: wayfinder issue labels and claim-by-assignment mechanics; BMAD sprint-status tracking; GSD yolo mode"
---

# Refine

**INPUT**: an idea, a goal, or a request not yet ready to build.
**OUTPUT**: Story Ready stories, published as Issues.

## Size

| Size | Path |
| --- | --- |
| Small (a bugfix or a bounded change) | Steps 3 and 6 produce a minimal Story Ready (problem, expected behaviour, acceptance criteria, scope_out), then step 7 |
| Standard | Steps 2 to 8 |
| Large or foggy | Step 1 first, then the rest once the decision map is resolved |

## Steps

| # | Step | ROLE | Kind | Input → Output | Gate | Human |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Decision map | orchestrator | orchestrate | request → destination, decisions made, open questions with dependencies, fog | No ticket is written for fog | Answers rounds of questions |
| 2 | Survey the real system | software-architect | architecture | → current behaviour, capabilities, constraints relevant to the change | Read-only; verified versus unknown stated | — |
| 3 | Delta spec | product-manager (business-analyst when the domain has many business rules) | docs | intent and survey → ADDED, MODIFIED, REMOVED requirements with scenarios | Every requirement testable; assumptions tagged | Confirms assumptions |
| 4 | Design, when UI or flows change | ux-designer, ui-designer | ui | delta spec → experience and visual spec; prototype when one question needs it | Reference features mapped to real capabilities | Chooses among variations |
| 5 | Threat model, when data, authentication, or a trust boundary is touched | security-engineer | review | delta spec → trust boundaries, threats, abuse cases, rollback | Every threat has a disposition | Accepts risks |
| 6 | Stories | backlog-analyst | docs | spec, designs, threat model → stories, slices, dependencies | Story Ready fields present | — |
| 7 | Readiness check | tech-lead (analysis mode) | review | stories and spec → PASS, CONCERNS, or FAIL with findings | Different author; read-only | Decides on concerns |
| 8 | Publish | Crew | — | stories → GitHub Issues, hierarchy by owner type | Marker and hash on each issue | — |

## Step notes

1. The map is an index of decisions, not a store: each decision lives in one place. Questions are asked in rounds, each with context, options, and a recommendation.
2. On an existing system the change is described as a delta on top of what the survey verified, never as a rewrite of the whole system's specification.
4. A prototype lives on a `crew/prototype/<name>` branch, answers one stated question, and is never merged; its verdict is recorded on the story.
5. Accepting a risk is the Human's decision and is recorded with that decision.
6. Stories depend only on earlier stories; data structures are created by the first story that needs them.
7. FAIL sends the work back to the step that owns each finding.
8. Organisation repos use issue types plus sub-issues; personal repos use labels plus sub-issues. Crew is the only automated writer to Issues.

## Manual mode

The Human relays packets and handoffs and posts the issue drafts from step 6 to GitHub.
