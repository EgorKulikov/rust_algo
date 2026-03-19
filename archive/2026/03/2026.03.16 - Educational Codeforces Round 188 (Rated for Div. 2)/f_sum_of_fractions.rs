//{"name":"F. Sum of Fractions","group":"Codeforces - Educational Codeforces Round 188 (Rated for Div. 2)","url":"https://codeforces.com/contest/2204/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n2 3 5 2 3\n0 1 2 10\n","output":"232923695\n332748137\n931694761\n133099397\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::string::str::StrReader;
use std::cmp::Reverse;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n);
    let k = input.read_size_vec(m);

    type Mod = ModIntF;

    let mut base = Mod::new(0);
    for i in 0..n {
        base += Mod::new(1) / a[i] * (i + 1) * (n - i);
    }
    let mut all = (0..=n + 1).collect::<BTreeSet<_>>();
    let order = (0..n).collect::<Vec<_>>().sorted_by_key(|&i| Reverse(a[i]));
    let mut v = Vec::new();
    let mut delta = Mod::new(0);
    for i in order {
        all.remove(&(i + 1));
        let left = i + 1 - all.prev(&(i + 1)).unwrap();
        let right = all.next(&(i + 1)).unwrap() - (i + 1);
        // base -= Mod::new(1) / a[i] * left * right;
        delta += Mod::new(1) / a[i] * left * right;
        v.push((a[i] - 1, left * right));
    }
    v.sort();
    let mut ans = base;
    let mut last = 0;
    let mut at = 0;
    for k in k {
        while at < v.len() && v[at].0 <= k {
            ans += delta * (v[at].0 - last);
            delta -= Mod::from(v[at].1) / (v[at].0 + 1);
            delta += v[at].1;
            last = v[at].0;
            at += 1;
        }
        ans += delta * (k - last);
        last = k;
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
