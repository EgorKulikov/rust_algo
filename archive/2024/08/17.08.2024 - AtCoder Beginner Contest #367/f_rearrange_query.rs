//{"name":"F - Rearrange Query","group":"AtCoder - AtCoder Beginner Contest 367","url":"https://atcoder.jp/contests/abc367/tasks/abc367_f","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n1 2 3 2 4\n2 3 1 4 2\n1 3 1 3\n1 2 3 5\n1 4 2 5\n1 5 1 5\n","output":"Yes\nNo\nNo\nYes\n"},{"input":"4 4\n4 4 4 4\n4 4 4 4\n1 2 2 3\n3 3 1 1\n1 3 1 4\n1 4 2 3\n","output":"Yes\nYes\nNo\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FRearrangeQuery"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::random::random;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::collections::HashMap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n);
    let b = input.read_size_vec(n);

    let mut map = HashMap::new();
    let mut get = |id: usize| -> u64 { *map.entry(id).or_insert(random().gen()) };
    let mut xa = vec![0];
    let mut cur = 0u64;
    for a in a {
        cur = cur.overflowing_add(get(a)).0;
        xa.push(cur);
    }
    let mut xb = vec![0];
    let mut cur = 0u64;
    for b in b {
        cur = cur.overflowing_add(get(b)).0;
        xb.push(cur);
    }

    out.set_bool_output(BoolOutput::YesNo);
    for _ in 0..q {
        let l1 = input.read_size() - 1;
        let r1 = input.read_size();
        let l2 = input.read_size() - 1;
        let r2 = input.read_size();
        out.print_line(xa[r1].overflowing_sub(xa[l1]).0 == xb[r2].overflowing_sub(xb[l2]).0);
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
