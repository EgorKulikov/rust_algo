//{"name":"K. Traversal of a Triangular Grid","group":"Universal Cup - GP of St. Petersburg","url":"https://contest.ucup.ac/contest/3384/problem/17171","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n","output":"404240022\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::graph::edges::bi_edge::BiEdgeWithId;
use algo_lib::graph::euler_path::EulerPath;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;
use algo_lib::when;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut graph = Graph::new((n + 1) * (n + 2) / 2);
    let mut next = 0;
    let id = Arr2d::with_gen(n + 1, n + 1, |i, j| {
        if j <= i {
            let res = next;
            next += 1;
            res
        } else {
            0
        }
    });
    let mut by_id = vec![(0, 0); (n + 1) * (n + 2) / 2];
    for i in 0..=n {
        for j in 0..=i {
            by_id[id[(i, j)]] = (i, j);
        }
    }
    for i in 0..=n {
        for j in 0..=i {
            if i < n {
                graph.add_edge(BiEdgeWithId::new(id[(i, j)], id[(i + 1, j)]));
                graph.add_edge(BiEdgeWithId::new(id[(i, j)], id[(i + 1, j + 1)]));
            }
            if j < i {
                graph.add_edge(BiEdgeWithId::new(id[(i, j)], id[(i, j + 1)]));
            }
        }
    }
    let mut c = graph.euler_path().unwrap();
    let pos = c.copy_find(0).unwrap();
    c.rotate_left(pos + 1);
    c.pop();
    let mut ans = Str::new();
    let mut last = 0;
    for v in c {
        let (r0, c0) = by_id[last];
        let (r1, c1) = by_id[v];
        let next = when! {
            r0 == r1 && c1 == c0 + 1 => b'0',
            r1 + 1 == r0 && c0 == c1 => b'1',
            r1 + 1 == r0 && c1 + 1 == c0 => b'2',
            r0 == r1 && c1 + 1 == c0 => b'3',
            r1 == r0 + 1 && c0 == c1 => b'4',
            r1 == r0 + 1 && c0 + 1 == c1 => b'5',
        };
        ans.push(next);
        last = v;
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
