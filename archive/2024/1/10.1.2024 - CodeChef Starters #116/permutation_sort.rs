//{"name":"Permutation Sort","group":"CodeChef - START116A","url":"https://www.codechef.com/START116A/problems/PERMSORT","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3\n2 1 3\n5\n3 1 5 4 2\n5\n1 2 3 4 5\n","output":"1\n2 1 2\n2\n4 5 3 1 2\n2 2 3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PermutationSort"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut p = input.read_size_vec(n).dec();

    let mut ans = Vec::new();
    let mut rec = RecursiveFunction3::new(|rec, level, from, to| {
        if (from + 1) == to {
            return;
        }
        let mid = (from + to) / 2;
        let mut left = Vec::new();
        for i in from..mid {
            if p[i] >= mid {
                left.push(i);
            }
        }
        let mut right = Vec::new();
        for i in mid..to {
            if p[i] < mid {
                right.push(i);
            }
        }
        assert_eq!(left.len(), right.len());
        if !left.is_empty() {
            if ans.len() == level {
                ans.push(Vec::new());
            }
            for (a, b) in left.into_iter().zip(right) {
                ans[level].push(a + 1);
                ans[level].push(b + 1);
                p.swap(a, b);
            }
            rec.call(level + 1, from, mid);
            rec.call(level + 1, mid, to);
        } else {
            rec.call(level, from, mid);
            rec.call(level, mid, to);
        }
    });
    rec.call(0, 0, n);
    out.print_line(ans.len());
    for v in ans {
        out.print_line((v.len(), v));
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
