//{"name":"P11363 [NOIP2024] 树的遍历（民间数据）","group":"Luogu","url":"https://www.luogu.com.cn/problem/P11363?contestId=217331","interactive":false,"timeLimit":1000,"tests":[{"input":"1 1\n4 1\n1 2\n2 3\n2 4\n1\n","output":"2\n"},{"input":"7 1\n5 2\n1 2\n1 3\n2 4\n2 5\n1 3\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P11363NOIP2024"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdgeWithId;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_utils::factorials;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();
    let special = input.read_size_vec(k).dec();

    let is_special = vec![false; n - 1].do_with(|bs| {
        for s in special {
            bs[s] = true;
        }
    });
    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(BiEdgeWithId::new(u, v));
    }
    type Mod = ModInt7;
    let ff = factorials::<Mod>(n);
    let mut base = Mod::one();
    for i in 0..n {
        base *= ff[graph[i].len() - 1];
    }
    let mut ans = base * Mod::from_index(k);

    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> Mod {
        let mut total = Mod::zero();
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let mut call = f.call(e.to(), vert);
            if is_special[e.id()] {
                ans -= base * call;
                call = Mod::one();
            }
            if total != Mod::zero() {
                ans -= base * call * total / Mod::from_index(graph[vert].len() - 1);
            }
            total += call;
        }
        if graph[vert].len() == 1 {
            assert!(total == Mod::zero() || vert == 0);
            Mod::zero()
        } else {
            total / Mod::from_index(graph[vert].len() - 1)
        }
    });
    dfs.call(0, n);
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    input.read_size();

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
