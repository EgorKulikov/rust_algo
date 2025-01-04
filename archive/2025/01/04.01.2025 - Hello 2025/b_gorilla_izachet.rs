//{"name":"B. Горилла и зачет","group":"Codeforces - Hello 2025","url":"https://codeforces.com/contest/2057/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1 0\n48843\n3 1\n2 3 2\n5 3\n1 2 3 4 5\n7 0\n4 7 1 3 2 4 1\n11 4\n3 2 1 4 4 3 4 2 1 3 3\n5 5\n1 2 3 4 5\n","output":"1\n1\n2\n5\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BGorillaIZachet"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut k = input.read_size();
    let a = input.read_int_vec(n);

    let mut qty = DefaultHashMap::new(0);
    for i in a.copy_iter() {
        qty[i] += 1;
    }
    let mut qty = qty.into_iter().collect::<Vec<_>>();
    qty.sort_unstable_by_key(|x| x.1);
    for i in qty.indices() {
        let cur = qty[i].1;
        if cur <= k {
            k -= cur;
        } else {
            out.print_line(qty.len() - i);
            return;
        }
    }
    out.print_line(1);
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
