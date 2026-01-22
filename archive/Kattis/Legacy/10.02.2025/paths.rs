//{"name":"Paths","group":"Kattis","url":"https://open.kattis.com/problems/paths","interactive":false,"timeLimit":3000,"tests":[{"input":"4 3 3\n1 2 1 3\n1 2\n2 3\n4 2\n","output":"10\n"},{"input":"9 11 4\n1 2 3 4 1 2 1 2 2\n1 2\n1 3\n2 3\n2 4\n3 6\n6 2\n6 5\n4 3\n4 5\n7 8\n9 8\n","output":"70\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let col = input.read_size_vec(n).dec();
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut mem = Memoization2d::new(n, 1 << k, |mem, vert, mask| -> i64 {
        if mask.is_set(col[vert]) {
            0
        } else {
            let mut res = 1;
            for e in &graph[vert] {
                res += mem.call(e.to(), mask.with_bit(col[vert]));
            }
            res
        }
    });
    let mut ans = 0;
    for i in 0..n {
        ans += mem.call(i, 0) - 1;
    }
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
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
//END MAIN
