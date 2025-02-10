//{"name":"Closing the Loop","group":"Kattis","url":"https://open.kattis.com/problems/closingtheloop","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1\n5B\n4\n6R 1B 7R 3B\n7\n5B 4R 3R 2R 5R 4R 3R\n2\n20B 20R\n","output":"Case #1: 0\nCase #2: 13\nCase #3: 8\nCase #4: 38\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ClosingTheLoop"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::io::scan::Parse;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Reverse;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut red = Vec::new();
    let mut blue = Vec::new();
    for s in input.iter_str().take(n) {
        let col = s[Back(0)];
        let len: usize = s[..s.len() - 1].parse();
        match col {
            b'R' => red.push(len),
            b'B' => blue.push(len),
            _ => unreachable!(),
        }
    }

    red.sort_unstable_by_key(|&x| Reverse(x));
    blue.sort_unstable_by_key(|&x| Reverse(x));
    red.truncate(blue.len());
    blue.truncate(red.len());
    output!(
        out,
        "Case #{}: {}",
        test_case,
        red.copy_sum() + blue.copy_sum() - 2 * red.len()
    );
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
