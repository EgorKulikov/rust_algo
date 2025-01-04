//{"name":"C. Поездка на олимпиаду","group":"Codeforces - Hello 2025","url":"https://codeforces.com/contest/2057/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n0 2\n0 8\n1 3\n6 22\n128 137\n69 98\n115 127\n0 1073741823\n","output":"1 2 0\n8 7 1\n2 1 3\n7 16 11\n134 132 137\n98 85 76\n123 121 118\n965321865 375544086 12551794\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPoezdkaNaOlimpiadu"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l = input.read_unsigned();
    let r = input.read_unsigned();

    for i in (0..30).rev() {
        if l.is_set(i) != r.is_set(i) {
            let b = (r >> i) << i;
            let a = b - 1;
            let c = if a == l { b + 1 } else { a - 1 };
            out.print_line((a, b, c));
            return;
        }
    }
    unreachable!();
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
