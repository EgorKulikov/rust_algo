//{"name":"B - Missing Number in Graph","group":"AtCoder - AtCoder Regular Contest 214","url":"https://atcoder.jp/contests/arc214/tasks/arc214_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 1\n1 2 3\n1 0\n4 4\n2 1 4\n1 3 5\n1 4 7\n2 3 1\n","output":"0\n-1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::ops::BitXor;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let abx = input.read_vec::<(usize, usize, usize)>(m).dec();

    if n % 2 == 1 {
        out.print_line(-1);
        return;
    }
    let mut graph = Graph::new(n);
    for (a, b, x) in abx {
        graph.add_edge(BiEdge::with_payload(a, b, x));
    }
    let mut xor = vec![0; n];
    let mut processed = BitSet::new(n);
    let mut rec = RecursiveFunction2::new(|rec, vert: usize, x: usize| {
        if processed[vert] {
            return;
        }
        processed.set(vert);
        xor[vert] = x;
        for e in &graph[vert] {
            rec.call(e.to(), x ^ *e.payload());
        }
    });
    rec.call(0, 0);
    let x = xor.copy_fold(0, usize::bitxor);
    let r = (0..=n).fold(0, usize::bitxor);
    out.print_line(x ^ r);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
