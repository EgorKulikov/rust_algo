//{"name":"T539827 202411H Enemy","group":"Luogu","url":"https://www.luogu.com.cn/problem/T539827?contestId=213071","interactive":false,"timeLimit":1000,"tests":[{"input":"5 8\n#A#AH#B#\n#A##B#B#\n##AA##BB\nAAA###BB\nAAA#ABBB\n","output":"AA##H##B\nBB######\n########\n########\n####A###\n"},{"input":"3 2\n#A\n##\n#A\n","output":"#A\n##\n#A\n"},{"input":"3 3\nAAB\nA#B\nABB\n","output":"#AA\n###\nBB#\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T539827202411HEnemy"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dCharWrite, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let t = input.read_char_table(n, m);

    let mut u = Arr2d::new(n, m, b'#');
    for i in 0..n {
        let a = t.row(i).filter(|&&c| c == b'A').count();
        let b = t.row(i).filter(|&&c| c == b'B').count();
        if let Some(pos) = t.row(i).position(|&c| c == b'H') {
            u[(i, pos)] = b'H';
            for j in 0..a {
                u[(i, j)] = b'A';
            }
            for j in 0..b {
                u[(i, m - 1 - j)] = b'B';
            }
        } else {
            if a > b {
                for j in 0..a {
                    u[(i, m - 1 - j)] = b'A';
                }
            } else if a < b {
                for j in 0..b {
                    u[(i, j)] = b'B';
                }
            }
        }
    }
    let mut v = Arr2d::new(n, m, b'#');
    for i in 0..n {
        for j in 0..m {
            if (u[(i, j)] == b'A' || u[(i, j)] == b'B')
                && (i > 0 && u[(i - 1, j)] == u[(i, j)] || i < n - 1 && u[(i + 1, j)] == u[(i, j)])
            {
                v[(i, j)] = b'#';
            } else {
                v[(i, j)] = u[(i, j)];
            }
        }
    }
    out.print_table(&v);
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
