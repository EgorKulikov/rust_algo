//{"name":"Controlled Inflation","group":"Google Coding Competitions - Round 1B 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/000000000087711b/0000000000accfdb","interactive":false,"timeLimit":5000,"tests":[{"input":"2\n3 3\n30 10 40\n20 50 60\n60 60 50\n5 2\n1 1000000000\n500000000 1000000000\n1 1000000000\n500000000 1\n1 1000000000\n","output":"Case #1: 110\nCase #2: 4999999996\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ControlledInflation"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

use algo_lib::out_line;

fn solve(input: &mut Input, test_case: usize) {
    let n = input.read_usize();
    let p = input.read_usize();
    let x = input.read_table::<i64>(n, p);

    let mut ans_low = 0;
    let mut ans_high = 0;
    let mut low = 0;
    let mut high = 0;
    for i in 0..n {
        let c_low = *x.row(i).min().unwrap();
        let c_high = *x.row(i).max().unwrap();
        let c_ans_low = (ans_low + (low - c_high).abs() + c_high - c_low)
            .min(ans_high + (high - c_high).abs() + c_high - c_low);
        let c_ans_high = (ans_low + (low - c_low).abs() + c_high - c_low)
            .min(ans_high + (high - c_low).abs() + c_high - c_low);
        ans_low = c_ans_low;
        ans_high = c_ans_high;
        low = c_low;
        high = c_high;
    }
    out_line!(format!("Case #{}:", test_case), ans_low.min(ans_high));
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    true
    // input.skip_whitespace();
    // !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
