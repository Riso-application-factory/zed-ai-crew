---
id: kickoff
version: 1
session_type: kickoff
output: "Outer Harness foundation: reproducible baseline check, sources of truth, project-scope configuration with verification steps"
done_when:
  - "The baseline check, re-run from a clean worktree, gives the same result"
  - "The Human confirmed every assumption"
  - "Files added to the repository went through a merged pull request"
pr: when-adding-files
seed:
  - "Crew protocol: brief 4.3, 5.4.5, NEED 04, 19, 23, 27, 39; spec 5.1-5.3; plan T5.8"
  - "bmad-method@6.12.4 src/bmm-skills/plan/bmad-project-context/SKILL.md: verify each command claim; existing instructions are the baseline; never ask what a scan can answer; show the proposal before writing"
  - "get-shit-done-cc@1.42.3 agents/gsd-codebase-mapper.md: survey checklist"
  - "dropped: BMAD _bmad/ layout, resolver scripts and managed AGENTS.md block; GSD hooks"
---

# Kickoff

**INPUT**: a target repository that Crew has not worked on yet (onboarding finished: GitHub App connected, repo and permissions checked).
**OUTPUT**: the foundation later sessions inherit: a reproducible baseline check, the sources of truth, and the project-scope Crew configuration.

## Size

The first Kickoff of a project runs every step. A later Kickoff (refresh) re-runs steps 1, 3, and 4 against what changed and proposes a diff.

## Steps

| # | Step | ROLE | Kind | Input → Output | Gate | Human |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Survey how the project runs | platform-engineer | research | repository → commands with recorded results, environment needs, network needs | Read-only; every command run from a clean worktree | — |
| 2 | Survey structure and sources of truth | software-architect | architecture | repository → component map, trust boundaries, Guides (`AGENTS.md`, `CLAUDE.md`, glossary, ADRs), how Issues are used | Read-only | — |
| 3 | Propose project configuration | orchestrator | orchestrate | surveys → proposal for the `crew` key: verification baseline and steps (network per step), permissions (merge, release, environments), GitHub repository and owner type, network allowlist inside the user-scope ceiling, `artifacts_path` | Values stay within user-scope ceilings | Confirms each assumption and value |
| 4 | Prove the baseline is reproducible | platform-engineer | test | confirmed baseline → two runs from clean worktrees | Results identical; any difference explained | Decides on flaky steps |
| 5 | Add the files | platform-engineer | implement | confirmed proposal → diff (`.zed/settings.json` `crew` key, plus any harness files the Human agreed to) | Project checks | — |
| 6 | Independent evaluation | evaluator | review | diff and survey evidence → verdict | Evaluator is not the author | — |
| 7 | Pull request and merge | Crew | — | branch → merged pull request | Merge only on the approved head | Reviews and approves on GitHub |

## Step notes

1. Read manifests, scripts, and CI before running anything; record the exact invocation, exit code, and duration of each command. Secret files are never opened; secrets are recorded by name and source.
2. Existing instructions in the repo are the baseline to build on, never raw material to discard. Note contradictions between instruction files.
3. Present the proposal before anything is written. Ask only what no survey can answer: what is off limits, what a domain term means, why a constraint exists. Prefer a check over a written rule wherever a check can enforce it.
4. A step whose result differs between clean runs is not part of the baseline until it is stabilised or the Human accepts it as flaky.
5. Configuration keys follow spec 5.2. Project-scope keys only narrow user-scope settings; a key that tries to widen them is dropped with a warning.

## Manual mode

The Human creates the clean worktrees, relays packets and handoffs, opens the pull request, and merges after approving on GitHub.
