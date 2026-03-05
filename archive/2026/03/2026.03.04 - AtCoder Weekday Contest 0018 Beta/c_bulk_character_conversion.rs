//{"name":"C - Bulk Character Conversion","group":"AtCoder - AtCoder Weekday Contest 0018 Beta","url":"https://atcoder.jp/contests/awc0018/tasks/awc0018_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\nabc\ndog\nbanana\na b\nb c\n","output":"ccc\ndog\nccncnc\n"},{"input":"2 3\nhello\nworld\nl x\no a\nx l\n","output":"hella\nwarld\n"},{"input":"5 5\nabcdefghij\nprogramming\ncontest\nalgorithm\ndatastructure\na z\ne a\nz e\nt s\ns t\n","output":"ebcdafghij\nprogremming\ncontatt\nelgorithm\ndetettructura\n"},{"input":"10 8\nthequickbrownfoxjumpsoverthelazydog\nabcdefghijklmnopqrstuvwxyz\naabbccddee\nzzzzzyyyyyxxxxx\nhelloworldfromcompetitiveprogramming\nsubstitutioncipherexample\nreplacementoperation\nqueriesandfiles\nmultiplestrings\nfinaloutputtest\na b\nb c\nc d\nd e\ne f\nf g\ng h\nz a\n","output":"thhquihkhrownhoxjumpsovhrthhlhayhoh\nhhhhhhhhijklmnopqrstuvwxya\nhhhhhhhhhh\naaaaayyyyyxxxxx\nhhlloworlhhromhomphtitivhprohrhmminh\nsuhstitutionhiphhrhxhmplh\nrhplhhhmhntophrhtion\nquhrihshnhhilhs\nmultiplhstrinhs\nhinhloutputthst\n"},{"input":"1 1\na\na b\n","output":"b\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

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
    let q = input.read_size();
    let s = input.read_str_vec(n);
    let ab = input.read_vec::<(u8, u8)>(q);

    let mut map = ArrayMap::with_gen(b'a'..=b'z', |c| c);
    for (a, b) in ab {
        for c in b'a'..=b'z' {
            if map[c] == a {
                map[c] = b;
            }
        }
    }
    for s in s {
        for c in s {
            out.print(map[c]);
        }
        out.print_line(());
    }
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
