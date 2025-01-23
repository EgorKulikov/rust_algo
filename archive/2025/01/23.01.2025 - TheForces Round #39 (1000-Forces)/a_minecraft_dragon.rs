//{"name":"A. Minecraft Dragon","group":"Codeforces - TheForces Round #39 (1000-Forces)","url":"https://codeforces.com/gym/105672/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4 3 2\n4 3 1\n10 7 3\n96 101 23\n","output":"1\n0\n3\n17\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();
    let m = input.read_int();
    let k = input.read_int();

    let mut we = m;
    let mut dragon = n;
    let mut attacks = 0;
    let mut heals = 0;
    loop {
        attacks += 1;
        if attacks % 3 == 0 {
            dragon -= 1;
        }
        dragon -= 1;
        if dragon <= 0 {
            break;
        }
        if we <= k {
            we = m;
            heals += 1;
        }
        we -= k;
    }
    out.print_line(heals);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
