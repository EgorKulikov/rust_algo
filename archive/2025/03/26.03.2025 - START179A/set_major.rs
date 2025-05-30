//{"name":"Set Major","group":"CodeChef - START179A","url":"https://www.codechef.com/START179A/problems/SETMJR","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n5\n1 1 2 1 2\n4\n3 3 3 3\n","output":"6\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::by_index;
use algo_lib::collections::treap::treap_map::TreapSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    let by_pos = by_index(&a);
    let mut ans = 0;
    for v in by_pos.into_values() {
        let mut cur = 0;
        let mut val = 0;
        let mut prev = Vec::new();
        let mut limit = Vec::new();
        let mut nodes = TreapSet::new();
        for i in v {
            let minus = i - cur;
            if minus >= val {
                val = 0;
                prev.clear();
                limit.clear();
                nodes.clear();
            } else {
                for _ in 0..minus {
                    val -= 1;
                    limit[val] = i;
                }
            }
            cur = i + 1;
            if let Some(&p) = prev.get(val) {
                nodes.remove(&p);
            }
            nodes.insert(i);
            let c_limit = limit.get(val).copied().unwrap_or(0);
            ans += nodes.more_or_eq(&c_limit);
            if prev.len() == val {
                prev.push(i);
                limit.push(i);
            } else {
                prev[val] = i;
                limit[val] = i;
            }
            val += 1;
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
