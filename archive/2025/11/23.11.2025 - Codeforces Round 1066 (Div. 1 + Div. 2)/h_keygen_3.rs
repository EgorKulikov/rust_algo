//{"name":"H. Keygen 3","group":"Codeforces - Codeforces Round 1066 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2157/problem/H","interactive":false,"timeLimit":4000,"tests":[{"input":"6 3\n","output":"9\n1 4 5 6 3 2\n6 5 4 3 2 1\n1 2 4 5 6 3\n1 2 5 6 4 3\n1 3 4 6 5 2\n1 5 6 4 3 2\n3 5 6 4 2 1\n1 3 6 5 4 2\n2 6 5 4 3 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::{Random, RandomTrait};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::StrReader;
use std::collections::VecDeque;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let nn = input.read_size();
    let m = input.read_size();

    if nn - m < 19 {
        let n = nn.min(19);
        let m = n - (nn - m);
        let mut ans = Vec::new();
        let mut s = BitSet::new(n);
        for mask in usize::iter_all(n - 1) {
            let mut p = VecDeque::new();
            p.push_back(n - 1);
            for i in (0..n - 1).rev() {
                if mask.is_set(i) {
                    p.push_front(i);
                } else {
                    p.push_back(i);
                }
            }
            s.fill(false);
            let mut c = 0;
            for i in 0..n {
                if s[i] {
                    continue;
                }
                let mut cur = i;
                loop {
                    s.set(cur);
                    cur = p[cur];
                    if cur == i {
                        break;
                    }
                }
                c += 1;
            }
            if c == m {
                let mut cur = Vec::new();
                for i in 0..(nn - n) {
                    cur.push(i + 1);
                }
                for i in 0..n {
                    cur.push(p[i] + 1 + (nn - n));
                }
                ans.push(cur);
                if ans.len() == 2000 {
                    break;
                }
            }
        }
        out.print_line(ans.len());
        out.print_per_line(&ans);
    } else {
        let n = nn + 1 - m;
        let m = 1;
        let mut ans = Vec::new();
        let mut r = Random::new_with_seed(239);
        let mut seen = FxHashSet::default();
        let mut s = BitSet::new(n);
        while ans.len() < 2000 {
            let mut p = VecDeque::new();
            p.push_back(n - 1);
            for i in (0..n - 1).rev() {
                if r.gen_bool() {
                    p.push_front(i);
                } else {
                    p.push_back(i);
                }
            }
            s.fill(false);
            let mut c = 0;
            for i in 0..n {
                if s[i] {
                    continue;
                }
                let mut cur = i;
                loop {
                    s.set(cur);
                    cur = p[cur];
                    if cur == i {
                        break;
                    }
                }
                c += 1;
            }
            if c == m {
                if !seen.insert(p.clone()) {
                    continue;
                }
                let mut cur = Vec::new();
                for i in 0..(nn - n) {
                    cur.push(i + 1);
                }
                for i in 0..n {
                    cur.push(p[i] + 1 + (nn - n));
                }
                ans.push(cur);
            }
        }
        out.print_line(ans.len());
        out.print_per_line(&ans);
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
