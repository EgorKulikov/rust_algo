//{"name":"d1000000","group":"Google Coding Competitions - Qualification Round 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/0000000000876ff1/0000000000a46471","interactive":false,"timeLimit":5000,"tests":[{"input":"4\n4\n6 10 12 8\n6\n5 4 5 4 4 4\n10\n10 10 7 6 7 4 4 5 7 4\n1\n10\n","output":"Case #1: 4\nCase #2: 5\nCase #3: 9\nCase #4: 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1000000"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, test_case: usize) {
    let n = input.read_usize();
    let mut s = input.read_usize_vec(n);

    s.sort_unstable();
    let mut next = 1;
    for i in s {
        if i >= next {
            next += 1;
        }
    }
    out_line!(format!("Case #{}: ", test_case), next - 1);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
