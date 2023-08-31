//{"name":"B. Сортировка разбиением","group":"Codeforces - Pinely Round 2 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1863/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1\n1\n2\n2 1\n6\n6 4 3 5 2 1\n3\n3 1 2\n19\n10 19 7 1 17 11 8 5 12 9 4 18 14 2 6 15 3 16 13\n","output":"0\n1\n4\n1\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSortirovkaRazbieniem"}}}

use algo_lib::collections::permutation::Permutation;
use algo_lib::collections::vec_ext::{ConsecutiveIter, IncDec};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();

    let q = Permutation::new(p).inv();
    let mut ans = 0;
    for (i, j) in q.consecutive_iter() {
        if i > j {
            ans += 1;
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
