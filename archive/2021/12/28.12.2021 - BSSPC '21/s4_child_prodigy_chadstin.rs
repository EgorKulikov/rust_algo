//{"name":"S4 - Child Prodigy Chadstin","group":"DMOJ - BSSPC '21","url":"https://dmoj.ca/problem/bsspc21s4","interactive":false,"timeLimit":5000,"tests":[{"input":"4 4 2\n1 1 2 2\n3 3 4 4\n","output":"1\n"},{"input":"6 6 4\n1 2 2 6\n2 1 6 2\n1 1 1 1\n5 5 6 6\n","output":"1\n"},{"input":"2 4 1\n1 1 2 2\n","output":"1\n"},{"input":"2 2 3\n1 1 1 1\n1 2 1 2\n2 2 2 2\n","output":"0\n"},{"input":"9 9 4\n4 1 6 3\n1 4 3 6\n4 7 6 9\n7 4 9 6\n","output":"0\n"},{"input":"4 4 2\n1 1 3 4\n4 1 4 4\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"S4ChildProdigyChadstin"}}}

use algo_lib::collections::vec_ext::{Detuple, IncDec};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable6, RecursiveFunction6};
use algo_lib::numbers::gcd::gcd;
use algo_lib::{compress, out_line, zip};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let k = input.read_usize();
    let rect: Vec<(usize, usize, usize, usize)> = input.read_vec(k);

    let (t, l, b, r) = rect.detuple();
    let t = t.dec_by_one();
    let l = l.dec_by_one();
    let (y, (t, b)) = compress!(t, b);
    let (x, (l, r)) = compress!(l, r);
    let mut ans = gcd(n, m);
    for (y, x, t, l, b, r) in [(&y, &x, &t, &l, &b, &r), (&x, &y, &l, &t, &r, &b)] {
        #[derive(Clone)]
        struct Node {
            has_zero: bool,
            delta: usize,
        }

        impl Default for Node {
            fn default() -> Self {
                Self {
                    has_zero: true,
                    delta: 0,
                }
            }
        }
        let mut nodes = vec![Node::default(); 4 * x.len()];
        #[derive(Ord, PartialOrd, Eq, PartialEq)]
        struct Event {
            at: usize,
            remove: bool,
            l: usize,
            r: usize,
        }
        let mut events = Vec::with_capacity(2 * k);
        for (t, b, l, r) in zip!(t.iter(), b.iter(), l.iter(), r.iter()) {
            let t = *t;
            let b = *b;
            let l = *l;
            let r = *r;
            events.push(Event {
                at: t,
                remove: false,
                l,
                r,
            });
            events.push(Event {
                at: b,
                remove: true,
                l,
                r,
            });
        }
        events.sort();
        let mut query = RecursiveFunction6::new(
            |f,
             root: usize,
             left: usize,
             right: usize,
             from: usize,
             to: usize,
             to_remove: bool|
             -> bool {
                if to <= left || right <= from {
                    false
                } else if from <= left && right <= to {
                    if to_remove {
                        nodes[root].delta -= 1;
                        nodes[root].delta == 0 && nodes[root].has_zero
                    } else {
                        nodes[root].delta += 1;
                        nodes[root].delta == 1 && nodes[root].has_zero
                    }
                } else {
                    let mid = (left + right) >> 1;
                    let l_res = f.call(2 * root + 1, left, mid, from, to, to_remove);
                    let r_res = f.call(2 * root + 2, mid, right, from, to, to_remove);
                    nodes[root].has_zero = nodes[2 * root + 1].has_zero
                        && nodes[2 * root + 1].delta == 0
                        || nodes[2 * root + 2].has_zero && nodes[2 * root + 2].delta == 0;
                    nodes[root].delta == 0 && (l_res || r_res)
                }
            },
        );
        for event in events {
            if query.call(0, 0, x.len(), event.l, event.r, event.remove) {
                ans = gcd(ans, y[event.at]);
            }
        }
    }
    out_line!(ans.trailing_zeros());
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
