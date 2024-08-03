//{"name":"E. Игра с раскрасками","group":"Codeforces - Pinely Round 4 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1991/problem/E","interactive":true,"timeLimit":2000,"tests":[{"input":"2\n3 3\n1 2\n2 3\n3 1\n\n\n3 1\n\n2 2\n\n1 1\n4 4\n1 2\n2 3\n3 4\n4 1\n\n2 3\n\n1 2\n\n2 1\n\n3 1\n","output":"\n\n\n\n\nAlice\n3 1\n\n1 2\n\n2 1\n\n\n\n\n\n\nBob\n\n1 2\n\n2 1\n\n4 1\n\n3 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EIgraSRaskraskami"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::from_biedges(n, &edges);
    let mut color = vec![2; n];
    let mut good = true;
    let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, col: usize| {
        if color[vert] != 2 && color[vert] != col {
            good = false;
            return;
        }
        if color[vert] != 2 {
            return;
        }
        color[vert] = col;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert, 1 - col);
            if !good {
                return;
            }
        }
    });
    dfs.call(0, 0, 0);
    if good {
        let mut zeores = Vec::new();
        let mut ones = Vec::new();
        for i in 0..n {
            if color[i] == 0 {
                zeores.push(i);
            } else {
                ones.push(i);
            }
        }
        out.print_line("Bob");
        out.flush();
        for _ in 0..n {
            let a = input.read_size();
            let b = input.read_size();
            if a == 1 || b == 1 {
                if let Some(i) = zeores.pop() {
                    out.print_line((i + 1, 1));
                } else {
                    out.print_line((ones.pop().unwrap() + 1, a + b - 1));
                }
            } else {
                if let Some(i) = ones.pop() {
                    out.print_line((i + 1, 2));
                } else {
                    out.print_line((zeores.pop().unwrap() + 1, a + b - 2));
                }
            }
            out.flush();
        }
    } else {
        out.print_line("Alice");
        for _ in 0..n {
            out.print_line((1, 2));
            out.flush();
            input.read_size();
            input.read_size();
        }
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
}
//END MAIN
