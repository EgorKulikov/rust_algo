//{"name":"Rectangle Coloring","group":"CodeChef - START221A","url":"https://www.codechef.com/START221A/problems/LMP5","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n6 3 9 5 4\n4 4 4 8 4\n2 3 1 1 8\n3 3 1 10 1\n","output":"5\n4\n5\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::next_permutation::NextPermutation;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l = input.read_size();
    let w = input.read_size();
    let rgb = input.read_size_vec(3).sorted();

    let mut sides = vec![l, l, w, w].sorted();
    let mut ans = 6;
    loop {
        let mut rgb = rgb.clone();
        loop {
            let mut cur = 0;
            let mut i = 0;
            let mut j = 0;
            let mut rem_i = 0;
            let mut rem_j = 0;
            while i < 4 || j < 3 {
                if rem_i == 0 {
                    rem_i += sides[i];
                    i += 1;
                }
                if rem_j == 0 {
                    rem_j += rgb[j];
                    j += 1;
                }
                if rem_i >= rem_j {
                    rem_i -= rem_j;
                    rem_j = 0;
                } else {
                    rem_j -= rem_i;
                    rem_i = 0;
                }
                cur += 1;
            }
            ans.minim(cur);
            if !rgb.next_permutation() {
                break;
            }
        }
        if !sides.next_permutation() {
            break;
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
