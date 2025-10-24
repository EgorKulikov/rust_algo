//{"name":"E - Development","group":"AtCoder - AtCoder Beginner Contest 416","url":"https://atcoder.jp/contests/abc416/tasks/abc416_e","interactive":false,"timeLimit":3500,"tests":[{"input":"4 1\n1 2 10\n2 100\n1 3\n5\n3\n1 2 3 60\n3\n2 4\n3\n","output":"440\n280\n900\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, i64)>(m).dec();
    let k = input.read_size();
    let t = input.read_long();
    let d = input.read_size_vec(k).dec();

    const INFTY: i64 = i64::MAX / 3;
    let mut dst = Arr2d::new(n + 1, n + 1, INFTY);
    for i in 0..=n {
        dst[(i, i)] = 0;
    }
    for (u, v, w) in edges {
        dst[(u, v)].minim(w * 2);
        dst[(v, u)].minim(w * 2);
    }
    for i in d {
        dst[(i, n)].minim(t);
        dst[(n, i)].minim(t);
    }
    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n {
                let cand = dst[(j, i)] + dst[(i, k)];
                dst[(j, k)].minim(cand);
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            ans += if dst[(i, j)] < INFTY { dst[(i, j)] } else { 0 };
        }
    }

    let q = input.read_size();
    for _ in 0..q {
        let tp = input.read_int();
        let (from, to, d) = match tp {
            1 => {
                let x = input.read_size() - 1;
                let y = input.read_size() - 1;
                let d = input.read_long();
                (x, y, d * 2)
            }
            2 => {
                let x = input.read_size() - 1;
                (x, n, t)
            }
            3 => {
                out.print_line(ans / 2);
                continue;
            }
            _ => unreachable!(),
        };
        for i in 0..=n {
            for j in 0..=n {
                let was = dst[(i, j)];
                for k in [from, to] {
                    let cand = dst[(i, k)] + d + dst[(from + to - k, j)];
                    dst[(i, j)].minim(cand);
                }
                if i < n && j < n && dst[(i, j)] != was {
                    if was < INFTY {
                        ans -= was;
                    }
                    ans += dst[(i, j)];
                }
            }
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
