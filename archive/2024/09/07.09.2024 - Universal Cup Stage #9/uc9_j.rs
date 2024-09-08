//{"name":"uc9_j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"uc9_j"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::graph::all_distances::AllDistances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = Vec<Vec<Arr2d<i32>>>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let t = input.read_size();
    let n = input.read_size();
    let k = input.read_size();
    let x = input.read_size();
    let y = input.read_size();

    if t == 0 {
        out.print_line(x.abs_diff(y));
    } else if k == 1 {
        out.print_line(1);
    } else if n <= k {
        out.print_line(-1);
    } else if k == 2 {
        if n == 3 {
            if t == 1 {
                if (x, y) == (1, 3) || (x, y) == (3, 1) {
                    out.print_line(1);
                } else {
                    out.print_line(-1);
                }
            } else {
                out.print_line(-1);
            }
        } else {
            if t % 2 == 0 {
                out.print_line(x.abs_diff(y));
            } else if x.abs_diff(y) == 1 {
                if n == 4 && ((x, y) == (2, 3) || (x, y) == (3, 2)) {
                    out.print_line(3);
                } else {
                    out.print_line(2);
                }
            } else {
                out.print_line(1);
            }
        }
    } else if t == 1 {
        if x.abs_diff(y) >= k {
            out.print_line(1);
        } else if x.abs_diff(n) >= k && y.abs_diff(n) >= k
            || x.abs_diff(1) >= k && y.abs_diff(1) >= k
        {
            out.print_line(2);
        } else if (x.abs_diff(n) >= k || x.abs_diff(1) >= k)
            && (y.abs_diff(n) >= k || y.abs_diff(1) >= k)
        {
            out.print_line(3);
        } else {
            out.print_line(-1);
        }
    } else if k >= 4 || n > 7 {
        out.print_line(-1);
    } else {
        if data[n].len() > t {
            out.print_line(if data[n][t][(x - 1, y - 1)] == i32::MAX {
                -1
            } else {
                data[n][t][(x - 1, y - 1)]
            });
        } else {
            out.print_line(-1);
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut res = vec![Vec::new(); 8];
    for n in 4..=7 {
        let mut graph = Graph::new(n);
        for i in 1..n {
            graph.add_edge(BiWeightedEdge::new(i - 1, i, 1));
        }
        while graph.edge_count() > 0 {
            let d = graph.all_distances();
            graph.clear();
            for i in 0..n {
                for j in 0..i {
                    if d[(i, j)] != i32::MAX && d[(i, j)] >= 3 {
                        graph.add_edge(BiWeightedEdge::new(i, j, 1));
                    }
                }
            }
            res[n].push(d);
        }
    }
    let mut pre_calc = res;

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
