//{"name":"E. Fog Canyon","group":"Codeforces - GoForGold Long Challenge - January 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/579716/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"8 5\n1 1 1 1 1 2 8 2\n1 1\n0 1 3\n1 1\n0 3 4\n1 2\n","output":"8 7\n8 5\n7 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::link_cut::{LinkCutNode, LinkCutPayload};
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let h = input.read_size_vec(n);
    struct Node(usize);
    impl LinkCutPayload for Node {
        fn update(&mut self, _left: Option<&Self>, _right: Option<&Self>) {}
    }
    let nodes = Vec::with_gen_back(n, |i, nodes| {
        let node = LinkCutNode::new(Node(i + 1));
        if i + h[i] < n {
            node.link(nodes[i + h[i]]);
        }
        node
    });
    for _ in 0..m {
        let t = input.read_int();
        match t {
            0 => {
                let i = input.read_size() - 1;
                let x = input.read_size();
                nodes[i].cut();
                if i + x < n {
                    nodes[i].link(nodes[i + x]);
                }
            }
            1 => {
                let i = input.read_size() - 1;
                let len = nodes[i].dist_to_root() + 1;
                let last = nodes[i].find_root().payload().0;
                out.print_line((last, len));
            }
            _ => unreachable!(),
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
