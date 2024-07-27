//{"name":"B - Grid Walk","group":"AtCoder - Japan Registry Services (JPRS) Programming Contest 2024#2 (AtCoder Beginner Contest 364)","url":"https://atcoder.jp/contests/abc364/tasks/abc364_b","interactive":false,"timeLimit":2000,"tests":[{"input":"2 3\n2 1\n.#.\n...\nULDRU\n","output":"2 2\n"},{"input":"4 4\n4 2\n....\n.#..\n...#\n....\nDUUUURULRD\n","output":"2 4\n"},{"input":"6 6\n1 1\n.#####\n######\n######\n######\n######\n######\nRURLDLULLRULRDL\n","output":"1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BGridWalk"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let h = input.read_size();
    let w = input.read_size();
    let mut row = input.read_size() - 1;
    let mut col = input.read_size() - 1;
    let c = input.read_char_table(h, w);
    let x = input.read_str();

    for d in x {
        if d == b'L' {
            if col > 0 && c[(row, col - 1)] == '.' {
                col -= 1;
            }
        } else if d == b'R' {
            if col + 1 < w && c[(row, col + 1)] == '.' {
                col += 1;
            }
        } else if d == b'U' {
            if row > 0 && c[(row - 1, col)] == '.' {
                row -= 1;
            }
        } else if d == b'D' {
            if row + 1 < h && c[(row + 1, col)] == '.' {
                row += 1;
            }
        }
    }
    out.print_line((row + 1, col + 1));
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
