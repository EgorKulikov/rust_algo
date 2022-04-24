//{"name":"G. Postmen","group":"Codeforces - Practice Round","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378187/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"2 3\n1 2 1 2\n1 2 2 3\n","output":"0 0 3\n"},{"input":"2 3\n0 2 1 3\n1 3 1 2 3\n","output":"2 1 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GPostmen"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let mut days = Vec::with_capacity(n);
    for _ in 0..n {
        let tp = input.read_int() == 0;
        let k = input.read_usize();
        days.push((
            tp,
            input.read_usize_vec(k).into_iter().collect::<HashSet<_>>(),
        ));
    }

    let mut good = HashSet::new();
    for (tp, s) in &days {
        if *tp {
            for &id in s {
                good.insert(id);
            }
        }
    }
    let mut bad = HashSet::new();
    for (tp, s) in days {
        if !tp {
            let diff = s.difference(&good).collect_vec();
            if diff.len() == 1 {
                bad.insert(*diff[0]);
            }
        }
    }
    out_line!(good.len(), bad.len(), m - good.len() - bad.len());
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
