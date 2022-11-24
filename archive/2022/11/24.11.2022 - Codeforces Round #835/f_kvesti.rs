//{"name":"F. Квесты","group":"Codeforces - Codeforces Round #835 (Div. 4)","url":"https://codeforces.com/contest/1760/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n2 5 4\n1 2\n2 20 10\n100 10\n3 100 3\n7 2 6\n4 20 3\n4 5 6 7\n4 100000000000 2022\n8217734 927368 26389746 627896974\n2 20 4\n5 1\n","output":"2\nInfinity\nImpossible\n1\n12\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FKvesti"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let c = input.read_long();
    let d = input.read_usize();
    let mut a = input.read_long_vec(n);

    let mut ans = None;
    a.sort();
    a.reverse();
    if d > n {
        a.extend(vec![0; d - n]);
    }
    let s = a.as_slice().partial_sums();
    for i in 1..=d {
        let cur = s[i] * (d / i).into_i64() + s[d % i];
        if cur >= c {
            ans = Some(i);
        }
    }
    match ans {
        None => {
            out_line!("Impossible");
        }
        Some(ans) => {
            if ans == d {
                out_line!("Infinity");
            } else {
                out_line!(ans - 1);
            }
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
