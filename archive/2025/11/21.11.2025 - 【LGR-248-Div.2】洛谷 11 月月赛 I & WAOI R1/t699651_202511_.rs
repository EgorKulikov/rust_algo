//{"name":"T699651 [语言月赛 202511] 太空曼波","group":"Luogu","url":"https://www.luogu.com.cn/problem/T699651?contestId=291890","interactive":false,"timeLimit":1000,"tests":[{"input":"4\nmanbo\nmanba\nnba\nabc\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::hash::{SimpleHash, StringHash};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str_vec(n);

    let h = Vec::with_gen(n, |i| SimpleHash::new(&s[i]));
    let mut prefixes = DefaultHashMap::new(0);
    let mut suffixes = DefaultHashMap::new(0);
    for i in 0..n {
        for j in s[i].indices() {
            prefixes[h[i].hash(..=j)] += 1;
            suffixes[h[i].hash(j..)] += 1;
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 1..s[i].len() {
            if prefixes[h[i].hash(..j)] > 1 && suffixes[h[i].hash(j..)] > 1 {
                ans += 1;
                break;
            }
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
