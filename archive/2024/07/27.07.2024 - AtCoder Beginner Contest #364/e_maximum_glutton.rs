//{"name":"E - Maximum Glutton","group":"AtCoder - Japan Registry Services (JPRS) Programming Contest 2024#2 (AtCoder Beginner Contest 364)","url":"https://atcoder.jp/contests/abc364/tasks/abc364_e","interactive":false,"timeLimit":3000,"tests":[{"input":"4 8 4\n1 5\n3 2\n4 1\n5 3\n","output":"3\n"},{"input":"2 1 1\n3 2\n3 2\n","output":"1\n"},{"input":"2 100 100\n3 2\n3 2\n","output":"2\n"},{"input":"6 364 463\n230 381\n154 200\n328 407\n339 94\n193 10\n115 309\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMaximumGlutton"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let x = input.read_size();
    let y = input.read_size();
    let dishes = input.read_size_pair_vec(n);

    let mut min_salty = Arr2d::new(n + 1, x + 1, None);
    min_salty[(0, 0)] = Some(0);
    for (a, b) in dishes {
        for i in (0..n).rev() {
            for j in a..=x {
                if let Some(w) = min_salty[(i, j - a)] {
                    min_salty[(i + 1, j)].minim(w + b);
                }
            }
        }
    }
    for i in (0..n).rev() {
        for j in 0..=x {
            if let Some(w) = min_salty[(i, j)] {
                if w <= y {
                    out.print_line(i + 1);
                    return;
                }
            }
        }
    }
    unreachable!();
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
}
//END MAIN
