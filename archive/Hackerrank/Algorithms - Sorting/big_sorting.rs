//{"name":"Big Sorting","group":"HackerRank - Algorithms - Sorting","url":"https://www.hackerrank.com/challenges/big-sorting/problem?utm_campaign=challenge-recommendation&utm_medium=email&utm_source=24-hour-campaign","interactive":false,"timeLimit":4000,"tests":[{"input":"6\n31415926535897932384626433832795\n1\n3\n10\n3\n5\n","output":"1\n3\n3\n5\n10\n31415926535897932384626433832795\n"},{"input":"8\n1\n2\n100\n12303479849857341718340192371\n3084193741082937\n3084193741082938\n111\n200\n","output":"1\n2\n100\n111\n200\n3084193741082937\n3084193741082938\n12303479849857341718340192371\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BigSorting"}}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::unsigned_big_int::UBigInt;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let arr: Vec<UBigInt> = input.read_vec(n).sorted();

    out.print_per_line(&arr);
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
