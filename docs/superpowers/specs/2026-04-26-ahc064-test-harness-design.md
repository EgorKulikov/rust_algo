# AHC064 (Non-Crossing Railcar Rearrangement) — Test Harness

## Goal

Build local infrastructure to evaluate our solution against many seeds of the
AtCoder Heuristic Contest 064 problem, with per-seed scores and an aggregate
summary, by porting the contest's `tools/` simulator into the task crate.

## Non-Goals

- SVG / `vis.html` visualization (deferred — option (d) for later).
- Comparison against an alternative or reference solution.
- Parallel execution across cores (defer until per-seed runtime makes it
  worthwhile).
- A CLI for selecting seed ranges — the seed range is a constant in
  `tester.rs` that we edit between runs.

## Problem Recap

- 10 main lines × 10 cars each, 10 empty siding lines. Capacities 15 / 20.
- Each turn applies a set of moves (main→siding take from end / prepend to
  siding; siding→main take from front / append to main). Constraint: each
  main and each siding used at most once per turn, and when sorted by main
  index ascending, siding indices strictly increase ("non-crossing").
- Goal: main line `r` holds `[10r, 10r+1, …, 10r+9]`.
- Score: `5000 − T` if goal reached in `T ≤ 4000` turns, otherwise partial
  score (per car: +1 for correct main, +9 if also correct slot).

## Architecture

```
tasks/a_non_crossing_railcar_rearrangement/
├── Cargo.toml          # add rand = "0.8", rand_chacha = "0.3"
├── src/
│   ├── main.rs         # untouched
│   ├── tester.rs       # wire StressTest + check() + aggregator
│   └── scoring.rs      # NEW
└── tools/              # extracted from zip; used as ground truth for gen()
```

## Components

### `scoring.rs`

Self-contained port of the relevant pieces of `tools/src/lib.rs`. Drops the
SVG / `Display` / proconio dependencies. Exposes:

- Constants: `R = 10`, `TMAX = 4000`, `TOTAL_CARS = 100`, `CARS_PER_LINE = 10`,
  `MAIN_LINE_CAPACITY = 15`, `SIDING_LINE_CAPACITY = 20`.
- `pub struct Input { pub main_lines: Vec<Vec<usize>> }` (R is a constant, no
  need to store it). With:
  - `pub fn write(&self, out: &mut algo_lib::io::output::Output)` — emits
    `R\n` then 10 space-separated lines of 10 car ids.
  - `pub fn read(input: &mut algo_lib::io::input::Input) -> Input`.
- `pub fn gen(seed: u64) -> Input` — uses
  `rand_chacha::ChaCha20Rng::seed_from_u64(seed)` and
  `rand::seq::SliceRandom::shuffle` so the byte output matches the contest's
  `tools/in/{seed:04}.txt`.
- `pub type Move = (u8, usize, usize, usize)`.
- `pub fn parse_output(input: &Input, out: &mut algo_lib::io::input::Input)
  -> Result<Vec<Vec<Move>>, String>` — same field-by-field validation as the
  contest tools' `parse_output`.
- `pub fn compute_score(input: &Input, turns: &[Vec<Move>])
  -> (i64, String, Option<usize>)` — returns `(score, err_msg, turns_if_goal)`.
  Same simulation logic as the tools' `compute_score_details`.

### `tester.rs`

Replaces the placeholder `StressTest` and `check`:

```rust
const STRESS_SEEDS: std::ops::Range<u64> = 0..100;

struct StressTest;
impl GeneratedTestSet for StressTest {
    type TestId = u64;
    fn tests(&self) -> impl Iterator<Item = u64> { STRESS_SEEDS }
    fn input(&self, seed: &u64, out: &mut Output) { scoring::gen(*seed).write(out); }
    fn output(&self, _: &u64, _: &mut Input, _: &mut Output) -> bool { false }
}
```

Score aggregation uses two thread-locals updated in lockstep, since the
`check()` callback receives bytes only (no test-id):

```rust
thread_local! {
    static NEXT_SEED: Cell<u64> = Cell::new(STRESS_SEEDS.start);
    static RESULTS: RefCell<Vec<RunResult>> = RefCell::new(vec![]);
}

struct RunResult { seed: u64, score: i64, turns: Option<usize>, err: String }
```

`StressTest::input()` reads & advances `NEXT_SEED`; `check()` reads it (already
advanced — but we know the order: input is called first, then run, then
check) — see "Seed ordering" below.

`check()`:

1. Parse input (we have the input bytes), parse output via
   `scoring::parse_output`. On parse failure, record `RunResult` with score 0
   and the parse error, return `Ok(())`.
2. Call `scoring::compute_score`.
3. `eprintln!("seed {seed:04}: score={score} turns={turns:?} err={err}")`.
4. Push `RunResult` into `RESULTS`.
5. Return `Ok(())` always — heuristic problems never "fail".

`run_tests()` clears the thread-locals, runs `tester.test_generated("Stress",
true, StressTest)`, then prints the aggregate.

### Seed ordering

The Tester's iteration is single-threaded and strictly sequential per test:
`tests()` → `input(test)` → solution runs → `check(input, expected, actual)` →
next test. We exploit this:

- `input()` reads `NEXT_SEED`, generates input from that seed, then bumps
  `NEXT_SEED` by 1.
- `check()` reads (already-bumped) `NEXT_SEED − 1` to know which seed it's
  scoring.

This is fragile if Tester ever parallelises, but it doesn't, and we'd notice
immediately because seeds in the per-line eprintln output would be wrong.

### Aggregate output

After the test set finishes, `run_tests()` prints:

```
=== Stress test summary (seeds 0..100) ===
solved:   17/100   (avg turns when solved: 1342.4)
score:    total=85234   mean=852.34   median=910
invalid:  0
worst:    seed=0042   score=120   (no goal)
```

`invalid` counts seeds with non-empty `err` (parse error or simulation
violation). "no goal" means `turns.is_none()` — partial score case.

## Reproducibility Verification

After implementation, run a one-off check: write `gen(0)` to a buffer and
`assert_eq!` against `std::fs::read_to_string("tools/in/0000.txt")`. If it
matches, our `rand_chacha` versions agree with the contest's. We do this once
manually, not in CI; if it ever fails after a `cargo update`, fix the version
pin.

## Cargo.toml deltas

```toml
[dependencies]
algo_lib = { path = "../../algo_lib" }
tester = { path = "../../tester" }
rand = "0.8"
rand_chacha = "0.3"
```

(Plain deps. Only `scoring.rs` references them, which is `mod`-included
only from `tester.rs`, which is itself `#[cfg(feature = "local")]`. The
AtCoder submission is generated source via the rust_algo tooling, not a
built binary, so these never reach the submitted code.)

## Out-of-Scope / Future

- Saving SVGs for low-score seeds (option (d) — wire by importing the `svg`
  crate and the `vis()` function from the original `tools/src/lib.rs`, gated
  behind a const flag).
- Parallel execution (use `rayon` once a single seed costs > 1 s).
- Replaying a single saved seed against an updated solution (trivial — just
  set `STRESS_SEEDS` to `42..43`).
