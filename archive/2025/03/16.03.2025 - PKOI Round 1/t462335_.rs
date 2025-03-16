//{"name":"T462335 必胜","group":"Luogu","url":"https://www.luogu.com.cn/problem/T462335?contestId=183510","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1\n2\n2\n2 2\n1\n4\n","output":"Lucky_Holmes\nDraw\nAngry_Waston\n"},{"input":"4\n3\n9 10 15\n3\n9 10 30\n2\n11 14\n4\n11 4 5 14\n","output":"Angry_Waston\nLucky_Holmes\nLucky_Holmes\nAngry_Waston\n"},{"input":"3\n8\n7 12 15 17 21 23 30 31\n10\n10 11 12 13 14 15 16 17 18 19\n6\n16 17 18 16 17 18\n","output":"Angry_Waston\nAngry_Waston\nAngry_Waston\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::primes::sieve::divisor_table;

type PreCalc = Vec<usize>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let b = Vec::with_gen(n, |i| data[a[i]]);
    let num_odd = b.iter().filter(|&&x| x % 2 == 1).count();
    let num_ones = b.iter().filter(|&&x| x == 1).count();
    let num_even = n - num_odd;

    if num_odd % 2 == 1 || num_ones % 2 == 1 {
        out.print_line("Lucky_Holmes");
    } else if num_even == 0 {
        out.print_line("Draw");
    } else {
        out.print_line("Angry_Waston");
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = Vec::with_capacity(10_000_001);
    pre_calc.push(0);
    pre_calc.push(0);
    let dt = divisor_table(10_000_001);
    for i in 2..dt.len() {
        pre_calc.push(1 + pre_calc[i / dt[i]]);
    }

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
