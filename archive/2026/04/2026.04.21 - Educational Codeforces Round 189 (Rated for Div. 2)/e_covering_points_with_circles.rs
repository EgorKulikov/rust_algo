//{"name":"E. Covering Points with Circles","group":"Codeforces - Educational Codeforces Round 189 (Rated for Div. 2)","url":"https://codeforces.com/contest/2225/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4 100\n0 0\n0 100\n100 0\n100 100\n","output":"1\n70 70\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::{Random, RandomTrait};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::integer_sqrt::IntegerSqrt;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let r = input.read_long();
    let pts = input.read_long_pair_vec(n);

    let dy = (r * r * 3).upper_sqrt();

    let mut rnd = Random::new();
    loop {
        let cx = rnd.gen_range(0..2 * r) - 2 * r * 200_000;
        let cy = rnd.gen_range(0..dy) - dy * 200_000;

        let mut good = 0;
        let mut set = FxHashSet::default();
        'outer: for (x, y) in pts.copy_iter() {
            let ay = y - cy;

            let row = ay / dy;
            for row in row..=row + 1 {
                let cx = cx + row % 2 * r;
                let ax = x - cx;
                let col = ax / (2 * r);
                for col in col..=col + 1 {
                    let rx = cx + col * 2 * r;
                    let ry = cy + row * dy;
                    if (rx - x) * (rx - x) + (ry - y) * (ry - y) <= r * r {
                        set.insert((rx, ry));
                        good += 1;
                        continue 'outer;
                    }
                }
            }
        }
        if good * 100 >= n * 89 {
            out.print_line(set.len());
            for (x, y) in set {
                out.print_line((x, y));
            }
            return;
        }
    }
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
