//{"name":"Timebomb","group":"Kattis","url":"https://open.kattis.com/problems/timebomb","interactive":false,"timeLimit":1000,"tests":[{"input":"***   * * * *** *** *** ***\n* *   * * *   *   *   * *  \n* *   * *** *** *** *** ***\n* *   *   * *     * *   * *\n***   *   * *** *** *** ***\n","output":"BEER!!\n"},{"input":"  *   * *** *** *** * *\n  *   * **    * * * * *\n  *   * *** *** *** ***\n  *   * *   *   * *   *\n  *   * *** *** ***   *\n","output":"BOOM!!\n"},{"input":"*** ***   * *** ***   *\n*   * *   * * *   *   *\n*** * *   * *** ***   *\n  * * *   *   * *     *\n*** ***   * *** ***   *\n","output":"BOOM!!\n"},{"input":"*** *** *** * * ***\n  *   *   * * * * *\n***   * *** *** ***\n*     *   *   * * *\n***   * ***   * ***\n","output":"BEER!!\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_line_vec(5);

    let digits = [
        [b"***", b"* *", b"* *", b"* *", b"***"],
        [b"  *", b"  *", b"  *", b"  *", b"  *"],
        [b"***", b"  *", b"***", b"*  ", b"***"],
        [b"***", b"  *", b"***", b"  *", b"***"],
        [b"* *", b"* *", b"***", b"  *", b"  *"],
        [b"***", b"*  ", b"***", b"  *", b"***"],
        [b"***", b"*  ", b"***", b"* *", b"***"],
        [b"***", b"  *", b"  *", b"  *", b"  *"],
        [b"***", b"* *", b"***", b"* *", b"***"],
        [b"***", b"* *", b"***", b"  *", b"***"],
    ];

    let mut res = 0;
    for i in s[0].indices().step_by(4) {
        res *= 10;
        let mut found = false;
        for j in 0..10 {
            let mut ok = true;
            for k in 0..5 {
                if &s[k][i..i + 3] != digits[j][k] {
                    ok = false;
                    break;
                }
            }
            if ok {
                res += j;
                found = true;
                break;
            }
        }
        if !found {
            out.print_line("BOOM!!");
            return;
        }
    }
    out.print_line(if res % 6 == 0 { "BEER!!" } else { "BOOM!!" });
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
