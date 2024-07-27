//{"name":"F - Range Connect MST","group":"AtCoder - Japan Registry Services (JPRS) Programming Contest 2024#2 (AtCoder Beginner Contest 364)","url":"https://atcoder.jp/contests/abc364/tasks/abc364_f","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3\n1 2 2\n1 3 4\n2 4 5\n","output":"22\n"},{"input":"6 2\n1 2 10\n4 6 10\n","output":"-1\n"},{"input":"200000 4\n1 200000 1000000000\n1 200000 998244353\n1 200000 999999999\n1 200000 999999999\n","output":"199651870599998\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FRangeConnectMST"}}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut edges = input.read_vec::<(usize, usize, usize)>(q).dec();

    edges.sort_by_key(|x| x.2);
    let mut segments = BTreeSet::new();
    let mut ans = 0;
    for (l, r, c) in edges {
        let mut cur = l;
        let mut from = l;
        if let Some(&(al, ar)) = segments.prev(&(l, l)) {
            if ar >= l {
                ans += c;
                from = al;
                cur = ar + 1;
                segments.remove(&(al, ar));
            }
        }
        while let Some(&(al, ar)) = segments.ceil(&(cur, cur)) {
            if al <= r {
                ans += c * (al - cur + 1);
                cur = ar + 1;
                segments.remove(&(al, ar));
            } else {
                break;
            }
        }
        if cur <= r {
            ans += c * (r - cur + 1);
            cur = r + 1;
        }
        segments.insert((from, cur - 1));
    }
    if segments.len() != 1 || segments.into_iter().next().unwrap() != (0, n - 1) {
        out.print_line(-1);
    } else {
        out.print_line(ans);
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
    let test_type = TestType::Single;
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
}
//END MAIN
