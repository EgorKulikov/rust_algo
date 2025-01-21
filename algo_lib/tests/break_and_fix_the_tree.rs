//{"name":"Break And Fix The Tree","group":"CodeChef - CDRV2021","url":"https://www.codechef.com/problems/FIXTREE","interactive":false,"timeLimit":5000,"tests":[{"input":"5\n1 1 2 2\n1 2 3 4 5\n6\n3 4 5\n1 4 3\n1 5 3\n1 2 4\n2 4 10\n3 2 5\n","output":"11\n20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::link_cut::{LinkCutNode, LinkCutPayload};
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut p = input.read_size_vec(n - 1).dec();
    let v = input.read_long_vec(n);

    struct Node {
        val: i64,
        sum: i64,
    }
    impl LinkCutPayload for Node {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum = self.val + left.map_or(0, |x| x.sum) + right.map_or(0, |x| x.sum);
        }
    }

    let nodes = Vec::with_gen(n, |i| {
        LinkCutNode::new(Node {
            val: v[i],
            sum: v[i],
        })
    });
    for i in 0..n - 1 {
        nodes[i + 1].link(nodes[p[i]]);
    }

    let q = input.read_size();
    for _ in 0..q {
        let t = input.read_int();
        match t {
            1 => {
                let x = input.read_size() - 1;
                let w = input.read_size() - 1;
                nodes[x].cut();
                nodes[x].link(nodes[w]);
                p[x - 1] = w;
            }
            2 => {
                let x = input.read_size() - 1;
                let k = input.read_long();
                nodes[x].with_payload_mut(|node| node.val = k);
            }
            3 => {
                let x = input.read_size() - 1;
                let y = input.read_size() - 1;
                let lca = LinkCutNode::lca(nodes[x], nodes[y]).unwrap();
                let parent = lca.parent();
                lca.cut();
                let ans = nodes[x].with_payload(|p| p.sum) + nodes[y].with_payload(|p| p.sum)
                    - lca.with_payload(|p| p.val);
                if let Some(parent) = parent {
                    lca.link(parent);
                }
                out.print_line(ans);
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

mod tester {
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(dead_code)]
    #![allow(unused_imports)]

    use crate::{run, TASK_TYPE};
    use algo_lib::io::input::Input;
    use algo_lib::io::output::Output;
    use algo_lib::misc::random::Random;
    use tester::classic::default_checker;
    use tester::interactive::std_interactor;
    use tester::test_set::GeneratedTestSet;
    use tester::Tester;

    const PRINT_LIMIT: usize = 1000;

    fn interact(
        mut sol_input: Input,
        mut sol_output: Output,
        mut input: Input,
    ) -> Result<(), String> {
        Ok(())
    }

    fn check(mut input: Input, expected: Option<Input>, mut output: Input) -> Result<(), String> {
        Ok(())
    }

    struct StressTest;

    impl GeneratedTestSet for StressTest {
        type TestId = usize;

        fn tests(&self) -> Box<dyn Iterator<Item = Self::TestId>> {
            Box::new(1..)
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {
            let mut r = Random::new();
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    struct MaxTest;

    impl GeneratedTestSet for MaxTest {
        type TestId = usize;

        fn tests(&self) -> Box<dyn Iterator<Item = Self::TestId>> {
            Box::new(1..=1)
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {
            let mut r = Random::new();
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./break_and_fix_the_tree";
        let tl = 5000;
        let tester = match TASK_TYPE {
            crate::TaskType::Interactive => {
                Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
                // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, interact)
            }
            crate::TaskType::Classic => {
                Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, default_checker)
                // Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, check)
            }
        };
        let passed = tester.test_samples();
        // tester.test_generated("Max test", true, MaxTest);
        // tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn break_and_fix_the_tree() {
    assert!(tester::run_tests());
}
