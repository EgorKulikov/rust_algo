//{"name":"3D Printing","group":"Google Coding Competitions - Qualification Round 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/0000000000876ff1/0000000000a4672b","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n300000 200000 300000 500000\n300000 200000 500000 300000\n300000 500000 300000 200000\n1000000 1000000 0 0\n0 1000000 1000000 1000000\n999999 999999 999999 999999\n768763 148041 178147 984173\n699508 515362 534729 714381\n949704 625054 946212 951187\n","output":"Case #1: 300000 200000 300000 200000\nCase #2: IMPOSSIBLE\nCase #3: 400001 100002 100003 399994\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPrinting"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, test_case: usize) {
    let mut have = vec![1_000_000; 4];
    for _ in 0..3 {
        for j in &mut have {
            j.minim(input.read_int());
        }
    }
    let mut rem = 1_000_000;
    for j in &mut have {
        let cur = (*j).min(rem);
        rem -= cur;
        *j = cur;
    }
    if rem > 0 {
        out_line!(format!("Case #{}: IMPOSSIBLE", test_case));
    } else {
        out_line!(format!("Case #{}: ", test_case), have);
    }
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
