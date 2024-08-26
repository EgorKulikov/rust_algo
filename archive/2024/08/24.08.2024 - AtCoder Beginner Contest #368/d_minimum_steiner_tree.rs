//{"name":"D - Minimum Steiner Tree","group":"AtCoder - Hitachi Vantara Programming Contest 2024（AtCoder Beginner Contest 368）","url":"https://atcoder.jp/contests/abc368/tasks/abc368_d","interactive":false,"timeLimit":2000,"tests":[{"input":"7 3\n1 2\n1 3\n2 4\n2 5\n3 6\n3 7\n1 3 5\n","output":"4\n"},{"input":"4 4\n3 1\n1 4\n2 1\n1 2 3 4\n","output":"4\n"},{"input":"5 1\n1 4\n2 3\n5 2\n1 2\n1\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMinimumSteinerTree"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();
    let v = input.read_size_vec(k).dec();

    let mut poi = BitSet::new(n);
    for &i in &v {
        poi.set(i);
    }
    let graph = Graph::from_biedges(n, &edges);
    let mut ans = 00;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        let mut need = poi[vert];
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            need |= f.call(e.to(), vert);
        }
        if need {
            ans += 1;
        }
        need
    });
    dfs.call(v[0], n);
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
