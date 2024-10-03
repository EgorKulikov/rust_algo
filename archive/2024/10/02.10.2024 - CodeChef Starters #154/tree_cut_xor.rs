//{"name":"Tree Cut Xor","group":"CodeChef - START154A","url":"https://www.codechef.com/START154A/problems/TREECUTXOR","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\n1 2\n3\n1 2\n1 3\n7\n1 2\n4 2\n3 1\n5 7\n2 5\n5 6\n","output":"1\n1 2 1\n0\n3 1 3\n1 2 2\n0\n2 5 5\n1 3 1\n4 2 2\n1 2 1\n5 6 5\n5 7 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TreeCutXor"}}}

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
    let edges = input.read_size_pair_vec(n - 1).dec();

    if n == 2 {
        out.print_line(1);
        out.print_line((1, 2, 1));
        return;
    }
    let graph = Graph::from_biedges(n, &edges);
    let mut ans = Vec::new();
    out.print_line(0);
    let mut xor = 0;
    let mut size = n;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert);
            if size == 4 && xor == 0 || size == 3 && xor == 3 {
                ans.push((vert + 1, e.to() + 1, vert + 1));
                xor ^= size - 1;
            } else {
                ans.push((vert + 1, e.to() + 1, e.to() + 1));
                xor ^= 1;
            }
            size -= 1;
        }
    });
    dfs.call(0, 0);
    assert_eq!(xor, 0);
    out.print_per_line(&ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
