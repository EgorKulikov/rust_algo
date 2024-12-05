//{"name":"day_5","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_5"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::topological_sort::TopologicalSort;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::str_scan;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut before = Arr2d::new(100, 100, false);

    loop {
        let s = input.read_line();
        if s.is_empty() {
            break;
        }
        str_scan!(s, "@|@", a: usize, b: usize);
        // graph.add_edge(WeightedEdge::new(a, b, 1));
        before[(a, b)] = true;
    }

    let mut data = Vec::new();
    while !input.is_empty() {
        data.push(
            input
                .read_line()
                .split(',')
                .iter_map(|x| x.parse::<usize>())
                .collect::<Vec<_>>(),
        );
    }

    let mut is_incorrect = BitSet::new(data.len());
    // part 1
    {
        let mut ans = 0;
        for (id, row) in data.iter().enumerate() {
            let mut ok = true;
            for (i, j) in row.consecutive_iter() {
                if before[(*j, *i)] {
                    ok = false;
                    break;
                }
            }
            if ok {
                ans += row[row.len() / 2];
            } else {
                is_incorrect.set(id);
            }
        }
        out.print_line(ans);
    }

    // part 2
    {
        let mut ans = 0;
        for (id, row) in data.iter().enumerate() {
            if !is_incorrect[id] {
                continue;
            }
            let mut graph = Graph::new(100);
            for i in row.indices() {
                for j in row.indices() {
                    if before[(row[i], row[j])] {
                        graph.add_edge(Edge::new(row[i], row[j]));
                    }
                }
            }
            let pos = graph.topological_sort().unwrap().inv();
            let mut vert = row.clone();
            vert.sort_by_key(|&x| pos[x]);
            ans += vert[vert.len() / 2];
        }
        out.print_line(ans);
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
