//{"name":"Colorful Tree (Hard Version)","group":"CodeChef - START153A","url":"https://www.codechef.com/START153A/problems/CTRH","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3\n1 2\n2 3\n5\n3 4\n5 3\n2 3\n1 2\n","output":"6\n18\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ColorfulTreeHardVersion"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    type Mod = ModInt7;
    let mut zero_colors = Mod::new(3);
    let mut one_color = Mod::zero();
    let mut two_colors = Mod::zero();
    for i in 0..n {
        if graph[i].len() == 1 {
            continue;
        }
        for e in &graph[i] {
            let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (Mod, Mod) {
                if graph[vert].len() == 1 {
                    return (Mod::one(), Mod::zero());
                }
                let mut one_color_same = Mod::one();
                let mut two_colors_same = Mod::zero();
                let mut two_colors_diff = Mod::one();
                let mut three_colors_same = Mod::zero();
                let mut three_colors_diff = Mod::zero();
                for e in &graph[vert] {
                    if e.to() == prev {
                        continue;
                    }
                    let (call_diff, call_same) = f.call(e.to(), vert);
                    three_colors_same = two_colors_same * call_diff
                        + three_colors_same * (call_same + call_diff * Mod::new(2));
                    two_colors_same = one_color_same * call_diff * Mod::new(2)
                        + two_colors_same * (call_diff + call_same);
                    one_color_same = one_color_same * call_same;
                    three_colors_diff = two_colors_diff * call_diff
                        + three_colors_diff * (call_same + call_diff * Mod::new(2));
                    two_colors_diff = two_colors_diff * (call_diff + call_same);
                }
                // println!("{} {} {}", vert, three_colors_diff, three_colors_same);
                (three_colors_diff, three_colors_same)
            });
            let (call_diff, call_same) = dfs.call(e.to(), i);
            two_colors = two_colors * (call_diff * Mod::new(2) + call_same) + one_color * call_diff;
            one_color = one_color * (call_diff + call_same) + zero_colors * call_diff * Mod::new(2);
            zero_colors *= call_same;
        }
        out.print_line(two_colors);
        return;
    }
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
