---
name: zed-rebase
description: >-
  Use when the user wants to sync this fork with upstream Zed: fetch
  upstream/main and rebase main (origin/main) onto it, resolve conflicts, and
  optionally push the result to origin.
---

# Rebase onto upstream Zed

This repo is a fork. Remotes:

- `origin`: the fork (`Riso-application-factory/zed-ai-crew`)
- `upstream`: `zed-industries/zed` (push is disabled)

The goal is to replay the fork's own commits on top of `upstream/main`.

## Steps

1. **Preflight**
   - Run `git status --porcelain`. If the tree is dirty, stop and ask the user whether to stash or commit first.
   - Run `git remote -v` and confirm `upstream` points at `zed-industries/zed`. If it is missing, ask before adding it.
   - Note the current branch with `git branch --show-current`. Unless the user named another branch, the target is `main`.

2. **Fetch**
   - `git fetch upstream main`
   - `git fetch origin`

3. **Inspect before rebasing**
   - Fork-only commits: `git log --oneline upstream/main..origin/main`
   - New upstream commits: `git rev-list --count origin/main..upstream/main`
   - Tell the user how many commits will be replayed and how many upstream commits are incoming. If there is nothing new upstream, report that and stop.

4. **Back up**
   - Create a backup ref so the rebase is recoverable: `git branch backup/<branch>-pre-rebase-$(date +%Y%m%d-%H%M%S) <branch>`

5. **Rebase**
   - `git checkout <branch>` if needed, then `git rebase upstream/main`.
   - On conflicts: list them with `git diff --name-only --diff-filter=U`, resolve each file by reading both sides, `git add` the file, then `git rebase --continue`. Never use `git rebase --skip` or `-X ours/theirs` without asking. If a conflict is ambiguous, ask the user.
   - If the rebase becomes unmanageable, `git rebase --abort` and report.

6. **Verify**
   - `cargo check --workspace` (or a narrower `-p` set if the user prefers) to confirm the result builds.
   - Show `git log --oneline upstream/main..HEAD` so the user can see the fork commits sit on top.

7. **Push (requires explicit confirmation)**
   - Rewriting `origin/<branch>` is outward-facing and hard to reverse. Ask the user first, then run:
     `git push --force-with-lease origin <branch>`
   - Never use plain `--force`, and never push to `upstream`.

## Report

Summarize: upstream commits pulled in, fork commits replayed, conflicts resolved (files and how), build result, backup branch name, and whether the push happened.
