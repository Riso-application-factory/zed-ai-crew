---
id: research
version: 1
session_type: research
output: "A conclusion backed by evidence"
done_when:
  - "The report states the question and the scope"
  - "Every key claim cites a source that says what is claimed"
  - "Each conclusion states its certainty, and facts are separated from inference"
pr: none
seed:
  - "Crew protocol: brief 4.3, NEED 46, 49"
  - "mattpocock research (this repo: .devin/skills/research): investigate against primary sources and cite each claim"
  - "bmad-method@6.12.4 src/bmm-skills/agents/bmad-agent-analyst/customize.toml: research types (market, domain, technical, competitive, user voice, technology selection)"
  - "dropped: BMAD deep-recon skill mechanics and menus; mattpocock research background-agent mechanics (Crew spawns the worker)"
---

# Research

**INPUT**: a question whose answer informs a decision.
**OUTPUT**: a report with a conclusion, its sources, and its certainty.

## Size

| Size | Path |
| --- | --- |
| Small (one fact-finding question) | Use `quick-question` instead |
| Standard | All steps; independent sub-questions run as parallel investigate tasks |

## Steps

| # | Step | ROLE | Kind | Input → Output | Gate | Human |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Frame | orchestrator | orchestrate | request → question, scope, the decision it informs, the certainty needed | Question and scope written down | Confirms question and scope |
| 2 | Investigate | business-analyst (market, domain, competitive, user voice); software-architect (technical, technology selection) | research | frame → findings with sources | Primary sources first; facts separate from inference | — |
| 3 | Synthesise | business-analyst or software-architect | research | findings → report: question, scope, method, findings, conclusion, certainty, unknowns | Every key claim sourced | — |
| 4 | Check | evaluator | review | report → verdict | Sources say what is claimed; the conclusion follows; certainty stated | — |
| 5 | Deliver | orchestrator | orchestrate | report → short summary for the Human, with the link | — | Decides what to do with it |

## Step notes

1. Name the decision the research serves; it bounds the scope and the certainty needed.
2. Primary sources are the ones that own the information: official documentation, specifications, source code, first-party data. Follow every claim back to its owner; secondary write-ups only point the way.
3. A technology selection compares named candidates against criteria agreed in step 1, in a decision matrix with a recommendation.
4. A `fail` verdict returns to step 2 or 3 with the findings.

## Manual mode

The Human relays packets and handoffs; the report is saved where the target repo keeps such notes (`artifacts_path` once Kickoff has set it).
