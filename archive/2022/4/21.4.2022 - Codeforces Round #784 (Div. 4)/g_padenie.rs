//{"name":"G. Падение","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n6 10\n.*.*....*.\n.*.......*\n...o....o.\n.*.*....*.\n..........\n.o......o*\n2 9\n...***ooo\n.*o.*o.*o\n5 5\n*****\n*....\n*****\n....*\n*****\n","output":"..........\n...*....*.\n.*.o....o.\n.*........\n.*......**\n.o.*....o*\n\n....**ooo\n.*o**o.*o\n\n.....\n*...*\n*****\n*****\n*****\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GPadenie"}}}

use algo_lib::collections::arr2d::{Arr2dCharWrite, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let mut map = input.read_table::<char>(n, m);

    for i in 0..m {
        let mut stones = 0;
        for j in 0..=n {
            if j == n || map[(j, i)] == 'o' {
                for k in j - stones..j {
                    map[(k, i)] = '*';
                }
                stones = 0;
            } else if map[(j, i)] == '*' {
                map[(j, i)] = '.';
                stones += 1;
            }
        }
    }
    output().print_table(&map);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
