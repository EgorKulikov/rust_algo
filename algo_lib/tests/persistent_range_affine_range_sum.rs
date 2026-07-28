#![allow(unexpected_cfgs)]
use algo_lib::collections::payload::Payload;
use algo_lib::collections::treap::persistent_treap::PersistentTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_unsigned_vec(n);

    type Mod = ModIntF;
    #[derive(Clone)]
    struct Node {
        self_val: Mod,
        val: Mod,
        qty: Mod,
        k: Mod,
        b: Mod,
    }
    impl Node {
        fn new(a: u32) -> Self {
            Self {
                self_val: a.into(),
                val: a.into(),
                qty: Mod::one(),
                k: Mod::one(),
                b: Mod::zero(),
            }
        }
    }
    impl Payload for Node {
        const NEED_ACCUMULATE: bool = true;
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.val = self.self_val + left.map_or(Mod::zero(), |l| l.val) + right.map_or(Mod::zero(), |r| r.val);
            self.qty = Mod::one() + left.map_or(Mod::zero(), |l| l.qty) + right.map_or(Mod::zero(), |r| r.qty);
        }

        fn accumulate(&mut self, delta: &Self) {
            self.self_val = delta.k * self.self_val + delta.b;
            self.val = delta.k * self.val + delta.b * self.qty;
            self.k *= delta.k;
            self.b = delta.k * self.b + delta.b;
        }

        fn accumulate_self(&mut self, delta: &Self) {
            self.self_val = delta.k * self.self_val + delta.b;
        }

        fn reset_delta(&mut self) {
            self.k = Mod::one();
            self.b = Mod::zero();
        }

        fn need_push_down(&self) -> bool {
            self.k != Mod::one() || self.b != Mod::zero()
        }
    }
    let mut trees = vec![PersistentTree::with_gen(n, |i| Node::new(a[i]))];

    for _ in 0..q {
        let t = input.read_int();
        match t {
            0 => {
                let k = (input.read_int() + 1) as usize;
                let l = input.read_size();
                let r = input.read_size();
                let b: Mod = input.read();
                let c: Mod = input.read();
                let delta = Node {
                    self_val: Mod::zero(),
                    val: Mod::zero(),
                    qty: Mod::zero(),
                    k: b,
                    b: c,
                };
                trees.push(trees[k].push_range(l..r, &delta));
            }
            1 => {
                let k = (input.read_int() + 1) as usize;
                let s = (input.read_int() + 1) as usize;
                let l = input.read_size();
                let r = input.read_size();
                let (left, _, right) = trees[k].split_range_index(l..r);
                let mid = trees[s].range_index(l..r);
                trees.push(PersistentTree::merge_three(left, mid, right));
            }
            2 => {
                let k = (input.read_int() + 1) as usize;
                let l = input.read_size();
                let r = input.read_size();
                let cur = trees[k];
                trees.push(cur);
                out.print_line(cur.range_payload(l..r).unwrap().val);
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}


#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
mod tester {
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{run, TASK_TYPE, TEST_TYPE, TestType};
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::Random;
use algo_lib::string::str::StrReader;
use std::io::{Read, Write};
use std::thread::yield_now;
use tester::classic::default_checker;
use tester::classic::EPS;
use tester::interactive::std_interactor;
use tester::interactive::SolutionRunner;
use tester::test_set::GeneratedTestSet;
use tester::Tester;

const PRINT_LIMIT: usize = 1000;

fn interact(mut input: Input, expected: Option<Input>, mut runner: SolutionRunner) -> Result<Option<i64>, String> {
    let (mut sol, mut out) = runner.run();
    Ok(None)
}

fn run_twice(
    mut input: Input,
    expected: Option<Input>,
    mut runner: SolutionRunner,
) -> Result<Option<i64>, String> {
    let (mut sol, mut out) = runner.run();
    input.read_line();
    out.print_line("first");
    let t = match TEST_TYPE {
        TestType::RunTwiceSingle => None,
        TestType::RunTwiceMultiNumber => {
            let t = input.read_size();
            out.print_line(t);
            Some(t)
        }
        _ => unreachable!(),
    };
    let mut input_vec = Vec::new();
    input.read_to_end(&mut input_vec).unwrap();
    out.write_all(&input_vec).unwrap();
    out.flush();
    while !runner.is_finished() {
        yield_now();
    }
    let mut first_output = Vec::new();
    sol.read_to_end(&mut first_output).unwrap();

    let (mut sol, mut out) = runner.run();
    out.print_line("second");
    if let Some(t) = t {
        out.print_line(t);
    }
    out.write_all(&first_output).unwrap();
    out.flush();
    let mut ans = Vec::new();
    sol.read_to_end(&mut ans).unwrap();
    default_checker(Input::slice(&input_vec), expected, Input::slice(&ans))
    // check(Input::slice(&input_vec), expected, Input::slice(&ans))
}

fn check(mut input: Input, expected: Option<Input>, mut output: Input) -> Result<Option<i64>, String> {
    Ok(None)
}

struct StressTest;

impl GeneratedTestSet for StressTest {
    type TestId = usize;

    fn tests(&self) -> impl Iterator<Item=Self::TestId> {
        1..
    }

    fn input(&self, test: &Self::TestId, out: &mut Output) {
        let mut r = Random::new();
    }

    fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
        false
    }
}

struct MaxTest;

impl GeneratedTestSet for MaxTest {
    type TestId = usize;

    fn tests(&self) -> impl Iterator<Item=Self::TestId> {
        1..=1
    }

    fn input(&self, test: &Self::TestId, out: &mut Output) {
        let mut r = Random::new_with_seed(239);
    }

    fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
        false
    }
}

pub(crate) fn run_tests() -> bool {
    let path = "./persistent_range_affine_range_sum";
    let tl = 5000;
    let tester = match TASK_TYPE {
        crate::TaskType::Interactive => {
            Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
            // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, interact)
            // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, run_twice)
        }
        crate::TaskType::Classic => {
            Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, default_checker)
            // Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, check)
        }
    };
    let passed = tester.test_samples();
    // tester.test_generated("Max test", true, MaxTest);
    // tester.test_generated("Stress test", false, StressTest);
    passed
}
}
#[test]
fn persistent_range_affine_range_sum() {
    assert!(tester::run_tests());
}
