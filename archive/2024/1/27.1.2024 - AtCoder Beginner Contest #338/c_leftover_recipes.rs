//{"name":"C - Leftover Recipes","group":"AtCoder - AtCoder Beginner Contest 338","url":"https://atcoder.jp/contests/abc338/tasks/abc338_c","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n800 300\n100 100\n200 10\n","output":"5\n"},{"input":"2\n800 300\n100 0\n0 10\n","output":"38\n"},{"input":"2\n800 300\n801 300\n800 301\n","output":"0\n"},{"input":"10\n1000000 1000000 1000000 1000000 1000000 1000000 1000000 1000000 1000000 1000000\n0 1 2 3 4 5 6 7 8 9\n9 8 7 6 5 4 3 2 1 0\n","output":"222222\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CLeftoverRecipes"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_int_vec(n);
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);

    let mut ans = None;
    for i in 0.. {
        let mut ok = true;
        let mut cur = None;
        for j in 0..n {
            if q[j] < a[j] * i {
                ok = false;
                break;
            }
            if b[j] != 0 {
                cur.minim((q[j] - a[j] * i) / b[j]);
            }
        }
        if !ok {
            break;
        }
        ans.maxim(i + cur.unwrap());
    }
    out.print_line(ans);
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
                solve(&mut input, &mut output, i, &pre_calc);
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
