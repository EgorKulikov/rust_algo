//{"name":"Distinct Count Queries","group":"HackerRank - Bihari Coding Contest","url":"https://www.hackerrank.com/contests/bihari-coding-contest/challenges/distinct-count-queries","interactive":false,"timeLimit":4000,"tests":[{"input":"7\n0 1 4 2 3 3 1\n4\n0 3\n2 1\n6 4\n5 2\n","output":"69\n58\n61\n63\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DistinctCountQueries"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_size_vec(n);

    let mut pos = DefaultHashMap::<_, BTreeSet<usize>>::new();
    for i in 0..n {
        pos[a[i]].insert(i + 1);
    }
    let mut ans = 0;
    for v in pos.values() {
        let mut last = 0;
        for &i in v.iter() {
            ans += (i - last) * (n - i + 1);
            last = i;
        }
    }

    let q = input.read_size();
    for _ in 0..q {
        let at = input.read_size() + 1;
        let val = input.read_size();
        let prev = pos[a[at - 1]].range(..at).next_back().copied().unwrap_or(0);
        let next = pos[a[at - 1]]
            .range(at + 1..)
            .next()
            .copied()
            .unwrap_or(n + 1);
        pos[a[at - 1]].remove(&at);
        ans -= (at - prev) * (next - at);
        a[at - 1] = val;
        let prev = pos[val].range(..at).next_back().copied().unwrap_or(0);
        let next = pos[val].range(at + 1..).next().copied().unwrap_or(n + 1);
        ans += (at - prev) * (next - at);
        pos[val].insert(at);
        out.print_line(ans);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    if false {
        true
    } else {
        input.skip_whitespace();
        input.peek().is_none()
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
