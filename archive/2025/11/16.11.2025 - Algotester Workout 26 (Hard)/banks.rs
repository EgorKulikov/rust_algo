//{"name":"Banks","group":"Algotester","url":"https://algotester.com/en/ContestProblem/DisplayWithFile/147295","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n4 2\n4 2\n6 2\n3 4\n7 1\n3 7\n2 5\n4 7 3 2 4 7 4\n","output":"24 7\n5\n4 3 3\n4 7 8\n4 1 11\n7 5 4\n1 6 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
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
    let x = input.read_size() - 1;
    let y = input.read_size() - 1;
    let edges = input.read_size_pair_vec(n - 1).dec();
    let mut a = input.read_long_vec(n);

    let graph = Graph::with_biedges(n, &edges);
    let mut is_enabled = BitSet::new(n);
    is_enabled.fill(true);
    let dx = graph.edge_distances(x);
    let mut ans = Vec::new();
    for _ in 0..n - 2 {
        let mut end = 0;
        let mut dist = 0;
        for i in 0..n {
            if is_enabled[i] && dx[i] > dist && i != y {
                dist = dx[i];
                end = i;
            }
        }
        let dcur = graph.edge_distances(end);
        is_enabled.unset(end);
        let mut n_end = 0;
        dist = 0;
        for i in 0..n {
            if is_enabled[i] && dx[i] > dist {
                dist = dx[i];
                n_end = i;
            }
        }
        let dd = graph.edge_distances(n_end);
        let mut n_diameter = 0;
        for i in 0..n {
            if is_enabled[i] {
                n_diameter.maxim(dd[i]);
            }
        }
        let mut found = false;
        for i in 0..n {
            if is_enabled[i] && dcur[i] == n_diameter {
                ans.push((i + 1, end + 1, a[end]));
                found = true;
                a[i] += a[end];
                break;
            }
        }
        assert!(found);
    }
    ans.reverse();
    out.print_line((a[x], a[y]));
    out.print_line(ans.len());
    out.print_per_line(&ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
