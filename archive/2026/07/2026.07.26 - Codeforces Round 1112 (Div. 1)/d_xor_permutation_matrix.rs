use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_size();

    if n == 1 {
        out.print_line(0);
        return;
    }
    if n == 2 && x == 1 {
        out.print_line(-1);
        return;
    }
    // if n == 4 && x == 3 {
    //     out.print_line((0, 1, 3, 2));
    //     out.print_line((1, 3, 2, 0));
    //     out.print_line((3, 2, 0, 1));
    //     out.print_line((2, 0, 1, 3));
    //     return;
    // }
    if n.count_ones() != 1 {
        out.print_line(-1);
        return;
    }
    if x == n - 1 {
        let z = [0, 1, 3, 2];
        let mut ans = Arr2d::with_gen(4, 4, |i, j| z[(i + j) % 4]);
        for i in 2..n.trailing_zeros() as usize {
            let next = Arr2d::with_gen(2 * ans.d1(), 2 * ans.d2(), |a, b| {
                let mut res = ans[(a % ans.d1(), b % ans.d2())];
                let top_a = a / ans.d1();
                let top_b = b / ans.d1();
                if top_a + top_b != 1 {
                    if a % 2 == 1 && b % 2 == 0 {
                        res.flip_bit(i);
                    }
                } else {
                    if a % 2 == 0 || b % 2 == 1 {
                        res.flip_bit(i);
                    }
                }
                res
            });
            ans = next;
        }
        out.print_line(ans);
        return;
    }
    let mut ans = Arr2d::new(1, 1, 0);
    for i in 0..n.trailing_zeros() as usize {
        if !x.is_set(i) {
            let next = Arr2d::with_gen(2 * ans.d1(), 2 * ans.d1(), |a, b| {
                let mut res = ans[(a % ans.d1(), b % ans.d2())];
                if a / ans.d1() + b / ans.d2() == 1 {
                    res.flip_bit(i);
                }
                res
            });
            ans = next;
        }
    }
    for i in 0..n.trailing_zeros() as usize {
        if x.is_set(i) {
            let next = Arr2d::with_gen(2 * ans.d1(), 2 * ans.d2(), |a, b| {
                let mut res = ans[(a % ans.d1(), b % ans.d2())];
                let top_a = a / ans.d1();
                let top_b = b / ans.d1();
                if top_a + top_b != 1 {
                    if a % 2 == 1 && b % 2 == 0 {
                        res.flip_bit(i);
                    }
                } else {
                    if a % 2 == 0 || b % 2 == 1 {
                        res.flip_bit(i);
                    }
                }
                res
            });
            ans = next;
        }
    }
    out.print_line(ans);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
