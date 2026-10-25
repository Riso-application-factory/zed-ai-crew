---
id: review-retro
version: 1
session_type: review-retro
output: "Improvement proposals backed by evidence"
done_when:
  - "The Human decided every proposal"
  - "Accepted proposals are opened as new sessions or stories"
pr: none
seed:
  - "Crew protocol: brief 4.3, NEED 43, 49, 54"
  - "mattpocock retro (this repo: .devin/skills/retro): improvement categories; mechanical rules become checks, judgement calls become written standards"
  - "dropped: any self-scoring, self-evolving, or self-applied rule change (brief 5.3.1 'not used' list)"
---

# Review / retro

**INPUT**: a set of sessions or a period to review, and the questions the Human wants answered.
**OUTPUT**: improvement proposals, each with its evidence, decided by the Human.

## Size

| Size | Path |
| --- | --- |
| Small (one session) | Steps 1, 2, 3, 5 |
| Standard | All steps |

## Steps

| # | Step | ROLE | Kind | Input → Output | Gate | Human |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Scope | orchestrator | orchestrate | request → sessions or period, questions to answer | — | Confirms scope |
| 2 | Gather evidence | delivery-manager | research | recorded facts (task states, handoffs, verdicts, check results, pull request events, failures) → evidence set | Facts only, each with its location | — |
| 3 | Propose improvements | delivery-manager; qa-engineer for checks and tests | review | evidence → proposals by category, ordered by severity, each with an evaluation plan per Model–Harness pair | Every proposal tied to evidence | — |
| 4 | Check the proposals | evaluator | review | proposals and evidence → verdict | Each proposal follows from its evidence | — |
| 5 | Decide | orchestrator | orchestrate | proposals → one decision per proposal | — | Decides each proposal |
| 6 | Open follow-ups | backlog-analyst | docs | accepted proposals → sessions or stories | — | — |

## Step notes

3. Prefer changes to the Outer Harness over longer prompts: restore a check that exists but is unwired, turn a repeated mechanical mistake into a deterministic check, sharpen a standard for judgement calls, and remove steps that current models and checks make redundant.
5. A proposal that changes agent behaviour (a ROLE, a playbook, a model choice) is evaluated on representative cases per Model–Harness pair before it is applied; that evaluation belongs to the follow-up session.
6. Nothing changes until the Human accepts it; Crew never applies its own rule changes.

## Manual mode

The Human gathers the evidence the workers cannot reach (logs, pull request history) or gives them access, and relays packets and handoffs.
