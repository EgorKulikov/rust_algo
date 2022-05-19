//{"name":"D. Ловушки","group":"Codeforces - Codeforces Round #792 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1684/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n4 4\n8 7 1 4\n4 1\n5 10 11 5\n7 5\n8 2 5 15 11 2 8\n6 3\n1 2 3 4 5 6\n1 1\n7\n","output":"0\n21\n9\n6\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DLovushki"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let a = input.read_long_vec(n);

    let mut sum = 0;
    let mut delta = Vec::with_capacity(n);
    for (i, a) in a.into_iter().enumerate() {
        sum += a;
        delta.push((n - i - 1).into_i64() - a);
    }
    delta.sort();
    let ans = sum + delta.into_iter().take(k).sum::<i64>() - (k * (k - 1) / 2).into_i64();
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
