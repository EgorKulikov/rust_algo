//{"name":"P11229 [CSP-J 2024] 小木棍（民间数据）","group":"Luogu","url":"https://www.luogu.com.cn/problem/P11229?contestId=209924","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1\n2\n3\n6\n18\n","output":"-1\n1\n7\n6\n208\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P11229CSPJ2024"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let (prefix, eights) = match n % 7 {
        0 => (8, n - 7),
        1 => {
            if n == 1 {
                out.print_line(-1);
                return;
            } else {
                (10, n - 8)
            }
        }
        2 => (1, n - 2),
        3 => {
            if n == 3 {
                (7, 0)
            } else if n == 10 {
                (22, 0)
            } else {
                (200, n - 17)
            }
        }
        4 => {
            if n == 4 {
                (4, 0)
            } else {
                (20, n - 11)
            }
        }
        5 => (2, n - 5),
        6 => (6, n - 6),
        _ => unreachable!(),
    };
    assert_eq!(eights % 7, 0);
    out.print(prefix);
    for _ in 0..eights / 7 {
        out.print(8);
    }
    out.print_line(());
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
