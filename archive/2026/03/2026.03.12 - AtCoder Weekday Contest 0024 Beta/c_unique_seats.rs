//{"name":"C - Unique Seats","group":"AtCoder - AtCoder Weekday Contest 0024 Beta","url":"https://atcoder.jp/contests/awc0024/tasks/awc0024_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\nabc\nbca\ncab\n","output":"abcbcacab\n"},{"input":"2 2\naa\naa\n","output":""},{"input":"4 6\nabcdef\naghijk\nalmnop\naqrstu\n","output":"bcdefghijklmnopqrstu\n"},{"input":"8 10\nabcdefghij\nabcdefghij\nklmnopqrst\nklmnopqrst\nuvwxabcdef\nuvwxabcdef\nghijklmnop\nghijklmnop\n","output":""},{"input":"1 1\nz\n","output":"z\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::array_map::ArrayMap;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let h = input.read_size();
    let w = input.read_size();
    let g = input.read_char_table(h, w);

    let rows = Vec::with_gen(h, |i| {
        let mut res = ArrayMap::<_, i32>::new(b'a'..=b'z');
        for &c in g.row(i) {
            res[c] += 1;
        }
        res
    });
    let cols = Vec::with_gen(w, |j| {
        let mut res = ArrayMap::<_, i32>::new(b'a'..=b'z');
        for i in 0..h {
            res[g[(i, j)]] += 1;
        }
        res
    });
    let mut ans = Str::new();
    for (i, j) in g.indices() {
        let c = g[(i, j)];
        if rows[i][c] == 1 && cols[j][c] == 1 {
            ans.push(c);
        }
    }
    out.print_line(ans);
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
