//{"name":"P4 - Magic Barrier","group":"DMOJ - OTHS Coding Competition 2","url":"https://dmoj.ca/problem/othscc2p4","interactive":false,"timeLimit":3000,"tests":[{"input":"3 3 3\n1 7 11\n10 5 9\n4 3 2\n10 1 1 1 2\n3 2 2 3 3\n100 2 2 3 3\n","output":"no\nyes\nno\n"},{"input":"2 3 2\n1 2 3\n4 5 6\n1 1 1 1 1\n1 2 2 2 3\n","output":"yes\nno\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P4MagicBarrier"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let a = input.read_size_table(n, m);

    let mut map = DefaultHashMap::new();
    for i in 0..n {
        for j in 0..m {
            map[a[(i, j)]] = (i + 1, j + 1);
        }
    }

    for _ in 0..q {
        let k = input.read_size();
        let r1 = input.read_size();
        let c1 = input.read_size();
        let r2 = input.read_size();
        let c2 = input.read_size();
        let (r, c) = map[k];
        if r1 <= r && r <= r2 && c1 <= c && c <= c2 {
            out.print_line("yes");
        } else {
            out.print_line("no");
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
