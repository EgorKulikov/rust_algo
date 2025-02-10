//{"name":"Breaking Bad","group":"Kattis","url":"https://open.kattis.com/problems/breakingbad","interactive":false,"timeLimit":1000,"tests":[{"input":"5\nbattery_acid\ndrain_cleaner\nantifreeze\ncold_medicine\nlantern_fuel\n2\ncold_medicine battery_acid\nantifreeze lantern_fuel\n","output":"lantern_fuel drain_cleaner battery_acid\nantifreeze cold_medicine\n"},{"input":"3\nfuel\nlighter\nknife\n3\nfuel lighter\nlighter knife\nknife fuel\n","output":"impossible\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BreakingBad"}}}

use algo_lib::collections::id::Id;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let items = input.read_str_vec(n);
    let mut id = Id::new().do_with(|id| id.add(items.iter().cloned()));
    let m = input.read_size();
    let edges = input
        .iter::<(Str, Str)>()
        .take(m)
        .map(|(a, b)| (id.get(a), id.get(b)))
        .collect::<Vec<_>>();

    let graph = Graph::from_biedges(n, &edges);
    let mut color = vec![0; n];
    let mut ans = vec![Vec::new(); 2];
    for i in 0..n {
        if color[i] == 0 {
            let mut dfs = RecursiveFunction2::new(|f, vert: usize, col: i32| -> bool {
                if color[vert] != 0 {
                    if color[vert] != col {
                        return false;
                    }
                    return true;
                }
                color[vert] = col;
                ans[(col + 1) as usize / 2].push(&items[vert]);
                for e in &graph[vert] {
                    if !f.call(e.to(), -col) {
                        return false;
                    }
                }
                true
            });
            if !dfs.call(i, -1) {
                out.print_line("impossible");
                return;
            }
        }
    }
    out.print_per_line(&ans);
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
