//{"name":"D - All Assign Point Add","group":"AtCoder - AtCoder Beginner Contest 278","url":"https://atcoder.jp/contests/abc278/tasks/abc278_d","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 1 4 1 5\n6\n3 2\n2 3 4\n3 3\n1 1\n2 3 4\n3 3\n","output":"1\n8\n5\n"},{"input":"1\n1000000000\n8\n2 1 1000000000\n2 1 1000000000\n2 1 1000000000\n2 1 1000000000\n2 1 1000000000\n2 1 1000000000\n2 1 1000000000\n3 1\n","output":"8000000000\n"},{"input":"10\n1 8 4 15 7 5 7 5 8 0\n20\n2 7 0\n3 7\n3 8\n1 7\n3 3\n2 4 4\n2 4 9\n2 10 5\n1 10\n2 4 2\n1 10\n2 3 1\n2 8 11\n2 3 14\n2 1 9\n3 8\n3 8\n3 1\n2 6 5\n3 7\n","output":"7\n5\n7\n21\n21\n19\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DAllAssignPointAdd"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let mut a = input
        .read_long_vec(n)
        .into_iter()
        .enumerate()
        .collect::<DefaultHashMap<_, _>>();
    let q = input.read_usize();

    let mut base = 0;
    for _ in 0..q {
        let t = input.read_usize();
        match t {
            1 => {
                a.clear();
                base = input.read_long();
            }
            2 => {
                let ind = input.read_usize() - 1;
                let val = input.read_long();
                a[ind] += val;
            }
            3 => {
                let ind = input.read_usize() - 1;
                out_line!(a[ind] + base);
            }
            _ => unreachable!(),
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
