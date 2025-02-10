//{"name":"EpigDanceOff","group":"Kattis","url":"https://open.kattis.com/problems/epigdanceoff","interactive":false,"timeLimit":2000,"tests":[{"input":"13 50\n____$$$_______$$$______$$$________$$$______$$$____\n____$$$_______$$$______$$$________$$$______$$$____\n_____$_________$________$__________$________$_____\n___$_$_$_____$_$_$____$_$_$______$_$_$____$_$_$___\n__$__$_$____$__$__$___$_$__$____$__$__$___$_$__$__\n_$____$$____$__$__$___$$____$___$__$__$___$$____$_\n$_____$$___$___$___$__$$_____$_$___$___$__$$_____$\n_____$_$______$_$_____$_$_________$_$_____$_$_____\n____$___$____$___$____$___$______$___$____$___$___\n___$____$___$_____$___$____$____$_____$___$____$__\n__$_____$___$_____$___$_____$___$_____$___$_____$_\n__$_____$___$_____$___$_____$___$_____$___$_____$_\n_$$_____$$_$$_____$$_$$_____$$_$$_____$$_$$_____$$\n","output":"5\n"},{"input":"2 2\n$$\n_$\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let t = input.read_char_table(n, m);

    out.print_line(1 + (0..m).filter(|&j| t.col(j).all(|&c| c == b'_')).count());
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
