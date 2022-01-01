//{"name":"task_e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"task_e"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::{BTreeSet, VecDeque};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();

    let mut arr = VecDeque::new();
    let mut set = BTreeSet::new();
    for i in 0..n {
        let t = input.read_usize();
        match t {
            1 => {
                let x = input.read_usize();
                arr.push_back((x, i));
                set.insert((x, i));
            }
            2 => {
                let x = input.read_usize();
                arr.push_front((x, i));
                set.insert((x, i));
            }
            3 => {
                let x = arr.pop_back().unwrap();
                set.remove(&x);
                out_line!(x.0);
            }
            4 => {
                let x = arr.pop_front().unwrap();
                set.remove(&x);
                out_line!(x.0);
            }
            5 => {
                let x = input.read_usize();
                for j in 0..arr.len() {
                    if (j ^ x) > j {
                        arr.swap(j, j ^ x);
                    }
                }
            }
            6 => {
                out_line!(set.iter().rev().next().unwrap().0);
            }
            _ => unreachable!(),
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
