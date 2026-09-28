use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let h = input.read_size();
    let w = input.read_size();
    let c = input.read_long_table(h, w);

    let mut ans = Arr2d::new(h, w, 0);
    let mut started = Arr2d::with_gen(h, w, |i, j| i == 0 || j == 0 || i == h - 1 || j == w - 1);
    let mut heap = BinaryHeap::new();
    for (i, j) in started.indices() {
        if started[(i, j)] {
            heap.push((Reverse(c[(i, j)]), i, j));
        }
    }
    while let Some((Reverse(at), i, j)) = heap.pop() {
        ans[(i, j)] = at;
        for (r, col) in D4::iter(i, j, h, w) {
            if !started[(r, col)] {
                started[(r, col)] = true;
                heap.push((Reverse(c[(r, col)] + at), r, col));
            }
        }
    }

    let q = input.read_size();
    for _ in 0..q {
        let r = input.read_size() - 1;
        let c = input.read_size() - 1;
        out.print_line(ans[(r, c)]);
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
