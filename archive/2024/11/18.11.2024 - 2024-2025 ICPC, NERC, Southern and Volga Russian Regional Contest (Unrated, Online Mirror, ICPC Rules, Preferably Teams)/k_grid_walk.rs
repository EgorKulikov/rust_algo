//{"name":"K. Grid Walk","group":"Codeforces - 2024-2025 ICPC, NERC, Southern and Volga Russian Regional Contest (Unrated, Online Mirror, ICPC Rules, Preferably Teams)","url":"https://codeforces.com/contest/2038/problem/K","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2 4\n","output":"21\n"},{"input":"10 210 420\n","output":"125\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KGridWalk"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let a = input.read_long();
    let b = input.read_long();

    let sa = (1..=n).rev().filter(|&x| gcd(x, a) == 1).next().unwrap();
    let sb = (1..=n).rev().filter(|&x| gcd(x, b) == 1).next().unwrap();
    let mut ans = Arr2d::new((n - sa) as usize + 1, (n - sb) as usize + 1, 1);
    ans[(0, 0)] = 2;
    for i in 0..ans.d1() {
        for j in 0..ans.d2() {
            if i == 0 && j == 0 {
                continue;
            }
            let base = if i == 0 {
                ans[(0, j - 1)]
            } else if j == 0 {
                ans[(i - 1, 0)]
            } else {
                ans[(i - 1, j)].min(ans[(i, j - 1)])
            };
            ans[(i, j)] = base + gcd(sa + i as i64, a) + gcd(sb + j as i64, b);
        }
    }
    let mut ans = ans[ans.d1() - 1][ans.d2() - 1];
    for i in 1..sa {
        ans += gcd(i, a) + 1;
    }
    for j in 1..sb {
        ans += gcd(j, b) + 1;
    }
    out.print_line(ans);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
