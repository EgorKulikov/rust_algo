//{"name":"B. Игра на отрезках","group":"Codeforces - Codeforces Round #763 (Div. 2)","url":"https://codeforces.com/contest/1623/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1\n1 1\n3\n1 3\n2 3\n2 2\n6\n1 1\n3 5\n4 4\n3 6\n4 5\n1 6\n5\n1 5\n1 2\n4 5\n2 2\n4 4\n","output":"1 1 1\n\n1 3 1\n2 2 2\n2 3 3\n\n1 1 1\n3 5 3\n4 4 4\n3 6 6\n4 5 5\n1 6 2\n\n1 5 3\n1 2 1\n4 5 5\n2 2 2\n4 4 4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BIgraNaOtrezkakh"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use std::collections::BTreeSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_int();
    let mut seg: Vec<(i32, i32)> = input.read_vec(n.into_usize());

    seg.sort_by_key(|(a, b)| a - b);
    let mut set = [(1, -n)].into_iter().collect::<BTreeSet<_>>();
    let mut ans = Vec::with_capacity(n.into_usize());
    for (l, r) in seg {
        if l == r {
            ans.push((l, r, l));
        }
        if l == 1 && r == n {
            continue;
        }
        // eprintln!("{:#?}", set);
        let (a, b) = set
            .range(..=(l, -r))
            .into_iter()
            .rev()
            .next()
            .cloned()
            .unwrap();
        if a != l {
            assert_eq!(-b, r);
            ans.push((a, -b, l - 1));
            set.remove(&(a, b));
            if l - 1 != a {
                set.insert((a, -(l - 2)));
            }
            set.insert((l, -r));
        } else if b != -r {
            assert_eq!(a, l);
            ans.push((a, -b, r + 1));
            if r + 1 != -b {
                set.insert((r + 2, b));
            }
            set.insert((l, -r));
        }
    }
    assert_eq!(ans.len(), n.into_usize());
    output().print_per_line(&ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
