//{"name":"D. Watch the Videos","group":"Codeforces - 2022-2023 ICPC, NERC, Южный четвертьфинал (онлайн-трансляция, правила ICPC, командное соревнование)","url":"https://codeforces.com/contest/1765/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"5 6\n1 2 3 4 5\n","output":"16\n"},{"input":"5 5\n1 2 3 4 5\n","output":"17\n"},{"input":"4 3\n1 3 2 3\n","output":"12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DWatchTheVideos"}}}

use algo_lib::collections::multi_set::MultiTreeSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_long();
    let a = input.read_long_vec(n);

    let mut v = a.into_iter().collect::<MultiTreeSet<_>>();
    let mut ans = 0;
    while !v.is_empty() {
        let x = v.pop_last().unwrap();
        ans += x;
        let mut to_remove = None;
        if let Some(&y) = v.range_rev(..=m - x).next() {
            ans += y;
            to_remove = Some(y);
        } else if !v.is_empty() {
            ans += 1;
        }
        if let Some(y) = to_remove {
            v.remove(&y);
        }
    }
    out_line!(ans + 1);
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
