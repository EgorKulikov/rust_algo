#![allow(unexpected_cfgs)]
//{"name":"H. Dipper and the Machine","group":"Codeforces - Treaps","url":"https://codeforces.com/gym/539514/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4\n1 2 2 two\n1 2 3 aa\n2 3 1 1\n2 2 1 4\n","output":"a\ntwoa\n"},{"input":"3 6\n1 1 2 ab\n1 2 3 cd\n2 2 2 4\n2 3 1 2\n1 1 3 xyzu\n2 1 2 5\n","output":"bcd\ncd\nbxyz\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::treap::Tree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    struct TreeNode {
        id: usize,
        len: usize,
        s: Str,
    }

    impl Payload for TreeNode {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.len = self.s.len() + left.map_or(0, |l| l.len) + right.map_or(0, |r| r.len);
        }
    }

    impl OrdPayload for TreeNode {
        type Key = usize;

        fn key(&self) -> &Self::Key {
            &self.id
        }
    }

    let mut add = vec![Vec::new(); n];
    let mut remove = vec![Vec::new(); n];
    let mut queries = vec![Vec::new(); n];
    for i in 0..m {
        let mode = input.read_int();
        match mode {
            1 => {
                let l = input.read_size() - 1;
                let r = input.read_size() - 1;
                let s = input.read_str();
                add[l].push((i, s));
                remove[r].push(i);
            }
            2 => {
                let pos = input.read_size() - 1;
                let x = input.read_size() - 1;
                let y = input.read_size();
                queries[pos].push((i, x, y));
            }
            _ => unreachable!(),
        }
    }

    let mut ans = vec![Str::new(); m];

    let mut tree = Tree::<TreeNode>::new();
    for i in 0..n {
        for (id, s) in add[i].drain(..) {
            assert!(tree
                .insert(TreeNode {
                    id,
                    len: s.len(),
                    s,
                })
                .is_none());
        }
        for (id, mut x, y) in queries[i].drain(..) {
            let mut len = y - x;
            let slice = tree.split_by_tail(|node, left, _| {
                let total = left.map_or(0, |l| l.len) + node.s.len();
                if total <= x {
                    x -= total;
                    Direction::Right
                } else {
                    Direction::Left
                }
            });
            for node in slice.iter() {
                let s = &node.s;
                assert!(s.len() > x);
                let slice = &s[x..(x + len).min(s.len())];
                ans[id] += slice;
                len -= slice.len();
                x = 0;
                if len == 0 {
                    break;
                }
            }
            assert_eq!(len, 0);
        }
        for id in remove[i].drain(..) {
            assert!(tree.remove(&id).is_some());
        }
    }

    out.print_per_line_iter(ans.iter().filter(|s| !s.is_empty()));
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();

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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
mod tester {
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{run, TestType, TASK_TYPE, TEST_TYPE};
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::{Random, RandomTrait};
use algo_lib::string::str::{Str, StrReader};
use std::io::{Read, Write};
use std::thread::yield_now;
use tester::classic::default_checker;
use tester::interactive::std_interactor;
use tester::interactive::SolutionRunner;
use tester::test_set::GeneratedTestSet;
use tester::Tester;

const PRINT_LIMIT: usize = 1000;

fn interact(
    mut input: Input,
    expected: Option<Input>,
    mut runner: SolutionRunner,
) -> Result<(), String> {
    let (mut sol, mut out) = runner.run();
    Ok(())
}

fn run_twice(
    mut input: Input,
    expected: Option<Input>,
    mut runner: SolutionRunner,
) -> Result<(), String> {
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

fn check(mut input: Input, expected: Option<Input>, mut output: Input) -> Result<(), String> {
    Ok(())
}

struct StressTest;

impl GeneratedTestSet for StressTest {
    type TestId = usize;

    fn tests(&self) -> impl Iterator<Item = Self::TestId> {
        1..
    }

    fn input(&self, test: &Self::TestId, out: &mut Output) {
        let mut r = Random::new();
        let n = r.gen_range(1..=5);
        let m = r.gen_range(1..=5);
        out.print_line((n, m));
        let mut len = vec![0; n];
        for i in 0..m {
            loop {
                if r.gen_bool() {
                    let from = r.gen_range(1..=n);
                    let to = r.gen_range(from..=n);
                    let l = r.gen_range(1..=3);
                    for j in from - 1..to {
                        len[j] += l;
                    }
                    out.print_line((
                        1,
                        from,
                        to,
                        Str::from(Vec::with_gen(l, |_| r.gen_range(b'a'..=b'z'))),
                    ));
                    break;
                } else {
                    let pos = r.gen_range(1..=n);
                    if len[pos - 1] == 0 {
                        continue;
                    }
                    let x = r.gen_range(1..=len[pos - 1]);
                    let y = r.gen_range(x..=len[pos - 1]);
                    out.print_line((2, pos, x, y));
                    break;
                }
            }
        }
    }

    fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
        let n = input.read_size();
        let m = input.read_size();

        let mut s = vec![Str::new(); n];
        for _ in 0..m {
            let mode = input.read_int();
            match mode {
                1 => {
                    let l = input.read_size() - 1;
                    let r = input.read_size() - 1;
                    let add_str = input.read_str();
                    for i in l..=r {
                        s[i] += &add_str;
                    }
                }
                2 => {
                    let pos = input.read_size() - 1;
                    let x = input.read_size() - 1;
                    let y = input.read_size();
                    out.print_line(Str::from(&s[pos][x..y]));
                }
                _ => unreachable!(),
            }
        }
        true
    }
}

struct MaxTest;

impl GeneratedTestSet for MaxTest {
    type TestId = usize;

    fn tests(&self) -> impl Iterator<Item = Self::TestId> {
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
    let path = "./h_dipper_and_the_machine";
    let tl = 2000;
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
    tester.test_generated("Stress test", false, StressTest);
    passed
}
}
#[test]
fn h_dipper_and_the_machine() {
    assert!(tester::run_tests());
}
