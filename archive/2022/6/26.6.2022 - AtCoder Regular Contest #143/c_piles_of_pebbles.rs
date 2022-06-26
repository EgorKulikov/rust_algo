//{"name":"C - Piles of Pebbles","group":"AtCoder - AtCoder Regular Contest 143","url":"https://atcoder.jp/contests/arc143/tasks/arc143_c","interactive":false,"timeLimit":2000,"tests":[{"input":"2 1 1\n3 3\n","output":"First\n"},{"input":"2 1 2\n3 3\n","output":"Second\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPilesOfPebbles"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let x = input.read_int();
    let y = input.read_int();
    let a = input.read_int_vec(n);

    let mut force_second = true;
    for &i in &a {
        if i % (x + y) >= x {
            force_second = false;
        }
    }
    if force_second {
        out_line!("Second");
        return;
    }
    let mut force_first = true;
    let mut has_move = false;
    for &i in &a {
        if i >= x && (i - x) % (x + y) < y {
            has_move = true;
        } else if i % (x + y) >= y {
            force_first = false;
        }
    }
    if has_move && force_first || x < y {
        out_line!("First");
        return;
    }
    out_line!("Second");
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
