//{"name":"T436523 「GFOI Round 2」Jom & Terry","group":"Luogu","url":"https://www.luogu.com.cn/problem/T436523?contestId=210735","interactive":false,"timeLimit":1000,"tests":[{"input":"5 4 3\n4 3\n3 2\n1 5\n1 2\n2\n1 2\n5 4\n","output":"I'm here!\nJom\nJom\n"},{"input":"5 5 4\n1 4\n4 3\n3 2\n4 5\n5 3\n2\n3 1\n5 1\n","output":"I'm here!\nTerry\nTerry\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T436523GFOIRound2JomTerry"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let r = input.read_size() - 1;
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::from_biedges(n, &edges);
    let dist = graph.edge_distances(r);

    out.print_line("I'm here!");
    let q = input.read_size();
    for _ in 0..q {
        let a = input.read_size() - 1;
        let b = input.read_size() - 1;
        if dist[a] > dist[b] {
            out.print_line("Jom");
        } else {
            out.print_line("Terry");
        }
    }
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
