//{"name":"day14","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day14"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out_line, scan};

fn solve(input: &mut Input, _test_case: usize) {
    let mut table = Arr2d::new(200, 1000, false);

    let mut max_y = 0;
    while !input.is_exhausted() {
        let line = input.read_line();
        let tokens = line.split(" -> ").collect_vec();
        let mut bytes = tokens[0].as_bytes();
        let mut token_input = Input::new(&mut bytes);
        scan!(token_input, "@,@", x: usize, y: usize);
        let mut x = x;
        let mut y = y;
        for token in tokens.into_iter().skip(1) {
            let mut bytes = token.as_bytes();
            let mut token_input = Input::new(&mut bytes);
            scan!(token_input, "@,@", tx: usize, ty: usize);
            while (x, y) != (tx, ty) {
                max_y.maxim(y);
                table[(y, x)] = true;
                if x < tx {
                    x += 1;
                } else if x > tx {
                    x -= 1;
                } else if y < ty {
                    y += 1;
                } else if y > ty {
                    y -= 1;
                }
            }
            table[(y, x)] = true;
            max_y.maxim(y);
        }
        input.skip_whitespace();
    }

    for i in table.row_mut(max_y + 2) {
        *i = true;
    }

    for i in 0.. {
        let mut x = 500;
        let mut y = 0;

        if table[(y, x)] {
            out_line!(i);
            break;
        }

        /*let fall = */
        loop {
            // if y == 199 {
            //     break true;
            // }
            if !table[(y + 1, x)] {
                y += 1;
                continue;
            }
            if !table[(y + 1, x - 1)] {
                x -= 1;
                y += 1;
                continue;
            }
            if !table[(y + 1, x + 1)] {
                x += 1;
                y += 1;
                continue;
            }
            table[(y, x)] = true;
            break /*false*/;
        } //;
          // if fall {
          //     out_line!(i);
          //     return;
          // }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
