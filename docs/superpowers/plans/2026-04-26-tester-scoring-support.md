# Tester Scoring Support Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Teach the `tester` crate to track and report numeric scores per test, so heuristic tasks (AHC-style) get aggregate stats (sum/mean/median/min/max/count) without per-task boilerplate.

**Architecture:** Mechanical type-system migration — checker/interactor signature changes from `Result<(), String>` to `Result<Option<i64>, String>`, `Outcome::OK` gains a `score: Option<i64>` field, `Tester::test` accumulates a per-test-set score vector, and `print.rs` renders inline per-test scores plus an end-of-set summary line. All built-in checkers (`default_checker`, `default_checker_eps_*`, `std_interactor`) become score-less by returning `Ok(None)`.

**Tech Stack:** Rust workspace (cargo). Crates touched: `tester` (lib + classic + interactive + print). Files touched outside the crate: `templates/tester.rs`, `tasks/a_non_crossing_railcar_rearrangement/src/{tester.rs,main.rs}` (through cascading sig changes — main.rs is unaffected).

**Reference spec:** `docs/superpowers/specs/2026-04-26-tester-scoring-support-design.md`

---

### Task 1: Type-system migration

Migrate the checker/interactor signatures and `Outcome::OK` in one cohesive change. Adds the `score` field but doesn't yet *render* it — that's task 2. After this task the workspace compiles end-to-end with all built-in checkers returning `Ok(None)` and the AHC064 task's custom checker updated mechanically.

**Files:**
- Modify: `tester/src/lib.rs`
- Modify: `tester/src/classic.rs`
- Modify: `tester/src/interactive.rs`
- Modify: `tester/src/print.rs`
- Modify: `templates/tester.rs`
- Modify: `tasks/a_non_crossing_railcar_rearrangement/src/tester.rs`

- [ ] **Step 1: Update `Outcome::OK` and `Runner` field types in `tester/src/lib.rs`**

In `tester/src/lib.rs`, change `Outcome::OK` to carry a score, and change both `Runner` variants to use the new return type:

```rust
pub enum Outcome {
    OK {
        duration: Duration,
        input_exhausted: bool,
        score: Option<i64>,
    },
    TimeLimit {
        duration: Duration,
        input_exhausted: bool,
    },
    WrongAnswer {
        checker_output: String,
        input_exhausted: bool,
    },
    RuntimeError {
        panic_reason: String,
    },
}

pub enum Runner {
    Classic {
        checker: fn(Input, Option<Input>, Input) -> Result<Option<i64>, String>,
    },
    Interactive {
        interactor: fn(Input, Option<Input>, SolutionRunner) -> Result<Option<i64>, String>,
    },
}
```

Also update both constructors' `checker` / `interactor` parameter types to match (in `Tester::new_classic` and `Tester::new_interactive`). The pattern-match in `Tester::test` that reads `Outcome::OK { duration, .. }` already uses `..` so it's unaffected.

- [ ] **Step 2: Update `tester/src/classic.rs`**

Two changes — the three default checker variants return `Ok(None)`, and `run_single_test_classic`'s checker call captures the score and threads it into `Outcome::OK`.

In the function signatures, change `Result<(), String>` to `Result<Option<i64>, String>`:

```rust
pub fn default_checker(
    _input: Input,
    expected: Option<Input>,
    mut actual: Input,
) -> Result<Option<i64>, String> {
    // ... existing body unchanged except final Ok(()) → Ok(None) ...
    Ok(None)
}
```

Same change for `default_checker_eps_rel` and `default_checker_eps_abs` and the inner `default_checker_eps` helper. The inner helper's `Result<(), String>` is purely for delegation; updating it to `Result<Option<i64>, String>` is fine as long as both call sites (the rel/abs variants) receive the result and return it.

In `run_single_test_classic`, update the parameter type and capture the score:

```rust
pub(crate) fn run_single_test_classic<T: TestSet>(
    tester: &Tester,
    checker: fn(Input, Option<Input>, Input) -> Result<Option<i64>, String>,
    input: &[u8],
    expected: Option<&[u8]>,
    test_set: &T,
    test_id: &T::TestId,
) -> Outcome {
    // ... existing setup ...
    let checker_result = (checker)(
        Input::slice(input),
        expected.map(Input::slice),
        Input::slice(output.as_slice()),
    );
    if let Err(checker_output) = checker_result {
        (
            Outcome::WrongAnswer {
                input_exhausted: is_exhausted,
                checker_output,
            },
            output,
        )
    } else if duration.as_millis() as u64 > tester.time_limit {
        (
            Outcome::TimeLimit {
                duration,
                input_exhausted: is_exhausted,
            },
            output,
        )
    } else {
        let score = checker_result.unwrap();
        (
            Outcome::OK {
                duration,
                input_exhausted: is_exhausted,
                score,
            },
            output,
        )
    }
    // ... rest unchanged ...
}
```

Note: the `checker_result.unwrap()` is safe because the `if let Err` branch above handled the error case.

- [ ] **Step 3: Update `tester/src/interactive.rs`**

Update `std_interactor`'s return type to `Result<Option<i64>, String>` and its final `Ok(())` to `Ok(None)`:

```rust
pub fn std_interactor(
    _sol_input: Input,
    _expected: Option<Input>,
    mut runner: SolutionRunner,
) -> Result<Option<i64>, String> {
    // ... body unchanged ...
    Ok(None)
}
```

Update `run_single_test_interactive`'s parameter type and the success branch to thread the score:

```rust
pub(crate) fn run_single_test_interactive(
    tester: &Tester,
    interactor: fn(Input, Option<Input>, SolutionRunner) -> Result<Option<i64>, String>,
    input: &[u8],
    expected: Option<&[u8]>,
    print_details: bool,
) -> Outcome {
    // ... existing setup ...
    match std::panic::catch_unwind(|| {
        // ... runner setup ...
        interactor(Input::slice(input), expected.map(Input::slice), runner)
    }) {
        Ok(res) => match res {
            Ok(score) => {
                let duration = start.elapsed();
                if duration.as_millis() as u64 > tester.time_limit {
                    Outcome::TimeLimit {
                        duration,
                        input_exhausted: true,
                    }
                } else {
                    Outcome::OK {
                        duration,
                        input_exhausted: true,
                        score,
                    }
                }
            }
            Err(err) => Outcome::WrongAnswer {
                checker_output: err,
                input_exhausted: false,
            },
        },
        Err(err) => process_error(err),
    }
}
```

- [ ] **Step 4: Update `tester/src/print.rs` to accept the new `Outcome::OK` shape**

In `end_test`, the `Outcome::OK` arm currently destructures `{ duration, input_exhausted }`. Change to `{ duration, input_exhausted, score: _ }` for now — score rendering is task 2:

```rust
Outcome::OK {
    duration,
    input_exhausted,
    score: _,
} => {
    // ... existing body unchanged ...
}
```

- [ ] **Step 5: Update `templates/tester.rs`**

Three custom-helper return types need updating. In `templates/tester.rs`, change each function signature and final `Ok(())`:

```rust
fn interact(mut input: Input, expected: Option<Input>, mut runner: SolutionRunner) -> Result<Option<i64>, String> {
    let (mut sol, mut out) = runner.run();
    Ok(None)
}

fn run_twice(
    mut input: Input,
    expected: Option<Input>,
    mut runner: SolutionRunner,
) -> Result<Option<i64>, String> {
    // ... body unchanged through the existing default_checker call ...
    default_checker(Input::slice(&input_vec), expected, Input::slice(&ans))
    // (default_checker now returns Result<Option<i64>, String>, so direct return works)
}

fn check(mut input: Input, expected: Option<Input>, mut output: Input) -> Result<Option<i64>, String> {
    Ok(None)
}
```

(The `run_twice` body's `default_checker(...)` direct return is correct because we updated `default_checker`'s type in step 2.)

- [ ] **Step 6: Update `tasks/a_non_crossing_railcar_rearrangement/src/tester.rs` checker signature**

The task's `check` function currently returns `Result<(), String>`. Update its signature only — body simplifications come in task 4. For now, change `Result<(), String>` to `Result<Option<i64>, String>` and the final `Ok(())` to `Ok(None)`:

```rust
fn check(mut input: Input, _: Option<Input>, mut output: Input) -> Result<Option<i64>, String> {
    // ... existing body which manipulates RESULTS / NEXT_SEED is preserved ...
    // ... change final `Ok(())` to `Ok(None)` ...
    Ok(None)
}
```

The thread-locals and `print_summary` stay for now — task 4 cleans them up.

- [ ] **Step 7: Verify the workspace builds**

Run: `cargo build -p tester && cargo build -p a_non_crossing_railcar_rearrangement`

Expected: both succeed with no errors. (Warnings about unused variables/imports are OK.)

- [ ] **Step 8: Commit**

```bash
git add tester/ templates/tester.rs tasks/a_non_crossing_railcar_rearrangement/src/tester.rs
git commit -m "feat(tester): score-bearing checker signature

Outcome::OK gains a score: Option<i64> field; checker and interactor
contracts switch from Result<(), String> to Result<Option<i64>, String>.
Built-in checkers (default_checker, default_checker_eps_*, std_interactor)
return Ok(None). Per-test rendering and aggregate scoring are added in
follow-up commits."
```

---

### Task 2: Per-test score display

Render `[score=X]` on per-test OK lines when a score is present.

**Files:**
- Modify: `tester/src/print.rs`

- [ ] **Step 1: Update `end_test` to print `[score=X]`**

In `tester/src/print.rs`, replace the `Outcome::OK` arm of `end_test` with score-aware rendering. The detailed branch (`print_details == true`) and the compact branch both gain a trailing `[score=X]` when score is `Some`:

```rust
Outcome::OK {
    duration,
    input_exhausted,
    score,
} => {
    let score_suffix = match score {
        Some(s) => format!(" [score={}]", s),
        None => String::new(),
    };
    if print_details {
        print!(
            "{}Time elapsed: {:.3}s",
            BLUE,
            (duration.as_millis() as f64) / 1000.,
        );
        println!("{}", DEF);
        if !input_exhausted {
            println!("{}Input not exhausted{}", RED, DEF);
        }
        println!("{}Verdict: {}OK{}{}", BLUE, GREEN, score_suffix, DEF);
    } else {
        println!("{}OK{}{}", GREEN, score_suffix, DEF);
    }
}
```

- [ ] **Step 2: Verify the workspace builds**

Run: `cargo build -p tester`

Expected: succeeds.

- [ ] **Step 3: Commit**

```bash
git add tester/src/print.rs
git commit -m "feat(tester): show [score=X] on passed tests with a score"
```

---

### Task 3: Test-set score summary

Accumulate per-test scores in `Tester::test`, compute aggregate stats (sum/mean/median/min/max/count), and print a summary line when at least one test in the set produced a score.

**Files:**
- Modify: `tester/src/lib.rs`
- Modify: `tester/src/print.rs`

- [ ] **Step 1: Add a stats-printing helper and update `end_test_set` signature in `tester/src/print.rs`**

Add this helper (above `end_test_set`):

```rust
fn print_score_summary(scores: &[i64]) {
    if scores.is_empty() {
        return;
    }
    let count = scores.len();
    let sum: i64 = scores.iter().sum();
    let mean = sum as f64 / count as f64;
    let mut sorted: Vec<i64> = scores.to_vec();
    sorted.sort_unstable();
    let median = sorted[count / 2];
    let min = *sorted.first().unwrap();
    let max = *sorted.last().unwrap();
    println!(
        "{}=== Scores: count={} sum={} mean={:.2} median={} min={} max={} ==={}",
        BLUE, count, sum, mean, median, min, max, DEF
    );
}
```

Update `end_test_set` to accept the score vector and print the summary at the end (the existing pass/fail line stays unchanged):

```rust
pub(crate) fn end_test_set(
    test_failed: usize,
    test_total: usize,
    max_time: Duration,
    scores: &[Option<i64>],
) {
    if test_failed == 0 {
        println!(
            "{}All {}{}{} tests passed, max time elapsed: {:.3}s{}",
            BLUE,
            GREEN,
            test_total,
            BLUE,
            max_time.as_secs_f64(),
            DEF
        );
    } else {
        println!(
            "{}{}/{}{} tests failed{}",
            RED, test_failed, test_total, BLUE, DEF
        );
    }
    let scored: Vec<i64> = scores.iter().filter_map(|s| *s).collect();
    print_score_summary(&scored);
}
```

- [ ] **Step 2: Accumulate scores in `Tester::test` in `tester/src/lib.rs`**

Add a `Vec<Option<i64>>` to track scores, push to it when `Outcome::OK` comes back, and pass it to `end_test_set`. Modify the `test` method:

```rust
fn test<T: TestSet>(&self, test_set: T) -> bool {
    let mut test_failed = 0usize;
    let mut test_total = 0usize;
    print::start_test_set(test_set.name());
    let mut max_time = Duration::default();
    let mut scores: Vec<Option<i64>> = Vec::new();
    for test in test_set.tests() {
        test_total += 1;
        let input = test_set.input(&test);
        let expected = test_set.output(&test, &input);
        print::start_test(
            &test,
            self.trim(&input),
            expected.as_ref().map(|v| self.trim(v.as_slice())),
            test_set.print_details(),
        );
        let outcome = self.run_single_test(&input, expected.as_deref(), &test_set, &test);
        if let Outcome::OK { duration, score, .. } = outcome {
            max_time = max_time.max(duration);
            scores.push(score);
        } else {
            test_failed += 1;
            if !test_set.print_details() {
                print::start_test(
                    &test,
                    self.trim(&input),
                    expected.as_ref().map(|v| self.trim(v.as_slice())),
                    true,
                );
                print::end_test(outcome, true);
                for i in (0..1000).rev() {
                    let in_file =
                        format!("tasks/{}/tests/.failed_{:03}.in", self.task_folder, i);
                    if Path::new(&in_file).exists() {
                        continue;
                    }
                    File::create(in_file).unwrap().write_all(&input).unwrap();
                    if let Some(expected) = expected {
                        File::create(format!(
                            "tasks/{}/tests/.failed_{:03}.ans",
                            self.task_folder, i
                        ))
                        .unwrap()
                        .write_all(&expected)
                        .unwrap();
                    }
                    break;
                }
                return false;
            }
        }
        print::end_test(outcome, test_set.print_details());
    }
    print::end_test_set(test_failed, test_total, max_time, &scores);
    test_failed == 0
}
```

Two key edits versus the original loop:
1. The first `if let Outcome::OK { ... } = outcome` extracts both `duration` and `score`. The `score: Option<i64>` is pushed into the `scores` vector.
2. The final `end_test_set(...)` call passes `&scores`.

- [ ] **Step 3: Verify the workspace builds**

Run: `cargo build -p tester && cargo build -p a_non_crossing_railcar_rearrangement`

Expected: succeeds.

- [ ] **Step 4: Commit**

```bash
git add tester/src/lib.rs tester/src/print.rs
git commit -m "feat(tester): per-test-set score summary line

Aggregate scored tests with count/sum/mean/median/min/max and emit a
summary line after each test set when at least one test produced a score.
Sets without any scored tests render unchanged."
```

---

### Task 4: Simplify AHC064 task tester

Drop the per-task scoring boilerplate now that the framework owns it. The custom `check()` returns `Ok(Some(score))` and only keeps its `eprintln!` for the per-seed turn/error detail.

**Files:**
- Modify: `tasks/a_non_crossing_railcar_rearrangement/src/tester.rs`

- [ ] **Step 1: Replace `tester.rs` with the simplified version**

Open `tasks/a_non_crossing_railcar_rearrangement/src/tester.rs` and replace the whole body (preserving the existing imports for the things still used). The rewrite drops `RunResult`, `NEXT_SEED`, `RESULTS`, `print_summary`, and `verify_gen_reproducibility`-internal coupling to the thread-locals — but keeps `verify_gen_reproducibility` itself, `StressTest`, `check`, and `run_tests`.

Replace the file with:

```rust
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{run, TASK_TYPE, TEST_TYPE, TestType};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::ops::Range;
use tester::classic::default_checker;
use tester::interactive::std_interactor;
use tester::test_set::GeneratedTestSet;
use tester::Tester;

use crate::scoring;

const PRINT_LIMIT: usize = 1000;
const STRESS_SEEDS: Range<u64> = 0..150;

struct StressTest;

impl GeneratedTestSet for StressTest {
    type TestId = u64;

    fn tests(&self) -> impl Iterator<Item = Self::TestId> {
        STRESS_SEEDS
    }

    fn input(&self, seed: &Self::TestId, out: &mut Output) {
        scoring::gen(*seed).write(out);
    }

    fn output(&self, _: &Self::TestId, _: &mut Input, _: &mut Output) -> bool {
        false
    }
}

fn check(mut input: Input, _: Option<Input>, mut output: Input) -> Result<Option<i64>, String> {
    let parsed_input = scoring::Input::read(&mut input);
    match scoring::parse_output(&parsed_input, &mut output) {
        Ok(turns) => {
            let (score, err, turns_to_goal) = scoring::compute_score(&parsed_input, &turns);
            eprintln!(
                "score={} turns={} {}",
                score,
                turns_to_goal
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                if err.is_empty() {
                    String::new()
                } else {
                    format!("ERR: {}", err)
                }
            );
            Ok(Some(score))
        }
        Err(err) => {
            eprintln!("ERR: {}", err);
            Ok(Some(0))
        }
    }
}

fn verify_gen_reproducibility() {
    let mut buf = Vec::new();
    {
        let mut out = Output::buf(&mut buf);
        scoring::gen(0).write(&mut out);
    }
    let actual = String::from_utf8(buf).unwrap();
    let path = "tasks/a_non_crossing_railcar_rearrangement/tools/in/0000.txt";
    let expected = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Cannot read {path}: {e}"));
    assert_eq!(
        actual.trim(),
        expected.trim(),
        "gen(0) does not match {path} — rand/rand_chacha versions disagree",
    );
}

pub(crate) fn run_tests() -> bool {
    verify_gen_reproducibility();
    let path = "./a_non_crossing_railcar_rearrangement";
    let tl = 2000;
    let tester = match TASK_TYPE {
        crate::TaskType::Interactive => {
            Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
        }
        crate::TaskType::Classic => {
            Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, check)
        }
    };
    let passed = tester.test_samples();
    tester.test_generated("Stress test", false, StressTest);
    passed
}
```

What's gone vs. before:
- `RunResult` struct, `NEXT_SEED` and `RESULTS` thread-locals.
- `print_summary` function and the `RESULTS.borrow_mut().clear()` / `NEXT_SEED.set(...)` calls in `run_tests`.
- The seed-tracking `NEXT_SEED.with(|s| s.set(*seed))` in `StressTest::input`.

What's retained:
- The reproducibility check (`verify_gen_reproducibility`) — still useful, still called first.
- `test_samples()` (returns the pass/fail bool that `run_tests` propagates).
- `test_generated("Stress test", false, StressTest)` — drives the scored stress run.

- [ ] **Step 2: Verify the workspace builds**

Run: `cargo build -p a_non_crossing_railcar_rearrangement`

Expected: succeeds.

- [ ] **Step 3: Commit**

```bash
git add tasks/a_non_crossing_railcar_rearrangement/src/tester.rs
git commit -m "refactor(ahc064): drop scoring boilerplate, use tester crate

The check() function now returns Ok(Some(score)) and the tester crate
prints aggregate stats (sum/mean/median/min/max/count) automatically.
RunResult, NEXT_SEED, RESULTS, and print_summary are no longer needed."
```

---

### Task 5: End-to-end verification

Confirm the AHC064 stress run produces the same score sum as before and emits the new summary line.

- [ ] **Step 1: Run the stress test**

Run: `cargo run --release -p a_non_crossing_railcar_rearrangement 2>&1 | tail -20`

Expected output (last ~20 lines, allowing for HashMap-ordering noise around `sum`):
- A bunch of `Test N ... OK [score=...]` lines.
- A line that ends with `All 150 tests passed, max time elapsed: ...s`.
- A `=== Scores: count=150 sum=... mean=... median=... min=... max=... ===` line where:
  - `count=150`,
  - `sum` is in the neighborhood of **739163** (±100 from HashMap nondeterminism in beam ranking),
  - `mean ≈ 4927`,
  - `min ≥ 4910`, `max ≤ 4940`.

- [ ] **Step 2: Sanity-check a binary-only test path**

Pick any non-AHC task in the workspace (or run sample tests for AHC064 — the empty `tests/` directory yields zero sample tests, so this only confirms the no-score code path emits no summary line).

Run: `cargo run --release -p a_non_crossing_railcar_rearrangement 2>&1 | grep -E "Sample tests|Scores:" | head -5`

Expected: a `Test set: Sample tests` and `All 0 tests passed` line, with NO `=== Scores: ===` line following them (because no sample test produced a score).

- [ ] **Step 3: Commit (only if any followup tweaks were needed)**

If steps 1-2 pass cleanly, no commit needed for this task. If a regression was caught and patched, commit the fix here.
