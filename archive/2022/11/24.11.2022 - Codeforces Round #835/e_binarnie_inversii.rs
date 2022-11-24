//{"name":"E. Бинарные инверсии","group":"Codeforces - Codeforces Round #835 (Div. 4)","url":"https://codeforces.com/contest/1760/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4\n1 0 1 0\n6\n0 1 0 0 1 0\n2\n0 0\n8\n1 0 1 1 0 0 0 1\n3\n1 1 1\n","output":"3\n7\n1\n13\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBinarnieInversii"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_int_vec(n);

    let mut one_to_left = Vec::with_capacity(n);
    let mut cur = 0;
    let mut inv = 0i64;
    for &i in &a {
        one_to_left.push(cur);
        if i == 1 {
            cur += 1;
        } else {
            inv += cur;
        }
    }
    let mut zero_to_right = Vec::with_capacity(n);
    let mut cur = 0;
    for &i in a.iter().rev() {
        zero_to_right.push(cur);
        if i == 0 {
            cur += 1;
        }
    }
    zero_to_right.reverse();

    let mut ans = inv;
    for i in 0..n {
        if a[i] == 0 {
            ans.maxim(inv - one_to_left[i] + zero_to_right[i]);
        } else {
            ans.maxim(inv + one_to_left[i] - zero_to_right[i]);
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
