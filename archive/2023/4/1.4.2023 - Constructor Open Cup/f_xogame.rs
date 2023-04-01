//{"name":"F. XO Game","group":"Codeforces - Constructor Open Cup 2023","url":"https://constructor2023.contest.codeforces.com/group/sVRDLercWX/contest/431163/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"4\nXOXXO\nOOXOX\nOOXOXOXO\nXOXOX\n","output":"Alice\nTie\nBob\nTie\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FXOGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let mut s: Str = input.read();

    let mut can_win = false;
    let mut can_tie = false;
    let n = s.len();
    for i in 1..n {
        s.swap(i - 1, i);
        for id in (i - 1).max(2)..(i + 3).min(n) {
            if s[id - 2] == b'X' && s[id - 1] == b'X' && s[id] == b'X' {
                can_win = true;
                break;
            }
        }
        if can_win {
            break;
        }
        let mut bob_can_win = false;
        for j in 1..n {
            s.swap(j - 1, j);
            for id in (j - 1).max(2)..(j + 3).min(n) {
                if s[id - 2] == b'O' && s[id - 1] == b'O' && s[id] == b'O' {
                    bob_can_win = true;
                    break;
                }
            }
            s.swap(j - 1, j);
            if bob_can_win {
                break;
            }
        }
        if !bob_can_win {
            can_tie = true;
        }
        s.swap(i - 1, i);
    }
    if can_win {
        out_line!("Alice");
        return;
    }
    if can_tie {
        out_line!("Tie");
        return;
    }
    out_line!("Bob");
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
