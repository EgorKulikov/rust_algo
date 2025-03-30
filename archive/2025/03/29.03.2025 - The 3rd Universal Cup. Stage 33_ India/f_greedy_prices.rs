//{"name":"F. Greedy Prices","group":"Universal Cup - The 3rd Universal Cup. Stage 33: India","url":"https://contest.ucup.ac/contest/1954/problem/10270","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3 4\n1 1000 0\n3 6 4\n2\n6\n7\n19\n6 6\n0 0 1 1 2 3\n0 1 0 2 3 4\n0\n1\n8\n15\n3\n10000\n","output":"0 1 2 3\n2 3 4 5 4 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::rational::Rational;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let m = input.read_long_vec(n);
    let c = input.read_long_vec(n);

    let mut zeroes = Vec::new();
    let mut non_zeroes = Vec::new();
    let mut bonus = 0;
    for i in 0..n {
        if c[i] == 0 {
            bonus += 1;
        } else if m[i] == 0 {
            zeroes.push(c[i]);
        } else {
            non_zeroes.push((m[i], c[i]));
        }
    }
    zeroes.sort();
    non_zeroes.sort_by_key(|(m, c)| Rational::new(*c as i128, *m as i128));
    let mut base = vec![0];
    for c in zeroes {
        base.push(base[Back(0)] + c);
    }
    const LIMIT: i64 = 1_000_000_000;
    let mut ans = vec![0];
    for (m, c) in non_zeroes {
        for i in ans.indices().rev() {
            let cand = ans[i] * (m + 1) + c;
            if cand <= LIMIT && (i == ans.len() - 1 || cand < ans[i + 1]) {
                if i + 1 < ans.len() {
                    ans[i + 1] = cand;
                } else {
                    ans.push(cand);
                }
            }
        }
    }
    let mut res = Vec::with_capacity(q);
    for _ in 0..q {
        let p = input.read_long();
        let mut cur = 0;
        for i in ans.indices() {
            if ans[i] > p {
                break;
            }
            cur.maxim(i + base.less_or_eq(&(p - ans[i])) - 1 + bonus);
        }
        res.push(cur);
    }
    out.print_line(res);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
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
