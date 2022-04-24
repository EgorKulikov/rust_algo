//{"name":"H. Market","group":"Codeforces - Practice Round","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378187/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 10 6 1\n5\n9 3 2 8 3\n","output":"16\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HMarket"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_long_vec(n);
    let m = input.read_usize();
    let b = input.read_long_vec(m);

    let mut all = a
        .into_iter()
        .map(|id| (id, -1))
        .chain(b.into_iter().map(|id| (id, 1)))
        .collect_vec();
    all.sort_unstable();
    let mut rem_sellers = 0;
    let mut rem_buyers = m.into_i64();
    let mut ans = 0;
    for (price, tp) in all {
        if tp == -1 {
            rem_sellers += 1;
        } else {
            ans.maxim(price * (rem_sellers.min(rem_buyers)));
            rem_buyers -= 1;
        }
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
