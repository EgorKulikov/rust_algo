//{"name":"E - Repeat Playback and Partial Matching","group":"AtCoder - AtCoder Weekday Contest 0047 Beta","url":"https://atcoder.jp/contests/awc0047/tasks/awc0047_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5 5\nabcab\n1 3\n4 7\n5 6\n3 7\n11 13\n","output":"Yes\nNo\nNo\nNo\nYes\n"},{"input":"3 4\naab\n1 4\n4 6\n2 4\n1 3\n","output":"No\nYes\nNo\nYes\n"},{"input":"10 8\nabracadabr\n1 10\n11 20\n5 14\n3 5\n1 11\n15 18\n7 13\n100 103\n","output":"Yes\nYes\nNo\nYes\nNo\nYes\nNo\nNo\n"},{"input":"20 10\nabcdeabcdfabcdeabcdf\n1 10\n11 20\n5 16\n21 40\n1 21\n18 23\n3 7\n100000000000000000 1000000000000000000\n35 39\n6 10\n","output":"Yes\nYes\nYes\nYes\nNo\nYes\nYes\nNo\nYes\nYes\n"},{"input":"1 3\na\n1 1\n1000000000000000000 1000000000000000000\n999999999999999999 1000000000000000000\n","output":"Yes\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use algo_lib::string::string_algorithms::z_algorithm::ZAlgorithm;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_str();

    let z = s.z_algorithm();
    let rz = s.unwrap().reversed().z_algorithm().reversed();
    let mut max_fwd = vec![0; n];
    for i in 1..n {
        max_fwd[rz[i - 1]].maxim(z[i]);
    }
    for i in (1..n).rev() {
        let c = max_fwd[i];
        max_fwd[i - 1].maxim(c);
    }

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();
        let delta = l / n;
        let l = l - delta * n;
        let r = r - delta * n;
        out.print_line(r <= n || l != 0 && max_fwd[n - l] >= r - n);
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
