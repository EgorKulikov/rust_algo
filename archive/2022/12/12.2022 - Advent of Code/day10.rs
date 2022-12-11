//{"name":"day10","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day10"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
// use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let mut step = 0usize;
    let mut x = 1i64;
    // let mut ans = 0;
    // let mut next = 20;
    let mut ans = Arr2d::new(6, 40, '.');

    // while step < 220 {
    while step < 240 {
        let mut write = |step: usize| {
            if step >= 240 {
                return;
            }
            if ((step % 40).into_i64() - x).abs() <= 1 {
                ans[(step / 40, step % 40)] = '#';
            }
        };
        let tp = input.read_string();
        // step += match tp.as_str() {
        //     "addx" => 2,
        //     "noop" => 1,
        //     _ => unreachable!(),
        // };
        step += match tp.as_str() {
            "addx" => {
                write(step);
                write(step + 1);
                2
            }
            "noop" => {
                write(step);
                1
            }
            _ => unreachable!(),
        };
        // if step >= next {
        //     ans += next.into_i64() * x;
        //     next += 40;
        // }
        match tp.as_str() {
            "addx" => {
                x += input.read_long();
            }
            "noop" => {}
            _ => unreachable!(),
        }
    }
    // out_line!(ans);
    output().print_table(&ans);
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
