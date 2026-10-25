---
id: quick-question
version: 1
session_type: quick-question
output: "An answer to the Human's question"
done_when:
  - "The answer addresses the question that was asked"
  - "Every fact cites its source (path:line, command and output, or document)"
  - "Inference is labelled as inference"
pr: none
seed:
  - "written new: none of the 13 seed repositories has a workflow for quick questions (brief 5.3.1)"
  - "Crew protocol: brief 4.3, NEED 10, 18, 49"
---

# Quick question

**INPUT**: a question about the project, its code, its history, or its decisions.
**OUTPUT**: an answer.

## Size

A quick question is small by definition. When answering it needs a change, a document, or decisions, say so and propose the matching session type instead (research, refine, implement).

## Steps

| # | Step | ROLE | Kind | Input → Output | Gate | Human |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Restate the question | orchestrator | orchestrate | request → the question as understood | — | Corrects a misreading |
| 2 | Gather facts | orchestrator; a read-only worker (software-engineer or software-architect) when commands are needed | research | question → facts with sources | Read-only | — |
| 3 | Answer | orchestrator | orchestrate | facts → answer | Each fact sourced; inference labelled | — |

## Step notes

1. Restate only when the question is ambiguous; otherwise answer directly.
2. Delegate only when a fact needs a command or a long search. The packet's constraints say read-only (no file changes) and the deliverable is a short report with sources.
3. Lead with the answer, then the evidence. Separate what was verified now from recall and inference, and name what could not be checked.

## Manual mode

The Orchestrator answers in its own thread. When it needs command output it cannot get itself, it asks the Human for the exact command and output.
