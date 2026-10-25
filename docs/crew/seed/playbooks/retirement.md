---
id: retirement
version: 1
session_type: retirement
output: "The component removed and its data handled according to the plan"
done_when:
  - "The removal pull request is merged"
  - "Checks show that no references or old behaviour remain"
  - "Data was handled as the Human approved"
  - "Related ADRs are moved to superseded"
pr: required
seed:
  - "written new: none of the 13 seed repositories has a retirement workflow (brief 5.3.1)"
  - "Crew protocol: brief 4.3, NEED 06, 21, 39, 41"
  - "mattpocock to-tickets (this repo: .devin/skills/to-tickets): expand-contract sequencing for changes with a wide blast radius"
---

# Retirement

**INPUT**: a component, feature, or service to remove.
**OUTPUT**: the component removed, its data handled as approved, and the decision history updated.

## Size

| Size | Path |
| --- | --- |
| Small (dead code with no data and no consumers) | Steps 1, 4, 5 |
| Standard | All steps |

## Steps

| # | Step | ROLE | Kind | Input → Output | Gate | Human |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Inventory | software-architect | architecture | component → references, behaviours, data stores, contracts, consumers, and how each was found | Read-only | — |
| 2 | Removal plan | software-architect with platform-engineer | architecture | inventory → removal sequence, data-handling options with consequences, rollback, ADRs to supersede | Every consumer and data store covered | Chooses the data handling |
| 3 | Stories | backlog-analyst | docs | plan → Story Ready removal stories | Story Ready | — |
| 4 | Remove | an `implement` session | implement | stories → merged pull requests | Implement gates | Reviews the pull requests |
| 5 | Verify absence | qa-engineer | test | → searches, tests, and runtime checks showing no references or old behaviour remain | Evidence | — |
| 6 | Handle data | platform-engineer | implement | approved option → executed with permission | Project permission; rollback or stated irreversibility | Approves irreversible steps |
| 7 | Supersede ADRs | software-architect | docs | → ADRs moved to `superseded`, pointing to the decision that replaced them | — | — |

## Step notes

1. Search by symbol, configuration key, route, table, and feature flag; say how far each search reached.
2. When consumers exist, sequence expand and contract: deprecate, migrate consumers in batches, remove last. Data options (archive, migrate, delete) state what each makes impossible afterwards.
6. Data handling may need to run before the code removal (migrate away) or after it (delete); the plan fixes the order. Deleting data needs the project's permission and the Human's explicit choice.
7. ADRs keep their history: they move to `superseded` and point to their successor; they are never edited or deleted.

## Manual mode

The Human relays packets and handoffs, opens and merges the pull requests after approving them on GitHub, and performs or supervises the data handling.
