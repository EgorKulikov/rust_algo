//{"name":"H. Happy Bus Trip","group":"Codeforces - STAR Contest 2022","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378214/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2 4\n10 -10 2 3\n-1 -3 1 4\n6 -6 1 3\n7 4 2 4\n","output":"28\n"},{"input":"2 2 5\n0 5 2 4\n-5 -4 1 5\n","output":"-6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HHappyBusTrip"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::BTreeSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let p = input.read_usize();
    let passengers: Vec<(i64, i64, usize, usize)> = input.read_vec(n);

    let mut base_delta = vec![0; p];
    let mut enter = vec![Vec::new(); p];
    let mut left = vec![Vec::new(); p];
    for (i, (a, b, l, r)) in passengers.into_iter().enumerate() {
        base_delta[l - 1] += b;
        base_delta[r - 1] -= b;
        if a > b {
            enter[l - 1].push((a - b, i));
            left[r - 1].push((a - b, i));
        }
    }
    let mut ans = 0;
    let mut cur_base = 0;
    let mut cur_sit = 0;
    let mut stand = BTreeSet::new();
    let mut sit: BTreeSet<(i64, usize)> = BTreeSet::new();
    for i in 0..p {
        ans += cur_base + cur_sit;
        cur_base += base_delta[i];
        for &v in &enter[i] {
            stand.insert(v);
        }
        for &v in &left[i] {
            if sit.remove(&v) {
                cur_sit -= v.0;
            } else {
                stand.remove(&v);
            }
        }
        while let Some(&v) = stand.iter().last() {
            if sit.len() == m {
                let u = *sit.iter().next().unwrap();
                if u.0 >= v.0 {
                    break;
                }
                stand.insert(u);
                sit.remove(&u);
                cur_sit -= u.0;
            }
            sit.insert(v);
            stand.remove(&v);
            cur_sit += v.0;
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
