//{"name":"G. MST with Matching","group":"Codeforces - Educational Codeforces Round 163 (Rated for Div. 2)","url":"https://codeforces.com/contest/1948/problem/G","interactive":false,"timeLimit":6000,"tests":[{"input":"4 10\n0 1 8 0\n1 0 1 0\n8 1 0 2\n0 0 2 0\n","output":"21\n"},{"input":"4 5\n0 1 8 0\n1 0 1 0\n8 1 0 2\n0 0 2 0\n","output":"14\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GMSTWithMatching"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let c = input.read_int();
    let w = input.read_int_table(n, n);

    const INFTY: i32 = i32::MAX / 3;

    let mut mem = Memoization2d::new(n, 1 << (n - 1), |mem, vert, mask| -> (i32, i32) {
        debug_assert!(!mask.is_set(vert));
        if mask == 0 {
            (0, INFTY)
        } else {
            let mut empty = INFTY;
            let mut full = INFTY;
            let mandatory = 1 << mask.trailing_zeros();
            let mut sub_mask = mask - mandatory;
            let rem_mask = mask - mandatory;
            loop {
                let call_mask = sub_mask + mandatory;
                for i in 0..n {
                    let wt = w[(vert, i)];
                    if wt != 0 && call_mask.is_set(i) {
                        let (e1, f1) = mem.call(i, call_mask.without_bit(i));
                        let (e2, f2) = mem.call(vert, mask - call_mask);
                        empty.minim(f1 + e2 + wt);
                        full.minim(e1 + c + e2 + wt);
                        full.minim(f1.min(e1) + f2 + wt);
                    }
                }

                if sub_mask == 0 {
                    break (empty, full);
                }
                sub_mask = (sub_mask - 1) & rem_mask;
            }
        }
    });
    let (e, f) = mem.call(n - 1, usize::all_bits(n - 1));
    out.print_line(e.min(f));
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
