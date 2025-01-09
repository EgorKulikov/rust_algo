//{"name":"Volim","group":"Kattis","url":"https://open.kattis.com/problems/volim","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n5\n20 T\n50 T\n80 T\n50 T\n30 T\n","output":"5\n"},{"input":"3\n5\n100 T\n100 N\n100 T\n100 T\n100 N\n","output":"4\n"},{"input":"5\n6\n70 T\n50 P\n30 N\n50 T\n30 P\n80 T\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Volim"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut k = input.read_size() - 1;
    let n = input.read_size();
    let questions: Vec<(i32, u8)> = input.read_vec(n);

    let mut rem = 210;
    for (time, ans) in questions {
        if time > rem {
            out.print_line(k + 1);
            return;
        }
        rem -= time;
        if ans == b'T' {
            k = (k + 1) % 8;
        }
    }
    out.print_line(k + 1);
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
