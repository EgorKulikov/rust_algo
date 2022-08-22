//{"name":"Sort the Array","group":"CodeChef - LTIME111A","url":"https://www.codechef.com/LTIME111A/problems-old/SORTARRAY","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n6\n1 2 1 3 4 3\n3\n3 1 2\n","output":"2\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SortTheArray"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    let mut best = vec![None; n + 1];
    let mut last = Some(0);
    for i in 0..n {
        let mut cur = None;
        if i == 0 || a[i] >= a[i - 1] {
            if let Some(cand) = last {
                cur.minim(cand);
            }
        }
        if let Some(cand) = best[a[i]] {
            cur.minim(cand + 1);
        }
        if let Some(cand) = cur {
            best[a[i]].minim(cand);
        }
        last = cur;
    }
    out_line!(last);
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
