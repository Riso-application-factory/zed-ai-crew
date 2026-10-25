# Crew seed: ROLE and playbook drafts

| Field | Content |
| --- | --- |
| Status | Draft 1 · 2026-10-25. No code loads these files yet. |
| Feeds | T2.4 (Orchestrator ROLE), T5.2 (ROLE catalogue), T5.3 (playbooks), T5.5 (pre-implementation analysis) in `docs/crew/plan/` |
| Source rules | Brief 5.3.1 seed map and NEED 27: take content (prompts, templates, criteria), not mechanism (hooks, slash commands, harness-specific tools) |
| Human view | `index.html` in this directory (Vietnamese) |

## What this is

Draft content for the ROLE catalogue and the playbooks, written before the loader exists. When T2.4, T5.2, and T5.3 run, they move these files to `crates/crew/assets/roles/` and `crates/crew/assets/playbooks/`, adapt them to the T5.1 loader, and check them against their acceptance criteria and an independent review. Until then the Human can use them by hand (manual mode, below), which also exercises the packet and handoff contracts of spec 9.3 and 9.4 before any code depends on them.

## Files

| ROLE | Title | task_kinds | Main seed |
| --- | --- | --- | --- |
| `orchestrator` | Orchestrator | orchestrate | Crew protocol; mattpocock grilling, wayfinder; Superpowers brainstorming; GSD planner |
| `business-analyst` | Business Analyst | research, docs | Written new; mattpocock research |
| `product-manager` | Product Manager | docs, research | BMAD PM, PRD rubric; OpenSpec delta spec |
| `backlog-analyst` | Backlog Analyst | docs, review | BMAD epics and stories, readiness gate, correct course; mattpocock to-tickets |
| `ux-designer` | UX Designer (researcher and designer) | ui, research, docs | BMAD UX; mattpocock prototype |
| `ui-designer` | UI Designer | ui, docs | BMAD UX validation; mattpocock prototype |
| `software-architect` | Software Architect | architecture, research, docs | BMAD architect, architecture spine, reviewer gate |
| `tech-lead` | Tech Lead | architecture, review | GSD planner, plan checker; spec-kit analyze; mattpocock to-tickets |
| `software-engineer` | Software Engineer | implement, debug, test, ui, research | BMAD dev; compound worker contract; Superpowers TDD, verification, debugging |
| `qa-engineer` | QA Engineer | test, review | compound testing reviewer; BMAD verification gap; GSD verifier |
| `security-engineer` | Security Engineer | review, architecture | GSD security auditor, SECURITY.md template |
| `evaluator` | Evaluator (independent) | review | mattpocock code-review; compound ce-code-review, validator; BMAD claims check |
| `delivery-manager` | Delivery Manager | docs, review, research | Written new; mattpocock retro |
| `release-manager` | Release Manager | docs, review, test | Written new |
| `platform-engineer` | Platform / DevOps Engineer | implement, test, debug, research, docs | Written new; BMAD project context; GSD codebase mapper |
| `sre` | Site Reliability Engineer | debug, test, docs | wshobson incident-responder, postmortem-writing |

| Playbook | Session type (brief 4.3) |
| --- | --- |
| `quick-question` | Hỏi nhanh |
| `kickoff` | Kickoff |
| `brainstorm` | Brainstorm |
| `research` | Research |
| `refine` | Refine |
| `implement` | Implement |
| `release-migration` | Release / migration |
| `incident` | Incident |
| `review-retro` | Review / retro |
| `retirement` | Retirement |

## Sources read

Package versions are the ones read on 2026-10-25. GitHub was not reachable from the drafting environment, so sources came from package registries, from skills vendored in this repo, and, for wshobson, from a summarising web fetch.

| Source | Where it was read |
| --- | --- |
| BMAD | npm `bmad-method@6.12.4`, `src/bmm-skills/` |
| GSD | npm `get-shit-done-cc@1.42.3`, `agents/`, `get-shit-done/templates/` |
| spec-kit | PyPI `specify-cli@1.2.3`, `specify_cli/core_pack/commands/analyze.md` |
| compound-engineering | npm `@every-env/compound-plugin@3.8.3`, `plugins/compound-engineering/` |
| Superpowers | npm `superpowers-mcp@6.4.7` (an MIT repackaging of obra/superpowers), `skills/` |
| OpenSpec | npm `@fission-ai/openspec@1.15.0`, `schemas/spec-driven/` |
| mattpocock skills | This repo, `.devin/skills/` |
| wshobson agents | `plugins/incident-response/agents/incident-responder.md` and `skills/postmortem-writing/SKILL.md`, through a summarising fetch, not verbatim |

## ROLE format

Front matter follows spec 5.4. Conventions in these drafts:

- `allowed: { providers: ["*"], efforts: ["*"] }` means no narrowing beyond user scope. Narrowing a ROLE is the Human's decision.
- `tools_deny: []` everywhere. Read-only behaviour (Orchestrator, analysis, evaluation) is stated in the body and enforced through mode and gate (spec 7.5, 10.1), because tool names differ per provider.
- `handoff_schema`: `crew.handoff/1` for workers (spec 9.4); `crew.verdict/1` for the evaluator (the shape of spec 11.1; the id is new); `none` for the Orchestrator, which reports through `crew_propose_done`.
- `seed`: each source with package, version, and path, then `dropped:` entries naming the mechanisms removed (brief risk R8), and `not found:` entries for named sources that do not exist.
- Every ROLE that hands off with `crew.handoff/1` ends with the same "Crew worker rules" and "Handoff" sections, because each ROLE is loaded alone (NEED 55). Change them in every file at once.

## Playbook format

There is no playbook schema in the spec yet; these drafts use:

- Front matter: `id`, `version`, `session_type`, `output`, `done_when` (brief 4.3 and spec 11.2), optional `not_done`, `pr` (`required`, `when-adding-files`, `via-implement`, `none`), `seed`.
- Body: INPUT and OUTPUT, a size table (fast path), a steps table (step, ROLE, task kind, input → output, gate, Human decision), step notes for what each task packet must carry, and manual-mode notes.
- `Crew` in the ROLE column marks deterministic work done by Crew tools (integration, pull requests, merge, publishing Issues), not a ROLE.

## Manual mode

Before Crew exists, the Human plays the part of the Crew tools:

1. Open a thread with any provider and paste `roles/orchestrator.md`, then the request.
2. The Orchestrator classifies the session, asks rounds of questions, and proposes the pipeline from the playbook (paste `playbooks/<id>.md` if it asks).
3. For each task it prints `Task <id> → ROLE <role-id>` and a packet. Open a new thread, paste the ROLE file and the packet; give code tasks their own git worktree.
4. The worker ends with a `crew.handoff/1` block. Paste it back into the Orchestrator thread.
5. Evaluation runs in a fresh thread with `roles/evaluator.md` and the evaluation packet.
6. Commits, pull requests, and merges are done by the Human, following the Orchestrator's steps: stage only the files the handoff declares, scan the staged diff for secrets, and merge only the head that was reviewed. Without the Crew GitHub App the pull request is the Human's own, and GitHub does not let an author approve it, so the reviewed-head rule (for example `gh pr merge --match-head-commit <sha>`) stands in for the approval of NEED 14.

## Gaps found while drafting

1. **Task kinds.** Spec 13 has no kind for release or rollout, operations, security review, or product discovery. The drafts map such work to the nearest of the nine kinds; T5.3 decides whether to add kinds.
2. **Read-only workers.** The analysis worker (T5.5) and the evaluator must not edit files, but no field tells the gate that a worker is read-only. Decide in T5.1 or T5.5: a ROLE field, or a packet constraint the gate reads. Such a worker also cannot write a report file, and spec 9.4 checks that every output location exists, so the drafts put the analysis report inside the handoff (`summary`); a dedicated report field may fit better.
3. **Verdict schema id.** Spec 11.1 gives the evaluator's verdict shape without an id; the drafts call it `crew.verdict/1`.
4. **`problem_id` in the packet.** The handoff has `attempt_of_problem` and `crew_tasks` has `problem_id`, but the packet of spec 9.3 has no field to carry it to the worker. The Orchestrator's manual-mode packet adds `problem_id`; spec 9.3 should add it too.
5. **BMAD ticket template.** Brief 5.3.1 names `ticket-template.md` and `github-ticketing.toml`; neither exists in `bmad-method@6.12.4`. The story format comes from `bmad-create-epics-and-stories`. Check another BMAD version or module during T5.2.
6. **wshobson.** The SRE and incident drafts rest on a summarising fetch; re-read the original files during T5.2. The "ship-mate" source named in brief 5.3.1 was not located; Release Manager is written new, as the brief requires anyway.
7. **Artifact states.** Brief Q6 is open; the drafts use the tentative states of spec 4.3.

## Checks run on these drafts

- Front matter parses; every `id` matches its file name; every task kind is one of the nine in spec 13.
- Playbook steps reference only existing ROLE ids, and each step's task kind is one the assigned ROLE declares; the Orchestrator's session table names only existing playbooks.
- The shared worker sections are byte-identical across the 14 ROLEs that use them.
- JSON blocks match spec 9.4 (handoff keys), spec 9.3 (packet keys, plus `problem_id`), and spec 11.1 (verdict keys and finding categories). Every `crew_*` tool named exists in the spec.
- No ROLE or playbook body contains the removed mechanisms (self-approval, auto-merge, push to main, permission-bypass modes, `allow_always`, flag-file approval, model upgrades on retry); they appear only in `dropped:` provenance lines.

- An independent reviewer with fresh context checked the Orchestrator, Software Engineer, Evaluator, and Tech Lead ROLEs and the implement and refine playbooks against the spec and the brief. It reported 15 problems (3 high: manual-mode merge impossible without self-approval, the evaluator asked to run a base-commit check it cannot run, infra failures turned into rework); all 15 are fixed in this draft.

Not done yet: running any ROLE on a real provider. That, and the independent verification of each task, belong to T5.2 and T5.3.
