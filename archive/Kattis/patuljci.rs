//{"name":"Patuljci","group":"Kattis","url":"https://open.kattis.com/problems/patuljci","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n8\n10\n13\n15\n19\n20\n23\n25\n","output":"7\n8\n10\n13\n19\n20\n23\n"},{"input":"8\n6\n5\n1\n37\n30\n28\n22\n36\n","output":"8\n6\n5\n1\n30\n28\n22\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Patuljci"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_int_vec(9);

    for i in usize::iter_all(9) {
        if i.count_ones() != 7 {
            continue;
        }
        let sum: i32 = a
            .iter()
            .enumerate()
            .filter(|(j, _)| i.is_set(*j))
            .map(|(_, &x)| x)
            .sum();
        if sum == 100 {
            for j in 0..9 {
                if i.is_set(j) {
                    out.print_line(a[j]);
                }
            }
            return;
        }
    }
    unreachable!();
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
