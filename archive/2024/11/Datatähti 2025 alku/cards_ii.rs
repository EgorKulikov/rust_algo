//{"name":"Cards II","group":"CSES - Datatähti 2025 alku","url":"https://cses.fi/524/task/D","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3 1 2\n2 0 1\n5 2 2\n9 3 5\n4 4 1\n","output":"6\n0\n4200\n976757050\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CardsII"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::mem::swap;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::mod_utils::Combinations;
// use algo_lib::numbers::num_traits::algebra::Zero;
// use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size();
    let b = input.read_size();

    if a + b > n {
        out.print_line(0);
        return;
    }
    type Mod = ModInt7;
    let mut cur = Arr2d::new(a + 1, b + 1, 0i64);
    let mut next = Arr2d::new(a + 1, b + 1, 0i64);
    let c = Combinations::<Mod>::new(n + 1);
    cur[(0, 0)] = (c.c(n, a + b) * c.fact(n)).val() as i64;
    for i in 0..a + b {
        next.fill(0);
        for j in 0..=a.min(i) {
            for k in 0..=b.min(i) {
                let xa = i - k;
                let xb = i - j;
                if j < xa || k < xb {
                    continue;
                }
                let val = cur[(j, k)] % 1_000_000_007;
                if j < a && k < b {
                    next[(j + 1, k + 1)] += val;
                }
                if j < a {
                    next[(j + 1, k)] += val * (j - xa) as i64;
                }
                if k < b {
                    next[(j, k + 1)] += val * (k - xb) as i64;
                }
                next[(j, k)] += val * (j - xa) as i64 * (k - xb) as i64;
            }
        }
        swap(&mut cur, &mut next);
    }
    out.print_line(cur[(a, b)] % 1_000_000_007);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
