//{"name":"D. Проверка характеристик","group":"Codeforces - Educational Codeforces Round 170 (Rated for Div. 2)","url":"https://codeforces.com/contest/2025/problem/D","interactive":false,"timeLimit":2500,"tests":[{"input":"10 5\n0 1 0 2 0 -3 0 -4 0 -5\n","output":"3\n"},{"input":"3 1\n1 -1 0\n","output":"0\n"},{"input":"9 3\n0 0 1 0 2 -3 -2 -2 1\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DProverkaKharakteristik"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::BackwardSlice;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Ordering;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _m = input.read_size();
    let r = input.read_long_vec(n).do_with(|r| r.push(0));

    let mut ans = vec![0];
    let mut str = vec![0];
    let mut int = vec![0];
    let mut total_int = 0;
    let mut pts = 0;
    for i in r {
        match i.cmp(&0) {
            Ordering::Less => {
                let i = -i as usize;
                if pts >= i {
                    str[i] += 1;
                }
            }
            Ordering::Equal => {
                let mut cur = total_int;
                for j in 0..=pts {
                    cur += str[j];
                    ans[j] += cur;
                    cur -= int[j];
                }
                str.fill(0);
                str.push(0);
                int.fill(0);
                int.push(0);
                total_int = 0;
                ans.push(ans.backward()[0]);
                for j in (1..=pts).rev() {
                    let cand = ans[j - 1];
                    ans[j].maxim(cand);
                }
                pts += 1;
            }
            Ordering::Greater => {
                let i = i as usize;
                if pts >= i {
                    int[pts - i] += 1;
                    total_int += 1;
                }
            }
        }
    }
    out.print_line(ans.into_iter().max());
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
