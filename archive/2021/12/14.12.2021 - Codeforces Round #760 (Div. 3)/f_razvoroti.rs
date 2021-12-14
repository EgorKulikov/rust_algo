//{"name":"F. Развороты","group":"Codeforces - Codeforces Round #760 (Div. 3)","url":"https://codeforces.com/contest/1618/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n","output":"YES\n"},{"input":"7 4\n","output":"NO\n"},{"input":"2 8\n","output":"NO\n"},{"input":"34 69\n","output":"YES\n"},{"input":"8935891487501725 71487131900013807\n","output":"YES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FRazvoroti"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let x: u64 = input.read();
    let y: u64 = input.read();

    let mut x_str = format!("{:b}", x);
    let y_str = format!("{:b}", y);

    if y % 2 == 0 && x != y {
        out_line!("NO");
        return;
    }

    if y_str.contains(&x_str)
        && y.count_ones() - x.count_ones() == (y_str.len() - x_str.len()) as u32
    {
        out_line!("YES");
        return;
    }
    if y_str.contains(&x_str.chars().rev().collect::<String>())
        && y.count_ones() - x.count_ones() == (y_str.len() - x_str.len()) as u32
    {
        out_line!("YES");
        return;
    }
    while x_str.ends_with("0") {
        x_str.pop();
    }
    if y_str.contains(&x_str)
        && y.count_ones() - x.count_ones() == (y_str.len() - x_str.len()) as u32
    {
        out_line!("YES");
        return;
    }
    if y_str.contains(&x_str.chars().rev().collect::<String>())
        && y.count_ones() - x.count_ones() == (y_str.len() - x_str.len()) as u32
    {
        out_line!("YES");
        return;
    }
    out_line!("NO");
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
