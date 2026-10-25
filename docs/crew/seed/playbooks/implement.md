---
id: implement
version: 1
session_type: implement
output: "Change merged through a pull request, with evidence traceable to the story"
done_when:
  - "GitHub reports the pull request merged, with its merge_commit_sha"
  - "Every acceptance criterion is passed with an evidence reference"
  - "Evidence records the INPUT version, the commit, and the Model–Harness pair that produced and checked it"
not_done:
  - "An open pull request, a green CI run, or an approval of an older head"
  - "A session the Human closes before the merge"
pr: required
seed:
  - "Crew protocol: brief 4.3, 5.4.4, NEED 12, 14, 18, 21-22, 29, 32, 34-35, 37, 46-48; spec 9, 11; plan T5.5"
  - "specify-cli@1.2.3 core_pack/commands/analyze.md and get-shit-done-cc@1.42.3 agents/gsd-plan-checker.md: read-only pre-implementation analysis"
  - "mattpocock code-review and implement-spec (this repo: .devin/skills): integration branch, review against spec and standards"
  - "superpowers-mcp@6.4.7 skills/systematic-debugging: reproduce and find the root cause before fixing"
  - "dropped: Superpowers 5-round fix loop with model upgrade; compound autofix of review findings; any path that merges without the Human's approval on GitHub"
---

# Implement

**INPUT**: a Story Ready story, or for a small change or bugfix a minimal Story Ready (problem, expected behaviour, acceptance criteria, scope_out).
**OUTPUT**: the change merged, with evidence that traces to the story.

## Size

| Size | Path |
| --- | --- |
| Small (bugfix, one bounded change) | Steps 1, 4, 5, 6, 7, 8, 9. In step 1 the Orchestrator also checks that the single task covers every criterion and nothing more; that replaces steps 2 and 3. Independent evaluation, the pull request, and the Human's review stay. |
| Standard | All steps. |
| Large | Refine first: a story that is not Story Ready goes back to the `refine` playbook. |

## Steps

| # | Step | ROLE | Kind | Input → Output | Gate | Human |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Story Ready check | orchestrator | orchestrate | story → ready, or back to refine | Story Ready fields present; assumptions confirmed | Confirms open assumptions |
| 2 | Plan | tech-lead (planning) | architecture | story, architecture, conventions → task graph and packets | Each task has ROLE, criteria, scope_out, file_scope, verification, dependencies | — |
| 3 | Pre-implementation analysis | tech-lead (analysis, a different spawn from step 2) | review | story, plan, tasks → analysis report | Read-only; verdict `pass`, or `concerns` once the Human has decided each HIGH finding | Decides every HIGH finding |
| 4 | Implement each task | software-engineer (platform-engineer for build, CI, infrastructure) | implement · debug · ui | packet → diff and handoff | Handoff checks; project checks; fail-before/pass-after; test-tampering flags; secret scan | — |
| 5 | Integrate | Crew | — | worker branches → integration branch | Project checks re-run on the integrated result; conflicts become tasks | Decides conflicts that need a product call |
| 6 | Independent evaluation | evaluator; qa-engineer for test adequacy; security-engineer when a trust boundary is touched | review · test | criteria, diff, evidence → verdict | Evaluator is not an author; re-runs the evidence; UI is rendered | — |
| 7 | Pull request | Crew | — | integration branch → pull request under the App identity | Head verified | Reviews on GitHub |
| 8 | Review loop | software-engineer, then steps 5 to 7 | implement | review comments → updated pull request | Changed code re-verified and re-evaluated | Approves the latest head |
| 9 | Merge | Crew | — | approved head → merged | Merge only when the head equals the approved commit | — |

## Step notes

1. Unconfirmed assumptions go to the Human in this step. Missing criteria or scope_out, or an assumption the Human rejects, send the work back to refine. The Orchestrator never fills the gap.
2. Packets follow the Tech Lead's planning rules: vertical slices, interface-first, disjoint file_scopes for parallel tasks, abuse cases and rollback for persistent data, authentication, or trust boundaries.
3. The analysis worker changes no files; its report travels in its handoff. Implement tasks become ready only on `pass`, or on `concerns` once the Human has decided each HIGH finding. A `fail` goes back to step 2, or to refine when the story itself is at fault.
4. One worktree per task. For a bug, the first deliverable is a reproduction that fails on the base commit; the fix makes it pass. Three failed solutions to the same problem stop that task and wait for the Human.
5. Integration happens in a separate integration worktree, never in the Human's checkout. Passing branches individually does not prove the integrated result works; re-run the checks.
6. The evaluation packet holds the criteria, the diff and its base, and the author's evidence, never the author's reasoning transcript. A `fail` verdict with findings returns to step 4 with the findings as new acceptance criteria. A `fail` whose only obstacles are `infra:` items triggers an environment repair and a new evaluation; it is not rework and not an attempt.
7. The pull request body links the story, lists the evidence for each criterion, and names what was not verified.
8. Code changed after an approval must be verified and approved again. A closed pull request makes the session wait for the Human (new pull request, or close the session).
9. Done comes from the merge event, never from a green check.

## Detours

- **Design question** that conversation cannot settle: a prototype task on a `crew/prototype/<name>` branch (ui-designer or software-engineer) answers one stated question. It is throwaway evidence for the Human's decision, that decision is recorded, and no pull request is ever opened from that branch.
- **New work discovered**: insert a branch in the pipeline with its own criteria; anything outside the granted scope goes to the Human first.

## Manual mode

The Human performs the Crew steps and the Orchestrator gives the exact commands:

- Create one git worktree per task; relay packets and handoffs.
- Commit for the worker: stage only the files its handoff declares plus tracked files changed inside `file_scope`, and scan the staged diff for secrets (private keys, tokens, API keys) before committing.
- Run the fail-before/pass-after check for bug fixes (the new test on the base commit, then on the head) and give the output to the evaluator.
- Integrate (step 5) and open the pull request (step 7).
- Without the Crew GitHub App, the pull request is the Human's own, and GitHub does not let an author approve their own pull request. The Human reviews the diff, notes the head commit they reviewed, and merges only that head (for example `gh pr merge --match-head-commit <reviewed-sha>`); a push after the review means reviewing again.
