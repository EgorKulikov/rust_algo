//{"name":"B. Поход в кинотеатр","group":"Codeforces - VK Cup 2022 - Отборочный раунд (Engine)","url":"https://codeforces.com/contest/1781/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n1 1\n7\n0 1 2 3 4 5 6\n8\n6 0 3 3 6 7 2 7\n5\n3 0 0 3 3\n","output":"2\n1\n3\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPokhodVKinoteatr"}}}

use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut a = input.read_size_vec(n);

    let mut ans = 1;
    a.sort();
    if a[0] > 0 {
        ans += 1;
    }
    for (pos, (&i, &j)) in a.consecutive_iter().enumerate() {
        if i <= pos && j > pos + 1 {
            ans += 1;
        }
    }
    out_line!(ans);
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
