//{"name":"Minimal Inversions","group":"CodeChef - START74A","url":"https://www.codechef.com/START74A/problems/MININV","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n5\n4 2 3 1 5\n6\n1 2 3 4 5 6\n4\n2 1 1 1\n","output":"2\n0\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MinimalInversions"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut a = input.read_size_vec(n).dec_by_one();

    a.reverse();
    let mut qty = vec![0usize; n + 1];
    for &i in &a {
        qty[i] += 1;
    }
    let mut ans = 0;
    let mut cur = 0;
    let mut q_left = vec![0usize; n + 1];
    for i in a {
        qty[i] -= 1;
        q_left[i + 1] += 1;
        cur += qty[i + 1];
        cur -= q_left[i];
        ans.maxim(cur);
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
