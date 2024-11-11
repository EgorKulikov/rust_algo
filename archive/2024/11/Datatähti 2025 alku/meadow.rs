//{"name":"Meadow","group":"CSES - Datatähti 2025 alku","url":"https://cses.fi/524/task/E","interactive":false,"timeLimit":1000,"tests":[{"input":"4\nAAAK\nABAA\nABAA\nAAAA\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Meadow"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let map = input.read_char_table(n, n);

    let present = map
        .iter()
        .fold(0, |acc, &cell| acc.with_bit(cell as usize - b'A' as usize));
    let mut next = Arr2d::new(n, 26, n);
    let mut ans = 0;
    let mut best = vec![0; 26];
    for i in (0..n).rev() {
        for j in 0..n {
            next[(j, map[(i, j)] as usize - b'A' as usize)] = i;
        }
        for j in 0..n {
            best.fill(n);
            for k in j..n {
                let mut cur = i;
                for l in 0..26 {
                    if present.is_set(l) {
                        best[l].minim(next[(k, l)]);
                        cur.maxim(best[l]);
                    }
                }
                ans += n - cur;
            }
        }
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
