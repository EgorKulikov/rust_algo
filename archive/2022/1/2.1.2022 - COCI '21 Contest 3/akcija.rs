//{"name":"#3 - Akcija","group":"DMOJ - COCI '21 Contest 3","url":"https://dmoj.ca/problem/coci21c3p3","interactive":false,"timeLimit":5000,"tests":[{"input":"3 1\n1 1\n1 1\n1 3\n","output":"2 2\n"},{"input":"4 3\n1 1\n10 1\n2 3\n10 3\n","output":"3 13\n3 22\n2 3\n"},{"input":"2 4\n1 1\n2 2\n","output":"2 3\n1 1\n1 2\n0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Akcija"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;
use std::collections::BinaryHeap;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let k = input.read_usize();
    let goods: Vec<(i64, usize)> = input.read_vec(n);

    if n <= 20 {
        let mut res = Vec::new();
        for i in 0..1 << n {
            let mut cur = Vec::new();
            let mut weight = 0;
            for (j, (w, d)) in goods.iter().cloned().enumerate() {
                if i.is_set(j) {
                    cur.push(d);
                    weight += w;
                }
            }
            cur.sort_unstable();
            let mut good = true;
            let len = cur.len();
            for (j, k) in cur.into_iter().enumerate() {
                if j >= k {
                    good = false;
                    break;
                }
            }
            if good {
                res.push((len, weight));
            }
        }
        res.sort_by_key(|(len, w)| (n - *len, *w));
        for (len, w) in res.into_iter().take(k) {
            out_line!(len, w);
        }
        return;
    }

    let mut by_end = vec![Vec::new(); n + 1];
    for (w, d) in goods {
        by_end[d].push(w);
    }
    let mut size = 0;
    let mut cost = 0;
    let mut heap = BinaryHeap::with_capacity(n);
    for i in (1..=n).rev() {
        for w in by_end[i].drain(..) {
            heap.push(-w);
        }
        if let Some(w) = heap.pop() {
            size += 1;
            cost -= w;
        }
    }
    out_line!(size, cost);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
