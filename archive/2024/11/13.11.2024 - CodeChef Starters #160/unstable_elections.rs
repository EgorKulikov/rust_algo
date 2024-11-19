//{"name":"Unstable Elections","group":"CodeChef - START160A","url":"https://www.codechef.com/START160A/problems/STABLELEC","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 5\n3 2 5 1 4\n2 4\n2 3 4 1\n4 3 1 2\n3 3\n1 2 3\n2 3 1\n2 3 1\n","output":"-1\n2\n4 3\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"UnstableElections"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let p = input
        .read_size_table(n, m)
        .do_with(|p| p.iter_mut().for_each(|x| *x -= 1));

    let mut delta = Arr2d::new(m, m, 0);
    let mut score = vec![0; m];
    for i in 0..n {
        for j in 0..m {
            score[p[(i, j)]] += m - j;
            for k in j + 1..m {
                delta[(p[(i, k)], p[(i, j)])] += 1;
            }
        }
    }
    let mut order = (0..m).collect::<Vec<_>>();
    order.sort_by_key(|&x| (Reverse(score[x]), x));
    fn check(score: &[usize], order: &[usize], exclude1: usize, exclude2: usize) -> bool {
        let mut cur_score = usize::MAX;
        let mut cur_id = 0;
        for &i in order {
            if i == exclude1 || i == exclude2 {
                continue;
            }
            if score[i] > cur_score || score[i] == cur_score && i < cur_id {
                return true;
            }
            cur_score = score[i];
            cur_id = i;
        }
        false
    }
    for i in 0..m {
        for j in 0..m {
            if i != j {
                score[j] -= delta[(i, j)];
            }
        }
        if check(&score, &order, i, m) {
            out.print_line(1);
            out.print_line(i + 1);
            return;
        }
        for j in i + 1..m {
            for k in 0..m {
                if k != i && k != j {
                    score[k] -= delta[(j, k)];
                }
            }
            if check(&score, &order, i, j) {
                out.print_line(2);
                out.print_line((i + 1, j + 1));
                return;
            }
            for k in 0..m {
                if k != i && k != j {
                    score[k] += delta[(j, k)];
                }
            }
        }
        for j in 0..m {
            if j != i {
                score[j] += delta[(i, j)];
            }
        }
    }
    out.print_line(-1);
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
