//{"name":"Maximum Smaller","group":"CodeChef - START207A","url":"https://www.codechef.com/START207A/problems/MXSMALL","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3\n1 1 1\n3\n3 1 2\n2\n2 2\n","output":"3\n2 3 1\n2\n3 1 2\n2\n2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::by_index;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut ans = vec![0; n];
    let bi = by_index(&a);
    if bi.values().all(|x| x.len() > 1) {
        for v in bi.values() {
            for j in 1..v.len() {
                ans[v[j - 1]] = v[j];
            }
            ans[v[Back(0)]] = v[0];
        }
        out.print_line(n);
        out.print_line(ans.inc());
        return;
    }
    let order = (0..n).collect::<Vec<_>>().sorted_by_key(|&i| a[i]);
    for i in 1..n {
        ans[order[i]] = order[i - 1];
    }
    ans[order[0]] = order[Back(0)];
    out.print_line(n - 1);
    out.print_line(ans.inc());
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
