//{"name":"D. Root was Built by Love, Broken by Destiny","group":"Codeforces - Atto Round 1 (Codeforces Round 1041, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2127/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 1\n1 2\n3 3\n1 2\n1 3\n2 3\n5 4\n1 2\n1 3\n3 4\n3 5\n4 3\n1 2\n1 3\n1 4\n","output":"2\n0\n8\n12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::One;
use algo_lib::numbers::num_utils::factorials;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    if m != n - 1 {
        out.print_line(0);
        return;
    }
    type Mod = ModInt7;
    let f = factorials::<Mod>(n);
    let mut ans = Mod::one();
    let graph = Graph::with_biedges(n, &edges);
    let mut num_big = 0;
    for i in 0..n {
        if graph[i].len() > 1 {
            num_big += 1;
        }
        let mut single = 0;
        let mut big = 0;
        for e in &graph[i] {
            let to = e.to();
            if graph[to].len() == 1 {
                single += 1;
            } else {
                big += 1;
            }
        }
        if big > 2 {
            out.print_line(0);
            return;
        }
        ans *= f[single];
    }
    ans *= 2;
    if num_big > 1 {
        ans *= 2;
    }
    out.print_line(ans);
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
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
