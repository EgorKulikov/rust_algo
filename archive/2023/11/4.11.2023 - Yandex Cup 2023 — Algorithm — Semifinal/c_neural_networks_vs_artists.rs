//{"name":"C. Neural networks vs. artists","group":"Yandex - Yandex Cup 2023 — Algorithm — Semifinal","url":"https://contest.yandex.com/contest/54740/problems/C/","interactive":true,"timeLimit":1000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNeuralNetworksVsArtists"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let mut compare = |i: usize, j: usize| {
        if i >= n {
            return false;
        }
        if j >= n {
            return true;
        }
        out.print_line(('?', i + 1, j + 1));
        out.flush();
        input.read_char() == '<'
    };

    let mut left = 0;
    let mut right = k - 1;
    while left < right {
        let mid = (left + right) / 2;
        if compare(mid, k - mid - 2) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    if compare(left, k - 1 - left) {
        out.print_line(('!', 1, left + 1));
    } else {
        out.print_line(('!', 2, k - left));
    }
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
    // input.skip_whitespace();
    // input.peek().is_none()
    true
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
