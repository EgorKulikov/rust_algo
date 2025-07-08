//{"name":"C - Large Queue","group":"AtCoder - Denso Create Programming Contest 2025（AtCoder Beginner Contest 413）","url":"https://atcoder.jp/contests/abc413/tasks/abc413_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 2 3\n1 4 5\n2 3\n1 6 2\n2 5\n","output":"11\n19\n"},{"input":"10\n1 75 22\n1 81 72\n1 2 97\n1 84 82\n1 2 32\n1 39 57\n2 45\n1 40 16\n2 32\n2 42\n","output":"990\n804\n3024\n"},{"input":"10\n1 160449218 954291757\n2 17217760\n1 353195922 501899080\n1 350034067 910748511\n1 824284691 470338674\n2 180999835\n1 131381221 677959980\n1 346948152 208032501\n1 893229302 506147731\n2 298309896\n","output":"16430766442004320\n155640513381884866\n149721462357295680\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::VecDeque;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let q = input.read_size();

    let mut a = VecDeque::new();
    for _ in 0..q {
        let t = input.read_int();
        match t {
            1 => {
                let c = input.read_long();
                let x = input.read_long();
                a.push_back((c, x));
            }
            2 => {
                let mut c = input.read_long();
                let mut sum = 0;
                while let Some((cc, x)) = a.front_mut() {
                    if c < *cc {
                        sum += c * *x;
                        *cc -= c;
                        break;
                    } else {
                        sum += *cc * *x;
                        c -= *cc;
                        a.pop_front();
                    }
                }
                out.print_line(sum);
            }
            _ => unreachable!(),
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
