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
