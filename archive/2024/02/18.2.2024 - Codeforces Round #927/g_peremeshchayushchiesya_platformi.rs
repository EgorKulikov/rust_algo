//{"name":"G. Перемещающиеся платформы","group":"Codeforces - Codeforces Round 927 (Div. 3)","url":"https://codeforces.com/contest/1932/problem/G","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n3 3 10\n1 9 4\n2 3 0\n1 2\n3 2\n1 3\n2 1 10\n1 2\n4 6\n1 2\n8 7 25\n22 14 5 3 10 14 11 1\n9 5 4 10 7 16 18 18\n2 8\n6 3\n3 5\n7 5\n2 6\n1 4\n4 7\n","output":"6\n-1\n52\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GPeremeshchayushchiesyaPlatformi"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::indexed_heap::IndexedHeap;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::gcd::{extended_gcd, gcd};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let h = input.read_long();
    let l = input.read_long_vec(n);
    let s = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(m).dec();

    let mut graph = Graph::new(n);
    for (a, b) in edges {
        let delta = (l[b] - l[a] + h) % h;
        let step = (s[a] - s[b] + h) % h;
        let g = gcd(h, step);
        if delta % g != 0 {
            continue;
        }
        let m = h / g;
        let (_, mut rev, _) = extended_gcd((step / g) as i32, m as i32);
        if rev < 0 {
            rev += m;
        }
        let steps = delta / g * rev % h;
        let period = m;
        graph.add_edge(BiEdge::with_payload(a, b, ((steps + 1) % period, period)));
    }
    let mut heap = IndexedHeap::new(n);
    heap.add_or_relax(0, 0);
    let mut done = BitSet::new(n);
    while let Some((vert, w)) = heap.pop() {
        done.set(vert);
        if vert == n - 1 {
            out.print_line(w);
            return;
        }
        for e in &graph[vert] {
            let to = e.to();
            if done[to] {
                continue;
            }
            let &(delta, period) = e.payload();
            let mut time = (w / period) * period + delta;
            if time <= w {
                time += period;
            }
            heap.add_or_relax(to, time);
        }
    }
    out.print_line(-1);
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
