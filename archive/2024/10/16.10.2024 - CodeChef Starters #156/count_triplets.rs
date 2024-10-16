//{"name":"Count Triplets","group":"CodeChef - START156A","url":"https://www.codechef.com/START156A/problems/COUNTTRIP","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5\n1 2 1 2 1\n3\n1 2 3\n4\n1 4 2 3\n","output":"21\n17\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CountTriplets"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    let mut ans = 0;
    for i in 0..n {
        for j in i..(i + 100).min(n) {
            let delta = a[i].abs_diff(a[j]);
            if delta < j - i || delta % 2 != (j - i) % 2 {
                continue;
            }
            let shift = (delta - (j - i)) / 2;
            let mut cur = 0;
            if shift == 0 {
                cur += j - i + 1;
            } else {
                if i >= shift {
                    cur += 1;
                }
                if j + shift < n {
                    cur += 1;
                }
            }
            if i != j {
                ans += 2 * cur;
            } else {
                ans += cur;
            }
        }
    }
    out.print_line(ans);
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
