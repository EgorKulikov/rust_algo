//{"name":"B. Целостный массив","group":"Codeforces - Codeforces Round #775 (Div. 1, по задачам Открытой олимпиады школьников по программированию)","url":"https://codeforces.com/contest/1648/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 5\n1 2 5\n4 10\n1 3 3 7\n1 2\n2\n1 1\n1\n","output":"Yes\nNo\nNo\nYes\n"},{"input":"1\n1 1000000\n1000000\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTselostniiMassiv"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let c = input.read_usize();
    let a = input.read_usize_vec(n);

    let mut q = vec![0; c + 1];
    for i in a {
        q[i] += 1;
    }
    let s = q.as_slice().partial_sums();
    let have = |from: usize, to: usize| -> bool { s[from] != s[to.min(c + 1)] };
    for i in 1..=c {
        if !have(i, i + 1) {
            for j in 1..=c / i {
                if have(j, j + 1) && have(j * i, j * i + j) {
                    out_line!("No");
                    return;
                }
            }
        }
    }
    out_line!("Yes");
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
