---
name: commit-contests
description: Commit uncommitted competitive-programming contests under archive/, one commit per contest, with message "(contest) <name>" — or, for the "Kattis" pseudo-contest, move into archive/Kattis/<date>/ and commit as "(archive) Kattis". Then push.
---

# Commit Contests

Use when the user wants to commit accumulated contest solutions sitting in
the working tree under `archive/`.

## Layout

Each contest lives at `archive/YYYY/MM/<date> - <contest name>/` and holds
one or more `.rs` files (one per problem). Contest name is everything in
the leaf directory after the `<date> - ` prefix. Example:

```
archive/2026/05/2026.05.18 - Educational Codeforces Round 190 (Rated for Div. 2)/
     a_.rs   b_.rs   c_.rs   ...
```

→ contest name: `Educational Codeforces Round 190 (Rated for Div. 2)`.

## Procedure

1. **Find candidate contests.** Run:
   ```bash
   git status --short --untracked-files=all 2>/dev/null \
     | grep -E '^(\?\?|A | M) "?archive/[0-9]{4}/[0-9]{2}/' \
     | sed -E 's|^[A-Z? ]+ "?(archive/[0-9]{4}/[0-9]{2}/[^/]+/).*|\1|' \
     | sort -u
   ```
   This yields each candidate contest directory exactly once, whether its
   files are untracked or staged. Empty result → nothing to do; tell the
   user and stop.

2. **Reset the index** so each contest commit can be staged in isolation:
   ```bash
   git reset --quiet
   ```
   This unstages everything but leaves the working tree untouched. Any
   non-contest changes that were previously staged stay in the working
   tree and will need re-staging by the user afterwards — mention this
   if `git status` shows tracked-file modifications.

3. **For each contest directory in order**, stage just that directory and
   commit. Two cases:

   **a. Regular contest** — name is anything other than exactly `Kattis`.
   Stage in place and commit with `(contest) <name>`:
   ```bash
   git add "archive/YYYY/MM/<date> - <name>/"
   git commit -m "(contest) <name>" --quiet
   ```

   **b. Kattis** — when `<name>` is exactly `Kattis`, the contents are
   loose problem solutions that belong in the long-lived
   `archive/Kattis/` pool, organized by date. Move first, then commit
   with `(archive) Kattis`:
   ```bash
   mkdir -p "archive/Kattis"
   # Move into archive/Kattis/<date>/ — `<date>` is the YYYY.MM.DD prefix
   # taken from the source directory name.
   git mv "archive/YYYY/MM/<date> - Kattis" "archive/Kattis/<date>"
   # If the source directory is untracked (not in the index), `git mv`
   # will refuse — fall back to a plain `mv` followed by `git add`:
   #   mv "archive/YYYY/MM/<date> - Kattis" "archive/Kattis/<date>"
   #   git add "archive/Kattis/<date>/"
   git commit -m "(archive) Kattis" --quiet
   ```

   Always quote paths — directory names contain spaces. The contest-name
   commit message uses the name verbatim, including any parenthetical
   suffix like `(Rated for Div. 2)`.

4. **Push.**
   ```bash
   git push
   ```

5. **Report.** Show the new commit hashes (one line each) and the push
   output. Example:
   ```
   e6af61e (contest) GP of Wulin
   15fa402 (contest) Educational Codeforces Round 190 (Rated for Div. 2)
   84f1c20 (archive) Kattis
   ```

## Notes and pitfalls

- **Never use `git add -A`** or `git add .` — those would sweep in
  unrelated changes (e.g., `Cargo.lock`, `config.toml`, in-progress edits
  in `algo_lib/`). Only the contest directory should be staged.
- **One contest per commit, always.** Even if the user has dozens of
  contests sitting in the working tree, each gets its own commit.
- The `archive/` tree is not part of the workspace (`Cargo.toml` only
  lists `algo_lib`, `main`, `tester`, `run`), so these commits won't
  affect the build. No need to run `cargo build` or tests.
- If a contest directory is empty (no `.rs` files), skip it and warn the
  user — that's almost certainly an accident.
- If the push fails (e.g., non-fast-forward), report the error and stop;
  don't attempt force-push.
