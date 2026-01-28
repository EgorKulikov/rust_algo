//{"name":"Deleting Substrings","group":"CodeChef - START223A","url":"https://www.codechef.com/START223A/problems/DELSUB2","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n3 2\nabc\nac\n2 2\nab\nba\n4 4\nabcd\nabcd\n5 2\nababa\nba\n5 2\nababa\nbb\n","output":"1\n-1\n0\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let s = input.read_str();
    let t = input.read_str();

    let mut can = Memoization2d::new(n + 1, m + 1, |mem, i, j| {
        if j == 0 || i == 0 {
            0
        } else {
            if s[i - 1] != t[j - 1] {
                0
            } else {
                1 + mem.call(i - 1, j - 1)
            }
        }
    });
    let mut cost = vec![None; m + 1];
    cost[0] = Some(0);
    for i in 0..=n {
        for j in (1..=m).rev() {
            if i + j > n {
                continue;
            }
            let len = can.call(i + j, j);
            let pos = j - len;
            if let Some(val) = cost[pos] {
                let mut add = 1;
                if pos == 0 && i == 0 {
                    add -= 1;
                }
                if j == m && i + j != n {
                    add += 1;
                }
                cost[j].minim(val + add);
            }
        }
    }
    out.print_line(cost[m]);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
