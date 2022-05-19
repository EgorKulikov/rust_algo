//{"name":"E. MEX против DIFF","group":"Codeforces - Codeforces Round #792 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1684/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 1\n3 0 1 2\n4 1\n0 2 4 5\n7 2\n4 13 0 0 13 1337 1000000000\n6 2\n1 2 8 0 0 0\n","output":"0\n1\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMEXProtivDIFF"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let a = input.read_usize_vec(n);

    let mut qty: DefaultMap<usize, usize> = DefaultMap::new();
    for i in a {
        qty[i] += 1;
    }
    let mut order = Vec::new();
    for (&k, &v) in qty.iter() {
        order.push((v, k));
    }
    order.sort();
    let mut diff = order.len();
    let mut at = 0;
    let mut rem = k;
    let mut ans = None;
    let mut moves = 0;
    for i in 0..n {
        while at < order.len() && rem >= order[at].0 {
            if order[at].1 < i {
                at += 1;
                continue;
            }
            rem -= order[at].0;
            at += 1;
            diff -= 1;
        }
        ans.minim(diff);
        if diff == 0 {
            break;
        }
        if (qty[i], i) >= order[at] {
            diff -= 1;
        } else {
            rem += qty[i];
        }
        if qty[i] == 0 {
            moves += 1;
        }
        if moves > k {
            break;
        }
    }
    if moves <= k {
        ans.minim(0);
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
