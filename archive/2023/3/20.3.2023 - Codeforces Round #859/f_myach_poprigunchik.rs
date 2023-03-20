//{"name":"F. Мяч-попрыгунчик","group":"Codeforces - Codeforces Round 859 (Div. 4)","url":"https://codeforces.com/contest/1807/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n5 7 1 7 2 4 DL\n5 7 1 7 3 2 DL\n3 3 1 3 2 2 UR\n2 4 2 1 2 2 DR\n4 3 1 1 1 3 UL\n6 4 1 2 3 4 DR\n","output":"3\n-1\n1\n-1\n4\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMyachPoprigunchik"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_int();
    let m = input.read_int();
    let mut r = input.read_int() - 1;
    let mut c = input.read_int() - 1;
    let tr = input.read_int() - 1;
    let tc = input.read_int() - 1;
    let mut dr: i32 = if input.read_char() == 'D' { 1 } else { -1 };
    let mut dc: i32 = if input.read_char() == 'R' { 1 } else { -1 };

    let mut set = HashSet::new();
    let mut ans = 0;
    loop {
        if r == tr && c == tc {
            out_line!(ans);
            return;
        }
        if set.contains(&(r, c, dr, dc)) {
            out_line!("-1");
            return;
        }
        set.insert((r, c, dr, dc));
        let mut change = false;
        r += dr;
        if r == n || r == -1 {
            dr *= -1;
            r += dr * 2;
            change = true;
        }
        c += dc;
        if c == m || c == -1 {
            dc *= -1;
            c += dc * 2;
            change = true;
        }
        if change {
            ans += 1;
        }
    }
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
