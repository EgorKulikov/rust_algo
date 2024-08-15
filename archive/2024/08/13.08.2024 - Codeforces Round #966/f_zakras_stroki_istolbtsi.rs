//{"name":"F. Закрась строки и столбцы","group":"Codeforces - Codeforces Round 966 (Div. 3)","url":"https://codeforces.com/contest/2000/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n1 4\n6 3\n1 5\n4 4\n5 10\n1 1\n1 1\n1 1\n1 1\n1 1\n2 100\n1 2\n5 6\n3 11\n2 2\n3 3\n4 4\n3 25\n9 2\n4 3\n8 10\n4 18\n5 4\n8 5\n8 3\n6 2\n","output":"12\n14\n5\n-1\n17\n80\n35\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FZakrasStrokiIStolbtsi"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let ab = input.read_size_pair_vec(n);

    let mut mem = Memoization2d::new(n + 1, k + 1, |mem, id, done| -> usize {
        if id == n {
            if done == k {
                0
            } else {
                usize::MAX / 2
            }
        } else {
            let (mut a, mut b) = ab[id];
            let mut ans = None;
            let mut cost = 0;
            for i in done..=k {
                ans.minim(mem.call(id + 1, i) + cost);
                if a == 0 && b == 0 {
                    break;
                }
                cost += a.min(b);
                if a > b {
                    a -= 1;
                } else {
                    b -= 1;
                }
            }
            ans.unwrap()
        }
    });
    let ans = mem.call(0, 0);
    if ans >= usize::MAX / 2 {
        out.print_line(-1);
    } else {
        out.print_line(ans);
    }
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
