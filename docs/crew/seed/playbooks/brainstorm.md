---
id: brainstorm
version: 1
session_type: brainstorm
output: "A set of ideas that meets the criteria agreed with the Human"
done_when:
  - "The idea set meets the criteria the Human agreed to in step 1"
  - "Each idea states its trade-offs, risks, and open uncertainties"
pr: none
seed:
  - "Crew protocol: brief 4.3, 5.4.3, NEED 10, 19, 20, 45, 46"
  - "superpowers-mcp@6.4.7 skills/brainstorming: discover intent first, write back the understanding and let the Human correct it, separate what was said from assumptions"
  - "mattpocock grilling (this repo: .devin/skills/grilling): rounds of questions with a recommended answer"
  - "dropped: Superpowers hard gate that turns an approved design into a spec and implementation plan (implementing an idea is a separate request here); visual companion server"
---

# Brainstorm

**INPUT**: a goal or a problem the Human wants options for, for example "find ways to cut review effort on this project".
**OUTPUT**: a set of ideas that meets the agreed criteria. No pull request; implementing an idea is a new request.

## Size

| Size | Path |
| --- | --- |
| Small (a few options for a narrow question) | Steps 1, 3, 5, 6 |
| Standard | All steps |

## Steps

| # | Step | ROLE | Kind | Input → Output | Gate | Human |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Frame | orchestrator | orchestrate | request → intent, who benefits, success criteria for the idea set, constraints | Criteria written down | Confirms the criteria |
| 2 | Survey context | business-analyst, software-architect, or ux-designer, as needed | research | frame → facts about users, the system's real capabilities, constraints | Read-only; sources cited | — |
| 3 | Generate ideas | product-manager | docs | frame and survey → idea set: distinct directions, each with how it meets the criteria, trade-offs, risks, unknowns | Ideas genuinely distinct; none dropped silently | — |
| 4 | Check feasibility | software-architect | architecture | idea set → capability map per idea (supported, needs a capability, not feasible) | Every borrowed feature mapped to a real capability | — |
| 5 | Evaluate | evaluator | review | idea set and criteria → verdict | Each idea checked against goals, constraints, and capabilities | — |
| 6 | Present | orchestrator | orchestrate | → ideas with trade-offs and uncertainty | — | Picks, combines, or discards |

## Step notes

1. Write back the understanding (intended outcome, constraints, success criteria) and separate what the Human said from what is assumed. The criteria are what Done is judged against.
3. Diverge before converging: aim for directions that differ in approach, not variations of one idea. Mark ideas that depend on an assumption still open.
4. An idea that copies a reference product needs each borrowed feature mapped to a capability of this system; gaps are stated, not hidden.
6. Present the trade-offs plainly and recommend, but the choice is the Human's. A chosen idea becomes the INPUT of a new refine or implement session.

## Manual mode

The Human relays the packets for steps 2 to 5 and pastes the handoffs back into the Orchestrator thread.
