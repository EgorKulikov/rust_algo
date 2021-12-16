//{"name":"C. Паприка и перестановка","group":"Codeforces - Codeforces Round #761 (Div. 2)","url":"https://codeforces.com/contest/1617/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n1 7\n3\n1 5 4\n4\n12345678 87654321 20211218 23571113\n9\n1 2 3 4 18 19 5 6 7\n","output":"1\n-1\n4\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPaprikaIPerestanovka"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a: Vec<u32> = input.read_vec(n);

    let mut rem = BitSet::new(n + 1);
    rem.fill(true);
    rem.set(0, false);
    let mut big = Vec::new();
    for i in a.iter().cloned() {
        if i <= n as u32 && rem[i as usize] {
            rem.set(i as usize, false);
        } else {
            big.push(i);
        }
    }
    big.sort_unstable();
    let ans = big.len();
    for (i, j) in rem.iter().zip(big) {
        let i = i as u32;
        if j <= 2 * i {
            out_line!(-1);
            return;
        }
    }
    out_line!(ans);
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
