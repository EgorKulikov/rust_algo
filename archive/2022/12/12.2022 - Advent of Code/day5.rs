//{"name":"day5","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day5"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let mut stacks = vec![VecDeque::new(); 10];
    loop {
        let s = input.read_line();
        if !s.contains('[') {
            break;
        }
        for (i, c) in s.chars().into_iter().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].push_front(c);
            }
        }
    }

    while !input.is_exhausted() {
        input.read_string();
        let num = input.read_size();
        input.read_string();
        let from = input.read_size() - 1;
        input.read_string();
        let to = input.read_size() - 1;
        input.skip_whitespace();

        // for _ in 0..num {
        //     let element = stacks[from].pop_back().unwrap();
        //     stacks[to].push_back(element);
        // }
        let mut elements = Vec::with_capacity(num);
        for _ in 0..num {
            elements.push(stacks[from].pop_back().unwrap());
        }
        for element in elements.into_iter().rev() {
            stacks[to].push_back(element);
        }
    }
    for s in stacks {
        if !s.is_empty() {
            out!(s.back());
        }
    }
    out_line!();
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
