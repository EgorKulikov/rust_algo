//{"name":"Maximum Product Walk","group":"CodeChef - START213A","url":"https://www.codechef.com/START213A/problems/MXPRWA","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n-1 -1 2\n1 2\n2 3\n3\n5 -10 2\n1 2\n2 3\n2\n1000000000 1000000000\n1 2\n6\n1 -1 1 -1 1 -1\n2 4\n4 6\n2 1\n4 3\n6 5\n","output":"16\n32\n319630959\n16\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::recursive_function::RecursiveFunction2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();

    type Mod = ModIntF;
    let graph = Graph::with_biedges(n, &edges);
    let neg = a.iter().filter(|&&x| x < 0).count();
    let sum = a.copy_map(i64::abs).sum::<i64>();
    if neg % 2 == 0 {
        out.print_line(Mod::new(2).power(sum));
        return;
    }
    let mut ans = 0;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        let mut s = a[vert].abs();
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let call = f.call(e.to(), vert);
            if a[vert] < 0 {
                ans.maxim(call);
            }
            s += call;
        }
        if a[vert] < 0 {
            ans.maxim(sum - s);
        }
        s
    });
    dfs.call(0, n);
    out.print_line(Mod::new(2).power(ans));
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
