//{"name":"Новий працівник","group":"Algotester","url":"https://algotester.com/en/ContestProblem/DisplayWithFile/136186","interactive":false,"timeLimit":5000,"tests":[{"input":"4 4 3\n1 2 1\n2 1 4\n2 3 5\n2 4 7\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::bin_search::search_first_true;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let edges = input.read_vec::<(usize, usize, i32)>(m).dec();

    let graph = Graph::new(n).do_with(|g| {
        for (u, v, w) in edges.copy_iter() {
            g.add_edge(Edge::with_payload(u, v, w));
        }
    });
    out.print_line(search_first_true(0, 1_000_000, |x| {
        let mut rem = vec![0; n];
        for (_, v, w) in edges.copy_iter() {
            if w > x {
                rem[v] += 1;
            }
        }
        let mut queue = Vec::new();
        for i in 0..n {
            if rem[i] == 0 {
                queue.push(i);
            }
        }
        let mut done = 0;
        while let Some(vert) = queue.pop() {
            done += 1;
            if done == k {
                return true;
            }
            for e in &graph[vert] {
                if *e.payload() > x {
                    rem[e.to()] -= 1;
                    if rem[e.to()] == 0 {
                        queue.push(e.to());
                    }
                }
            }
        }
        false
    }));
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
