//{"name":"C. Тоня и Бурёнка-179","group":"Codeforces - Codeforces Round #814 (Div. 1)","url":"https://codeforces.com/contest/1718/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n2 1\n1 2\n1 3\n4 4\n4 1 3 2\n2 6\n4 6\n1 1\n3 11\n9 3\n1 7 9 4 5 2 3 6 8\n3 1\n2 1\n9 1\n6 3\n1 1 1 1 1 1\n1 5\n4 4\n3 8\n","output":"3\n5\n14\n16\n24\n24\n24\n57\n54\n36\n36\n6\n18\n27\n28\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTonyaIBuryonka179"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use std::collections::BTreeSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let q = input.read_usize();
    let mut a = input.read_long_vec(n);

    let mut ways = Vec::new();
    let mut set = BTreeSet::new();
    let mut rem = n;
    for i in 2..=n {
        if rem % i == 0 {
            while rem % i == 0 {
                rem /= i;
            }
            let d = n / i;
            let cf = d.into_i64();
            let mut cur = vec![0; d];
            for j in 0..n {
                cur[j % d] += a[j] * cf;
            }
            for j in 0..d {
                set.insert((cur[j], ways.len(), j));
            }
            ways.push(cur);
        }
    }
    out_line!(set.iter().rev().next().unwrap().0);
    for _ in 0..q {
        let pos = input.read_usize() - 1;
        let new_val = input.read_long();
        let delta = new_val - a[pos];
        a[pos] = new_val;
        for (i, v) in ways.iter_mut().enumerate() {
            let len = v.len();
            let c_pos = pos % len;
            let cf = len.into_i64();
            let c_val = v[c_pos];
            set.remove(&(c_val, i, c_pos));
            v[c_pos] += delta * cf;
            set.insert((v[c_pos], i, c_pos));
        }
        out_line!(set.iter().rev().next().unwrap().0);
    }
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
