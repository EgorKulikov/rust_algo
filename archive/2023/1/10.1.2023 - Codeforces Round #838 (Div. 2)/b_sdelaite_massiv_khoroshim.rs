//{"name":"B. Сделайте массив хорошим","group":"Codeforces - Codeforces Round #838 (Div. 2)","url":"https://codeforces.com/contest/1762/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4\n2 3 5 5\n2\n4 8\n5\n3 4 343 5 6\n3\n31 5 17\n","output":"4\n1 2\n1 1\n2 2\n3 0\n0\n5\n1 3\n1 4\n2 1\n5 4\n3 7\n3\n1 29\n2 5\n3 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSdelaiteMassivKhoroshim"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let min = *a.iter().min().unwrap();
    out_line!(n);
    for (i, a) in a.into_iter().enumerate() {
        let mut cur = min;
        while cur < a {
            cur *= 2;
        }
        out_line!(i + 1, cur - a);
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
