//{"name":"D. Двухцветные доминошки","group":"Codeforces - Pinely Round 2 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1863/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4 6\n..LR..\nULRU..\nDLRDUU\n..LRDD\n5 4\n.LR.\n.UU.\nUDDU\nD..D\nLR..\n2 2\n..\n..\n","output":"..WB..\nWWBB..\nBBWWWB\n..BWBW\n-1\n..\n..\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDvukhtsvetnieDominoshki"}}}

use algo_lib::collections::arr2d::{Arr2dCharWrite, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut t = input.read_char_table(n, m);

    for i in 0..n {
        let mut next_up = false;
        let mut next_down = true;
        for j in 0..m {
            if t[(i, j)] == 'U' {
                if next_up {
                    t[(i, j)] = 'W';
                } else {
                    t[(i, j)] = 'B';
                }
                next_up = !next_up;
            }
            if t[(i, j)] == 'D' {
                if next_down {
                    t[(i, j)] = 'W';
                } else {
                    t[(i, j)] = 'B';
                }
                next_down = !next_down;
            }
        }
        if next_up || !next_down {
            out_line!(-1);
            return;
        }
    }
    for i in 0..m {
        let mut next_left = false;
        let mut next_right = true;
        for j in 0..n {
            if t[(j, i)] == 'L' {
                if next_left {
                    t[(j, i)] = 'W';
                } else {
                    t[(j, i)] = 'B';
                }
                next_left = !next_left;
            }
            if t[(j, i)] == 'R' {
                if next_right {
                    t[(j, i)] = 'W';
                } else {
                    t[(j, i)] = 'B';
                }
                next_right = !next_right;
            }
        }
        if next_left || !next_right {
            out_line!(-1);
            return;
        }
    }
    output().print_table(&t);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
