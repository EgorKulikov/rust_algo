//{"name":"L. Books","group":"Codeforces - Constructor Open Cup 2026","url":"https://constructor2026.contest.codeforces.com/group/XdjJUfzFUt/contest/670933/problem/L","interactive":false,"timeLimit":3500,"tests":[{"input":"7\n1 1 1\n2 2 2\n2 2 3\n2 3 3\n7 8 10\n10 11 13\n5 7 9\n","output":"1\n1\n2\n1\n1535\n98303\n104\n"},{"input":"1\n362073723 394277321 456387899\n","output":"938745773\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::{FxHashMap, FxHashSet};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::number_ext::Power;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    type Mod = ModIntF;
    let t = input.read_size();

    let mut tests = Vec::with_capacity(t);
    let mut poi = FxHashSet::default();
    for _ in 0..t {
        let n = input.read_size();
        let l = input.read_size() - n;
        let r = input.read_size() - n;
        tests.push((n, l, r));
        poi.insert(l);
        poi.insert(r + 1);
    }
    let max = poi.iter().copied().max().unwrap();
    let mut fact = FxHashMap::default();
    let mut cur = Mod::from(1);
    fact.insert(0, cur);
    for i in 1..=max {
        cur *= i;
        if poi.contains(&i) {
            fact.insert(i, cur);
        }
    }

    for (n, l, r) in tests {
        let ans = Mod::from(r + 1).power(n - (r + 1)) * fact[&(r + 1)]
            - Mod::from(l).power(n - l) * fact[&l];
        out.print_line(ans);
    }
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
