//{"name":"B. Фибоначчиевы строки","group":"Codeforces - Codeforces Round #814 (Div. 1)","url":"https://codeforces.com/contest/1718/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1\n1\n2\n1 1\n2\n1 2\n3\n3 1 3\n2\n7 5\n6\n26 8 3 4 13 34\n","output":"YES\nYES\nNO\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFibonachchieviStroki"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let k = input.read_usize();
    let mut c = input.read_long_vec(k);

    let sum = c.iter().sum::<i64>();
    let mut fib = Vec::new();
    let mut c_sum = 0;
    while c_sum < sum {
        let next = if fib.is_empty() {
            1
        } else {
            fib[fib.len() - 1]
                + if fib.len() == 1 {
                    0
                } else {
                    fib[fib.len() - 2]
                }
        };
        c_sum += next;
        fib.push(next);
    }
    if sum != c_sum {
        out_line!(false);
        return;
    }
    let mut rec = RecursiveFunction2::new(|f, id: usize, forbidden: usize| -> bool {
        for i in 0..k {
            if c[i] >= fib[id] && i != forbidden {
                if id == 0 {
                    return true;
                }
                c[i] -= fib[id];
                if f.call(id - 1, i) {
                    return true;
                }
                c[i] += fib[id];
            }
        }
        false
    });
    out_line!(rec.call(fib.len() - 1, k));
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
