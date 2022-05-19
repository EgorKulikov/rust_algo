//{"name":"Spiraling Into Control","group":"Google Coding Competitions - Round 2 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/00000000008778ec/0000000000b15a74","interactive":false,"timeLimit":5000,"tests":[{"input":"4\n5 4\n5 3\n5 12\n3 1\n","output":"Case #1: 2\n2 17\n18 25\nCase #2: IMPOSSIBLE\nCase #3: 2\n11 22\n22 25\nCase #4: IMPOSSIBLE\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SpiralingIntoControl"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;

use algo_lib::out_line;

fn solve(input: &mut Input, test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();

    let mut total = n * n - 1 - k;
    let mut shortcuts = Vec::new();
    for initial in (1..9).step_by(2) {
        let mut step = initial;
        let mut cur = n * n;
        for _ in 0..n / 2 {
            if step != 1 {
                shortcuts.push((step - 1, cur - step, cur));
            }
            cur -= step;
            step += 8;
        }
    }
    shortcuts.sort();
    shortcuts.reverse();
    let mut i = 0;
    let mut ans = Vec::new();
    while i < shortcuts.len() {
        let (by, from, to) = shortcuts[i];
        if total >= by {
            total -= by;
            ans.push((from, to));
            i += 4;
        } else {
            i += 1;
        }
    }
    if total == 0 {
        out_line!(format!("Case #{}:", test_case), ans.len());
        output().print_per_line(&ans);
    } else {
        out_line!(format!("Case #{}: IMPOSSIBLE", test_case));
    }
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
