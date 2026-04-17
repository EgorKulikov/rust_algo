//{"name":"D - Souvenir Combinations","group":"AtCoder - AtCoder Weekday Contest 0049 Beta","url":"https://atcoder.jp/contests/awc0049/tasks/awc0049_d","interactive":false,"timeLimit":2000,"tests":[{"input":"2 3 3\n3 1\n2 4 1\n","output":"22\n"},{"input":"4 4 5\n5 3 7 1\n4 6 2 8\n","output":"196\n"},{"input":"5 6 8\n10 20 30 40 50\n3 6 9 12 15 18\n","output":"5040\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BinaryHeap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let a = input.read_long_vec(n).sorted().reversed();
    let b = input.read_long_vec(m).sorted().reversed();

    let mut at = vec![0; n];
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((a[i] * b[0], i));
    }
    let mut ans = 0;
    for _ in 0..k {
        let (cur, i) = heap.pop().unwrap();
        ans += cur;
        at[i] += 1;
        if at[i] < m {
            heap.push((a[i] * b[at[i]], i));
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
