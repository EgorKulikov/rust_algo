//{"name":"Fallacy of Wasseypur","group":"CodeChef - CDVN2023","url":"https://www.codechef.com/CDVN2023/problems/WASSEYPUR","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 2 3 4\n5\n1 2 3 4 5\n2\n2 5\n3 1\n","output":"Ramadhir\n"},{"input":"4\n4 3 2 1\n2\n7 1\n3\n3 1\n4 7\n4 1\n","output":"Sardar\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FallacyOfWasseypur"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut a = input
        .read_unsigned_vec(n)
        .into_iter()
        .collect::<HashSet<_>>();
    let m = input.read_size();
    let mut b = input
        .read_unsigned_vec(m)
        .into_iter()
        .collect::<HashSet<_>>();
    let k = input.read_size();

    for _ in 0..k {
        let i = input.read_unsigned();
        let j = input.read_unsigned();
        if !a.contains(&i) || !b.contains(&j) {
            continue;
        }
        if (i ^ j) & 1 == 0 {
            a.remove(&i);
        } else {
            b.remove(&j);
        }
    }
    out_line!(if a.len() < b.len() {
        "Ramadhir"
    } else {
        "Sardar"
    });
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
