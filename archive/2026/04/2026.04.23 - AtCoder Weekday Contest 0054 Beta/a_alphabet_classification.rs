//{"name":"A - Alphabet Classification","group":"AtCoder - AtCoder Weekday Contest 0054 Beta","url":"https://atcoder.jp/contests/awc0054/tasks/awc0054_a","interactive":false,"timeLimit":2000,"tests":[{"input":"5\napple\napricot\nbanana\navocado\nberry\n","output":"3\n"},{"input":"6\ncat\ncar\ndog\ndove\neel\negg\n","output":"2\n"},{"input":"12\nalpha\natom\nangle\nbeta\nbanana\nboat\nboat\ncat\ncircle\ncider\napple\nant\n","output":"5\n"},{"input":"30\nmoon\nmap\nmilk\nmango\nmint\nmouse\nmelon\nmild\nsun\nsand\nsea\nstone\nsmile\nsound\nsoup\napple\narrow\nant\nbook\nbird\nblue\ncat\ncloud\ncamel\ndog\ndrum\necho\nearth\nzebra\nzero\n","output":"8\n"},{"input":"1\nabcdefghijklmnopqrst\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::array_map::ArrayMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str_vec(n);

    let mut map = ArrayMap::<_, usize>::new(b'a'..=b'z');
    for s in s {
        map[s[0]] += 1;
    }
    out.print_line(map.iter().max());
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
