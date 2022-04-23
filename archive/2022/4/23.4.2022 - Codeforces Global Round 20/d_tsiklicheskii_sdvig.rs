//{"name":"D. Циклический сдвиг","group":"Codeforces - Codeforces Global Round 20","url":"https://codeforces.com/contest/1672/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5\n1 2 3 3 2\n1 3 3 2 2\n5\n1 2 4 2 1\n4 2 2 1 1\n5\n2 4 5 5 2\n2 2 4 5 5\n3\n1 2 3\n1 2 3\n3\n1 1 2\n2 1 1\n","output":"YES\nYES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTsiklicheskiiSdvig"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);

    let mut rem: DefaultMap<i32, i32> = DefaultMap::new();
    let mut at_a = n;
    for i in (0..n).rev() {
        if i + 1 < n && b[i] == b[i + 1] {
            rem[b[i]] += 1;
            continue;
        }
        while b[i] != a[at_a - 1] {
            if rem[a[at_a - 1]] == 0 {
                out_line!(false);
                return;
            }
            rem[a[at_a - 1]] -= 1;
            at_a -= 1;
        }
        at_a -= 1;
    }
    out_line!(true);
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
