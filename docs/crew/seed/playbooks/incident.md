---
id: incident
version: 1
session_type: incident
output: "Service restored and an incident record"
done_when:
  - "The original symptom no longer occurs when re-checked the same way it was observed"
  - "The record has a timeline, the reproduced cause, and the impact"
  - "Prevention work is opened as stories"
pr: via-implement
seed:
  - "Crew protocol: brief 4.3, NEED 06, 15, 37, 38, 48"
  - "wshobson/agents plugins/incident-response (incident-responder agent, postmortem-writing skill; read through a summarising fetch): triage, stabilise, investigate, recover, blameless postmortem"
  - "superpowers-mcp@6.4.7 skills/systematic-debugging: root cause before fix"
  - "dropped: external status communication, incident-command staffing, response-time SLAs, approval by flag file"
---

# Incident

**INPUT**: a report that something in a running environment is broken or degraded.
**OUTPUT**: the service restored, and a record that explains what happened and what will prevent it.

Environment actions need the project's permission; anything outside it waits for the Human with a recommendation. Code fixes go through `implement`.

## Size

| Size | Path |
| --- | --- |
| Small (SEV-3 or SEV-4, no environment action needed) | Steps 1, 3, 4, 5, 6 with a short record, then 7 |
| Standard | All steps |

## Steps

| # | Step | ROLE | Kind | Input → Output | Gate | Human |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Triage | sre | debug | report → severity, impact, scope | Evidence for the severity | Sets priority |
| 2 | Stabilise | sre (platform-engineer for infrastructure actions) | debug | → mitigation inside permissions, each action recorded | Permitted actions only | Approves actions outside permission |
| 3 | Reproduce and find the root cause | sre with software-engineer | debug | → a reproduction that fails before the fix, the root cause with evidence | Product failure separated from infra noise | — |
| 4 | Fix | an `implement` session, linked | implement | → merged fix | Implement gates | Reviews the pull request |
| 5 | Verify recovery | sre | test | → the original symptom re-checked the same way: gone | Evidence | — |
| 6 | Incident record | sre | docs | → timeline, impact, reproduced cause, contributing factors, action items | Blameless; every statement evidenced | — |
| 7 | Prevention stories | backlog-analyst | docs | action items → proposed stories (minimal Story Ready) | — | Chooses which to open |

## Step notes

1. Severity: SEV-1 complete outage or security breach; SEV-2 major degradation; SEV-3 limited impact; SEV-4 cosmetic.
2. Correlate with recent deployments, configuration, dependency, and infrastructure changes first. Rolling back a recent change is often the fastest safe mitigation when the permissions allow it.
3. Three failed solutions to the same problem stop that line of work and wait for the Human; other branches continue.
5. A different check passing is not recovery; the original symptom must be re-checked.
6. Action items are classified as prevention, detection, or mitigation; ask what conditions allowed the failure, never who caused it.

## Manual mode

The Human runs or supervises every environment action and relays packets and handoffs; the SRE worker prepares the commands and the record.
