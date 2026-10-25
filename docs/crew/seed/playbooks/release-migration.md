---
id: release-migration
version: 1
session_type: release-migration
output: "The change active in the target environment"
done_when:
  - "The rollout completed within the permissions the project granted"
  - "Post-deployment checks pass"
  - "The rollback is confirmed still usable"
pr: via-implement
seed:
  - "written new: none of the 13 seed repositories has a release or migration workflow (brief 5.3.1)"
  - "Crew protocol: brief 4.3, NEED 06, 14, 16, 44"
---

# Release / migration

**INPUT**: merged changes to release, or a migration to run, in a named environment.
**OUTPUT**: the change active in that environment, verified, with a rollback that still works.

Code changes reach this playbook already merged through `implement`. Without the project's permission for the environment, the work waits for the Human; merge permission never implies release permission.

## Size

| Size | Path |
| --- | --- |
| Small (configuration or feature-flag change, no data change) | Steps 1, 2, 4, 5, 6, 7; skip step 3 |
| Standard | All steps |

## Steps

| # | Step | ROLE | Kind | Input → Output | Gate | Human |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Permission and scope | orchestrator | orchestrate | request → action, environment, project permission, merged change set | Permission present, otherwise wait | Grants a missing permission in the project configuration |
| 2 | Release readiness | release-manager | docs | change set → readiness report, release notes, checks on the exact artifact | Checks pass on the artifact to be released | — |
| 3 | Migration plan, when data changes | platform-engineer | implement | schema or data change → expand-contract plan, backup and restore check, dry run | Dry-run result recorded; reversibility stated | — |
| 4 | Rollout plan | release-manager | docs | readiness and migration plan → stages, checks per stage, stop conditions, rollback per stage with proof it works | Rollback proof present | Approves the plan |
| 5 | Execute stage by stage | release-manager, platform-engineer | implement | approved plan → executed stages, each action recorded | Stop conditions honoured; within permission | Approves each gated stage |
| 6 | Post-deployment verification | sre | test | → check results compared with the baseline | All checks pass | — |
| 7 | Confirm rollback | release-manager | review | → evidence that the rollback mechanism still works after the rollout | Evidence present | — |

## Step notes

2. Release notes come from the merged changes: user-facing changes, fixes, breaking changes, and actions users must take.
3. Expand, migrate, contract: the old and new shapes coexist until every consumer has moved, so each stage can be rolled back.
4. A rollback states its limits, for example data written after the change that a rollback cannot remove.
5. Every action is recorded as it happens. After an interruption, establish which actions already took effect before continuing; nothing is replayed blindly.
6. A failed check triggers the stop condition of the stage and the rollback decision, which belongs to the Human unless the approved plan says otherwise.

## Manual mode

The Human holds the environment credentials and runs or supervises each stage; the workers prepare plans, commands, and checks.
