//{"name":"h","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"h"}}}

use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec_by_one();

    let mut deg = vec![0usize; n];
    for &(u, v) in &edges {
        deg[u] += 1;
        deg[v] += 1;
    }
    if deg.iter().count_eq(&&(n - 1)) == n {
        out_line!(true);
        return;
    }
    if deg.iter().count_eq(&&(n - 2)) == n - 1 && deg.iter().count_eq(&&0) == 1 {
        out_line!(true);
        return;
    }
    if deg.iter().count_eq(&&1) == n - 1 && deg.iter().count_eq(&&(n - 1)) == 1 {
        out_line!(true);
        return;
    }
    if n == 4 && deg.iter().count_eq(&&2) == 4 {
        out_line!(true);
        return;
    }
    if n == 4 && deg.iter().count_eq(&&1) == 4 {
        out_line!(true);
        return;
    }
    out_line!(false);
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
