//{"name":"B. Расшифровка повторяшек","group":"Codeforces - Codeforces Round #773 (Div. 1)","url":"https://codeforces.com/contest/1641/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n5 7\n2\n5 5\n6\n1 3 1 2 2 3\n6\n3 2 1 1 2 3\n","output":"-1\n0\n1\n2\n4\n1 3\n5 3\n5 3\n10 3\n2\n8 6\n5\n0 3\n8 3\n5 3\n6 2\n7 1\n4\n2 6 6 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BRasshifrovkaPovtoryashek"}}}

use algo_lib::collections::iter_ext::{IterExt, IterPartialEqExt};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut a = input.read_unsigned_vec(n);

    let mut prefix_len = 0;
    let mut parts = Vec::new();
    let mut inserts = Vec::new();
    while !a.is_empty() {
        match a.iter().skip(1).find(&a[0]) {
            None => {
                out_line!(-1);
                return;
            }
            Some(mut pos) => {
                pos += 1;
                for (i, &v) in a[1..pos].iter().enumerate() {
                    inserts.push((prefix_len + 1 + pos + i, v));
                }
                parts.push(2 * pos);
                prefix_len += 2 * pos;
                a = a[1..pos]
                    .iter()
                    .rev()
                    .chain(a[(pos + 1)..].iter())
                    .cloned()
                    .collect_vec();
            }
        }
    }
    out_line!(inserts.len());
    output().print_per_line(&inserts);
    out_line!(parts.len());
    out_line!(parts);
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
