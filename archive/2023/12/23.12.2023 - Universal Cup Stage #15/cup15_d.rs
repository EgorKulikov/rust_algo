//{"name":"cup15_d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"cup15_d"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, usize)>(m).dec();

    let mut red = HashSet::new();
    let mut blue = HashSet::new();
    let mut next = Vec::new();
    let mut graph = Graph::new(n);
    for (u, v, c) in edges {
        if c == 0 {
            red.insert((u, v));
            red.insert((v, u));
        } else {
            blue.insert((u, v));
            blue.insert((v, u));
        }
        graph.add_edge(BiEdge::with_payload(u, v, c));
    }
    for i in 0..n {
        let mut red = Vec::new();
        let mut blue = Vec::new();
        for e in &graph[i] {
            if e.payload() == &0 {
                red.push(e.to());
            } else {
                blue.push(e.to());
            }
        }
        if red.len() == 1 {
            next.push(Some(red[0]));
        } else if blue.len() == 1 {
            next.push(Some(blue[0]));
        } else {
            next.push(None);
        }
    }
    let mut ans = HashSet::new();
    for i in 0..n {
        let mut cur = i;
        let mut to_check = vec![i];
        for sz in 1..=4 {
            to_check.sort();
            if !ans.contains(&to_check) {
                let mut in_blue = HashSet::new();
                let mut in_red = HashSet::new();
                let mut count_blue = 0;
                let mut count_red = 0;
                for &x in &to_check {
                    for &y in &to_check {
                        if x == y {
                            break;
                        }
                        if red.contains(&(x, y)) {
                            in_red.insert(x);
                            in_red.insert(y);
                            count_red += 1;
                        }
                        if blue.contains(&(x, y)) {
                            in_blue.insert(x);
                            in_blue.insert(y);
                            count_blue += 1;
                        }
                    }
                }
                if in_blue.len() == sz
                    && in_red.len() == sz
                    && count_red >= sz - 1
                    && count_blue >= sz - 1
                {
                    ans.insert(to_check.clone());
                }
            }
            if let Some(v) = next[cur] {
                if to_check.contains(&v) {
                    break;
                }
                cur = v;
                to_check.push(v);
            } else {
                break;
            }
        }
    }
    out.print_line(ans.len() + n);
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
                solve(&mut input, &mut output, i, &pre_calc);
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
