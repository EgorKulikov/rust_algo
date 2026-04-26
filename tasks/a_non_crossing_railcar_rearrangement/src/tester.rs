#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{run, TASK_TYPE, TEST_TYPE, TestType};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::cell::{Cell, RefCell};
use std::ops::Range;
use tester::classic::default_checker;
use tester::interactive::std_interactor;
use tester::test_set::GeneratedTestSet;
use tester::Tester;

use crate::scoring;

const PRINT_LIMIT: usize = 1000;
const STRESS_SEEDS: Range<u64> = 0..150;

#[derive(Clone, Debug)]
struct RunResult {
    seed: u64,
    score: i64,
    turns: Option<usize>,
    err: String,
}

thread_local! {
    static NEXT_SEED: Cell<u64> = Cell::new(STRESS_SEEDS.start);
    static RESULTS: RefCell<Vec<RunResult>> = RefCell::new(Vec::new());
}

struct StressTest;

impl GeneratedTestSet for StressTest {
    type TestId = u64;

    fn tests(&self) -> impl Iterator<Item = Self::TestId> {
        STRESS_SEEDS
    }

    fn input(&self, seed: &Self::TestId, out: &mut Output) {
        NEXT_SEED.with(|s| s.set(*seed));
        scoring::gen(*seed).write(out);
    }

    fn output(&self, _: &Self::TestId, _: &mut Input, _: &mut Output) -> bool {
        false
    }
}

fn check(mut input: Input, _: Option<Input>, mut output: Input) -> Result<Option<i64>, String> {
    let seed = NEXT_SEED.with(|s| s.get());
    let parsed_input = scoring::Input::read(&mut input);
    let result = match scoring::parse_output(&parsed_input, &mut output) {
        Ok(turns) => {
            let (score, err, turns_to_goal) = scoring::compute_score(&parsed_input, &turns);
            RunResult {
                seed,
                score,
                turns: turns_to_goal,
                err,
            }
        }
        Err(err) => RunResult {
            seed,
            score: 0,
            turns: None,
            err,
        },
    };
    eprintln!(
        "seed {:04}: score={} turns={} {}",
        result.seed,
        result.score,
        result
            .turns
            .map(|t| t.to_string())
            .unwrap_or_else(|| "-".to_string()),
        if result.err.is_empty() {
            String::new()
        } else {
            format!("ERR: {}", result.err)
        }
    );
    RESULTS.with(|r| r.borrow_mut().push(result));
    Ok(None)
}

fn print_summary(seeds: Range<u64>) {
    let results = RESULTS.with(|r| r.borrow().clone());
    let n = results.len();
    if n == 0 {
        return;
    }
    let solved: Vec<&RunResult> = results.iter().filter(|r| r.turns.is_some()).collect();
    let invalid = results.iter().filter(|r| !r.err.is_empty()).count();
    let total: i64 = results.iter().map(|r| r.score).sum();
    let mean = total as f64 / n as f64;
    let mut scores: Vec<i64> = results.iter().map(|r| r.score).collect();
    scores.sort_unstable();
    let median = scores[n / 2];
    let avg_solved_turns = if !solved.is_empty() {
        solved
            .iter()
            .map(|r| r.turns.unwrap() as f64)
            .sum::<f64>()
            / solved.len() as f64
    } else {
        0.0
    };
    let worst = results.iter().min_by_key(|r| r.score).unwrap();

    eprintln!();
    eprintln!(
        "=== Stress test summary (seeds {}..{}) ===",
        seeds.start, seeds.end
    );
    eprintln!(
        "solved:   {}/{}{}",
        solved.len(),
        n,
        if solved.is_empty() {
            String::new()
        } else {
            format!("   (avg turns when solved: {:.1})", avg_solved_turns)
        }
    );
    eprintln!(
        "score:    sum={}   mean={:.2}   median={}",
        total, mean, median
    );
    eprintln!("invalid:  {}", invalid);
    eprintln!(
        "worst:    seed={:04}   score={}   ({})",
        worst.seed,
        worst.score,
        if worst.turns.is_some() {
            "goal".to_string()
        } else if !worst.err.is_empty() {
            format!("err: {}", worst.err)
        } else {
            "no goal".to_string()
        }
    );
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
    RESULTS.with(|r| r.borrow_mut().clear());
    NEXT_SEED.with(|s| s.set(STRESS_SEEDS.start));
    tester.test_generated("Stress test", false, StressTest);
    print_summary(STRESS_SEEDS);
    passed
}
