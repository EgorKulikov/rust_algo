//{"name":"D - Tiling","group":"AtCoder - Monoxer Programming Contest 2024（AtCoder Beginner Contest 345）","url":"https://atcoder.jp/contests/abc345/tasks/abc345_d","interactive":false,"timeLimit":2000,"tests":[{"input":"5 5 5\n1 1\n3 3\n4 4\n2 3\n2 5\n","output":"Yes\n"},{"input":"1 1 2\n2 3\n","output":"No\n"},{"input":"1 2 2\n1 1\n","output":"No\n"},{"input":"5 3 3\n1 1\n2 2\n2 2\n2 2\n2 2\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTiling"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let h = input.read_size();
    let w = input.read_size();
    let tiles = input.read_size_pair_vec(n);

    let mut state = Arr2d::new(h, w, false);
    let mut rec = RecursiveFunction::new(|rec, mask: u32| -> bool {
        for i in 0..h {
            for j in 0..w {
                if state[(i, j)] {
                    continue;
                }
                for k in 0..n {
                    if mask.is_set(k) {
                        continue;
                    }
                    let mut do_try = |a: usize, b: usize| -> bool {
                        if i + a > h || j + b > w {
                            return false;
                        }
                        for x in i..i + a {
                            for y in j..j + b {
                                if state[(x, y)] {
                                    return false;
                                }
                            }
                        }
                        for x in i..i + a {
                            for y in j..j + b {
                                state[(x, y)] = true;
                            }
                        }
                        if rec.call(mask.with_bit(k)) {
                            return true;
                        }
                        for x in i..i + a {
                            for y in j..j + b {
                                state[(x, y)] = false;
                            }
                        }
                        false
                    };
                    let (a, b) = tiles[k];
                    if do_try(a, b) || do_try(b, a) {
                        return true;
                    }
                }
                return false;
            }
        }
        true
    });
    out.set_bool_output(BoolOutput::YesNo);
    out.print_line(rec.call(0));
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
