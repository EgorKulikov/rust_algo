# Tester Scoring Support

## Goal

Teach the `tester` crate to track and report numeric scores per test, so
heuristic tasks (AHC-style) get aggregate stats (sum/mean/median/min/max/count)
without per-task boilerplate. The current `Result<(), String>` checker
contract is binary — pass/fail — and forces each scored task to reinvent
thread-local accumulation and custom summary printing.

## Non-Goals

- A new `TaskType::Heuristic`. Scoring is a capability of any classic or
  interactive task; the existing `Classic`/`Interactive` distinction is
  orthogonal.
- Richer per-test display rendered by the tester. Tasks that want detailed
  per-test info (turn count, error reason) keep using `eprintln!` from
  inside their checker, exactly as today.
- A custom score type. `Option<i64>` covers AHC and similar heuristic
  problems; we YAGNI a generic scoring trait.

## Checker Contract

```rust
type Checker     = fn(Input, Option<Input>, Input)        -> Result<Option<i64>, String>;
type Interactor  = fn(Input, Option<Input>, SolutionRunner) -> Result<Option<i64>, String>;
```

Semantics:

- `Ok(None)` — passes, no score. Used by all binary checkers
  (`default_checker`, `default_checker_eps_rel`, `default_checker_eps_abs`,
  `std_interactor`).
- `Ok(Some(n))` — passes with score `n`.
- `Err(s)` — wrong answer; halts the test set. Unchanged semantics.

A heuristic checker that wants to keep running after an invalid output
returns `Ok(Some(0))` and `eprintln!`s its diagnostic — same pattern as the
AHC064 check today.

## Outcome

```rust
pub enum Outcome {
    OK { duration: Duration, input_exhausted: bool, score: Option<i64> },
    TimeLimit    { duration: Duration, input_exhausted: bool },
    WrongAnswer  { checker_output: String, input_exhausted: bool },
    RuntimeError { panic_reason: String },
}
```

Only `OK` carries a score. The other variants represent something other
than "passed", so a score is meaningless.

## Per-Test Output

The existing `Test 12 ... OK` line gains a trailing bracketed score when
`score.is_some()`:

```
Test 12 ... OK [score=4920]
```

`OK` lines without a score render unchanged. Failure variants
(WrongAnswer/TimeLimit/RuntimeError) render unchanged.

## Test-Set Summary

After the existing `All N tests passed, max time elapsed: T` line, if any
test in the set produced `Some(_)`, print:

```
=== Scores: count=150 sum=739163 mean=4927.75 median=4928 min=4915 max=4938 ===
```

- Stats computed only over tests with `Some(_)` scores. Mixed sets
  (e.g., samples with `None` + generated with `Some`) report stats over the
  scored subset.
- If no test in the set produced a score, no extra line — old tasks look
  identical.
- Mean printed with two decimals; median is the lower middle for even
  counts; min/max are the extreme observed scores.

## Aggregation Storage

`Tester::test` already iterates tests; add a local `Vec<Option<i64>>`
collected as outcomes come back. Compute stats once at end-of-test-set.
**No thread-locals.** Each call to `test_samples()` / `test_generated()`
gets its own vector and its own summary, so multiple test-set calls don't
interfere.

## Migrations

### Tester crate

- `tester/src/lib.rs`
  - `Outcome::OK` — add `score: Option<i64>`.
  - `Runner::Classic` — `checker` field type updated.
  - `Runner::Interactive` — `interactor` field type updated.
  - `Tester::run_single_test` — propagate the score from checker into
    `Outcome::OK`.
  - `Tester::test` — accumulate `Vec<Option<i64>>` and pass to summary
    print at end.
- `tester/src/classic.rs`
  - `default_checker`, `default_checker_eps_rel`, `default_checker_eps_abs`:
    return `Ok(None)` instead of `Ok(())`.
  - `run_single_test_classic`: receives `Result<Option<i64>, String>`
    from the checker; on `Ok(score)` builds `Outcome::OK { score, ... }`.
- `tester/src/interactive.rs`
  - `std_interactor`: return `Ok(None)`.
  - `run_single_test_interactive`: same plumbing as classic.
- `tester/src/print.rs`
  - `end_test`: append `[score=X]` when score is `Some`.
  - `end_test_set`: receives the score vector; appends summary line if
    any score is `Some`.

### Archive sweep

Most archived tasks delegate to `default_checker`/`std_interactor` and need
no source change — their type updates flow from the lib change.

A handful have custom `check`/`interact` functions returning
`Result<(), String>`. Mechanical rewrite:
- `Result<(), String>` → `Result<Option<i64>, String>`
- `Ok(())` → `Ok(None)`
- Function bodies unchanged otherwise.

### AHC064 task

The point of the refactor — its `tester.rs` shrinks. After migration:

- Drop `RESULTS` and `NEXT_SEED` thread-locals.
- Drop `print_summary` and the `RunResult` struct.
- Drop the manual `tester.test_generated(...); print_summary(...)`
  sequence in `run_tests()`.
- `check()` becomes:

  ```rust
  fn check(mut input: Input, _: Option<Input>, mut output: Input)
      -> Result<Option<i64>, String>
  {
      let parsed_input = scoring::Input::read(&mut input);
      match scoring::parse_output(&parsed_input, &mut output) {
          Ok(turns) => {
              let (score, err, turns_to_goal) =
                  scoring::compute_score(&parsed_input, &turns);
              eprintln!("score={} turns={:?} err={}", score, turns_to_goal, err);
              Ok(Some(score))
          }
          Err(err) => {
              eprintln!("ERR: {}", err);
              Ok(Some(0))
          }
      }
  }
  ```

- `StressTest::input` no longer needs to advance a seed counter manually —
  the seed flows through `GeneratedTestSet::TestId` and the per-test print
  line already shows it.

### Templates

- `templates/tester.rs`: same Result type updates as archive sweep.

## Testing

- Build the entire workspace; existing archive tasks compile.
- Run `tester`'s own `cargo test` if any (no behavioral tests today; we
  rely on integration via task runs).
- Re-run AHC064 stress: should reproduce sum=739,163 (or current beam
  baseline) and emit the new `=== Scores: ... ===` line. Per-seed lines
  no longer printed by `tester.rs` are now printed by the simplified
  `check`.
- Run a non-scored task (any from the archive) end-to-end to confirm
  output is unchanged when no score is reported.

## Risk / Rollback

Mechanical migration; the failure modes are compile errors, not silent
behavior changes. A miss in the archive sweep produces a clear
`expected enum Result<Option<i64>, String>` error at the call site.
Rollback is `git revert` on the change set.
