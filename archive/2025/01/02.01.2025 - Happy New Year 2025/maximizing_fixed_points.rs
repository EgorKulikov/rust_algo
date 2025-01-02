//{"name":"Maximizing Fixed Points","group":"SeriousOJ - Happy New Year 2025","url":"https://judge.eluminatis-of-lu.com/contest/676ffd92569fb90008aac7da/1119","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3 2\n1 3 2\n1 2\n2 3\n3 2\n3 2 1\n1 2\n2 3\n","output":"3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MaximizingFixedPoints"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n).dec();
    let edges = input.read_size_pair_vec(q).dec();

    let mut dsu = DSU::new(n);
    for (u, v) in edges {
        dsu.union(u, v);
    }
    let sets = dsu.parts();
    let mut ans = 0;
    for part in sets {
        let set = part.copy_iter().collect::<FxHashSet<_>>();
        for i in part {
            if set.contains(&a[i]) {
                ans += 1;
            }
        }
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
