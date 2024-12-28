//{"name":"E. Resourceful Caterpillar Sequence","group":"Codeforces - Good Bye 2024: 2025 is NEAR","url":"https://codeforces.com/contest/2053/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n1 2\n5\n1 2\n1 3\n2 4\n2 5\n12\n1 6\n11 2\n4 8\n12 3\n2 7\n6 12\n8 1\n2 3\n5 12\n9 2\n10 3\n10\n1 2\n2 3\n3 4\n4 5\n5 6\n4 7\n6 8\n4 9\n4 10\n25\n1 16\n11 22\n6 14\n3 1\n20 14\n23 17\n25 19\n10 11\n3 18\n10 6\n2 21\n4 5\n11 12\n4 9\n9 13\n8 6\n6 1\n3 7\n8 19\n10 24\n15 13\n1 2\n3 4\n17 8\n","output":"0\n6\n40\n27\n171\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EResourcefulCaterpillarSequence"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let mut leafs = 0;
    let mut far = 0;
    let graph = Graph::from_biedges(n, &edges);
    for i in 0..n {
        if graph[i].len() == 1 {
            leafs += 1;
        } else {
            let mut is_far = true;
            for e in &graph[i] {
                if graph[e.to()].len() == 1 {
                    is_far = false;
                    break;
                }
            }
            if is_far {
                far += 1;
            }
        }
    }
    let mut ans = leafs * (n - leafs);
    for i in 0..n {
        if graph[i].len() == 1 {
            continue;
        }
        let mut is_near_leaf = false;
        let mut non_leaf_edges = 0;
        for e in &graph[i] {
            if graph[e.to()].len() == 1 {
                is_near_leaf = true;
            } else {
                non_leaf_edges += 1;
            }
        }
        if is_near_leaf && non_leaf_edges > 0 {
            ans += far * (non_leaf_edges - 1);
        }
    }
    out.print_line(ans);
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
