//{"name":"coderun_444","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"coderun_444"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let f = input.read_char_table(n, m);

    out.set_bool_output(BoolOutput::YesNo);
    for i in 0..n {
        for j in 0..m {
            if f[(i, j)] == '.' {
                continue;
            }
            for (dx, dy) in [(0, 1), (1, 0), (1, 1), (1, -1)] {
                let mut x = i as isize;
                let mut y = j as isize;
                let mut good = true;
                for _ in 0..4 {
                    x += dx;
                    y += dy;
                    if x < 0
                        || y < 0
                        || x as usize >= n
                        || y as usize >= m
                        || f[(x as usize, y as usize)] != f[(i, j)]
                    {
                        good = false;
                        break;
                    }
                }
                if good {
                    out.print_line(true);
                    return;
                }
            }
        }
    }
    out.print_line(false);
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
}
//END MAIN
