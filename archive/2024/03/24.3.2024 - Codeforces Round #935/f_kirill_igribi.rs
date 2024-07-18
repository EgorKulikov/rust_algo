//{"name":"F. Кирилл и грибы","group":"Codeforces - Codeforces Round 935 (Div. 3)","url":"https://codeforces.com/contest/1945/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3\n9 8 14\n3 2 1\n5\n1 2 3 4 5\n1 2 3 4 5\n6\n1 2 3 4 5 6\n6 5 4 3 2 1\n5\n1 4 6 10 10\n2 1 4 5 3\n4\n2 2 5 5\n4 2 3 1\n5\n1 2 9 10 10\n1 4 2 3 5\n","output":"16 2\n9 3\n8 2\n20 2\n5 1\n20 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FKirillIGribi"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::cmp::Reverse;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let v = input.read_size_vec(n);
    let p = input.read_size_vec(n).dec();

    let mut ans = None;
    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| Reverse(v[i]));
    let mut forbidden = BitSet::new(n);
    let mut set = BTreeSet::new();
    for i in 0..n {
        if set.len() >= 1 {
            let j = set.len() - 1;
            forbidden.set(p[j]);
            set.remove(&(v[p[j]], p[j]));
        }
        if !forbidden[order[i]] {
            set.insert((v[order[i]], order[i]));
        }
        ans.maxim((set.first().unwrap().0 * set.len(), Reverse(set.len())));
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
