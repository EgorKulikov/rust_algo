//{"name":"B. Садовник и массив","group":"Codeforces - Codeforces Round #843 (Div. 2)","url":"https://codeforces.com/contest/1775/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\n2 1 5\n2 2 4\n2 2 3\n2\n2 1 2\n1 2\n4\n3 1 2 4\n2 2 4\n4 1 2 5 6\n2 2 5\n5\n3 3 1 2\n3 2 5 3\n5 7 2 3 1 4\n5 1 2 6 3 5\n3 2 6 3\n2\n1 1\n1 2\n","output":"No\nYes\nYes\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSadovnikIMassiv"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_bool_output, BoolOutput};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        let k = input.read_size();
        let a = input.read_size_vec(k).dec_by_one();
        v.push(a);
    }

    let mut qty: DefaultHashMap<_, usize> = DefaultHashMap::new();
    for a in v.iter().flat_map(|x| x.iter()) {
        qty[*a] += 1;
    }
    for a in v {
        let mut found = false;
        for i in a {
            if qty[i] == 1 {
                found = true;
                break;
            }
        }
        if !found {
            out_line!(true);
            return;
        }
    }
    out_line!(false);
}

pub(crate) fn run(mut input: Input) -> bool {
    set_bool_output(BoolOutput::YesNo);
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
