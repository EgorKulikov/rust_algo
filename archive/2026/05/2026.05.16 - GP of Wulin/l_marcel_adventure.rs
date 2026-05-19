//{"name":"L. Marcel Adventure","group":"Universal Cup - GP of Wulin","url":"https://contest.ucup.ac/contest/3749/problem/18130","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n6 4\n3 3 3 5 5 5\n3 3 2\n5 5 4\n5 5 7\n4 5 5\n2 3 2\n4 1 1\n-1 0 3\n2 -2 -1\n1 4 9\n1 0 0\n3 2\n0 0 1 1 1 1\n0 0 0\n1 1 0\n1 1 2\n1 1 1\n1 1 2\n","output":"5\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let st = input.read_long_vec(3);
    let end = input.read_long_vec(3);
    let mut obst = Vec::with_capacity(n);
    for _ in 0..n {
        obst.push(input.read_long_vec(3));
    }
    let mut shifts = Vec::with_capacity(m);
    for _ in 0..m {
        shifts.push(input.read_long_vec(3));
    }

    let mut pos = DefaultHashMap::new(Vec::new());
    let mut st_id = n;
    let mut end_id = n;
    for i in 0..n {
        if obst[i][0] == st[0] && obst[i][1] == st[1] && obst[i][2] == st[2] - 1 {
            st_id = i;
        }
        if obst[i][0] == end[0] && obst[i][1] == end[1] && obst[i][2] == end[2] - 1 {
            end_id = i;
        }
        pos[(obst[i][0], obst[i][1])].push((obst[i][2], i));
    }
    for v in pos.values_mut() {
        v.sort();
    }
    assert_ne!(st_id, n);
    assert_ne!(end_id, n);
    let mut graph = Graph::new_linked(n);
    for i in 0..n {
        for j in 0..m {
            let x = obst[i][0] + shifts[j][0];
            let y = obst[i][1] + shifts[j][1];
            let z = obst[i][2] + shifts[j][2] + 1;
            let p = pos[(x, y)].upper_bound(&(z, n));
            if p == 0 || pos[(x, y)][p - 1].0 == z {
                continue;
            }
            graph.add_edge(Edge::new(i, pos[(x, y)][p - 1].1));
        }
    }
    let dist = graph.edge_distances(st_id)[end_id];
    if dist == u32::MAX {
        out.print_line(-1);
    } else {
        out.print_line(dist);
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
