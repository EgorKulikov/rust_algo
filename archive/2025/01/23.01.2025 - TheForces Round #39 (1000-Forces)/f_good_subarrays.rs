//{"name":"F. Good Subarrays","group":"Codeforces - TheForces Round #39 (1000-Forces)","url":"https://codeforces.com/gym/105672/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5 3\n2 1 1 4 1\n8 54\n7 11 53 56 99 28 75 84\n10 21\n73 60 27 67 3 76 52 66 39 27\n","output":"3\n21\n54\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::treap::pure_payload::PurePayload;
use algo_lib::collections::treap::Tree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_long();
    let a = input.read_long_vec(n);

    let mut sum = 0;
    let mut at = DefaultHashMap::new(Vec::new());
    let mut treap = Tree::new();
    treap.insert(PurePayload((0, 0)));
    let mut ans = 0;
    at[0].push((0, true));
    for i in 0..n {
        sum += a[i] - k;
        let key = (sum, i + 1);
        ans += treap.range(..=&key).size();
        treap.insert(PurePayload(key));
        at[sum + 1].push((i + 1, false));
        at[sum].push((i + 1, true));
    }
    let mut delta = vec![0i64; n + 1];
    for v in at.into_values() {
        let mut qty = v.copy_filter(|(_, add)| !*add).count() as i64;
        let mut closes = 0;
        for (i, add) in v {
            if add {
                delta[i] += qty;
                closes += 1;
            } else {
                delta[i] -= closes;
                qty -= 1;
            }
        }
        assert_eq!(qty, 0);
    }
    let mut cur = 0;
    let mut max = 0;
    for i in delta {
        cur += i;
        max.maxim(cur);
    }
    out.print_line(ans + max as usize);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
