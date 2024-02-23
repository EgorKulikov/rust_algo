//{"name":"A - Chocolate","group":"AtCoder - AtCoder Regular Contest 172","url":"https://atcoder.jp/contests/arc172/tasks/arc172_a","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4 4\n1 0 0 1\n","output":"Yes\n"},{"input":"5 7 6\n0 1 0 2 0 1\n","output":"Yes\n"},{"input":"3 2 7\n0 0 0 0 0 0 0\n","output":"No\n"},{"input":"11 11 2\n2 3\n","output":"No\n"},{"input":"777 777 6\n8 6 9 1 2 0\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AChocolate"}}}

use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let h = input.read_long();
    let w = input.read_long();
    let n = input.read_size();
    let a = input.read_size_vec(n);

    out.set_bool_output(BoolOutput::YesNo);
    let qty = a.qty_bound(30);
    let mut cur = 0;
    for i in (0..30).rev() {
        cur *= 4;
        let hh = h.is_set(i);
        let ww = w.is_set(i);
        if hh {
            cur += w / (1 << (i + 1)) * 2;
        }
        if ww {
            cur += h / (1 << (i + 1)) * 2;
        }
        if hh && ww {
            cur += 1;
        }
        if qty[i] as i64 > cur {
            out.print_line(false);
            return;
        }
        cur -= qty[i] as i64;
    }
    out.print_line(true);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
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
    output.flush();
    if false {
        true
    } else {
        input.skip_whitespace();
        input.peek().is_none()
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
