//{"name":"D. Безумные монстры","group":"Codeforces - Educational Codeforces Round 161 (Rated for Div. 2)","url":"https://codeforces.com/contest/1922/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5\n3 4 7 5 10\n4 9 1 18 1\n2\n2 1\n1 3\n4\n1 1 2 4\n3 3 4 2\n","output":"3 1 0 0 0\n0 0\n1 1 1 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBezumnieMonstri"}}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);
    let d = input.read_size_vec(n);

    let mut alive = (0..n).collect::<BTreeSet<_>>();
    let mut to_check = (0..n).collect_vec();
    let mut ans = Vec::with_capacity(n);
    for _ in 0..n {
        let mut dead = Vec::new();
        for &i in &to_check {
            let mut damage = 0;
            if let Some(&left) = alive.prev(&i) {
                damage += a[left];
            }
            if let Some(&right) = alive.next(&i) {
                damage += a[right];
            }
            if damage > d[i] {
                dead.push(i);
            }
        }
        ans.push(dead.len());
        for &i in &dead {
            alive.remove(&i);
        }
        to_check.clear();
        for i in dead {
            if let Some(&left) = alive.prev(&i) {
                to_check.push(left);
            }
            if let Some(&right) = alive.next(&i) {
                to_check.push(right);
            }
        }
        to_check.sort();
        to_check.dedup();
    }
    out.print_line(ans);
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
                solve(&mut input, &mut output, i + 1, &pre_calc);
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
