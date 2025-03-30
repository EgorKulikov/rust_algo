//{"name":"D - Reverse Brackets","group":"AtCoder - AtCoder Regular Contest 194 (Div. 2)","url":"https://atcoder.jp/contests/arc194/tasks/arc194_d","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n(())()\n","output":"2\n"},{"input":"2\n()\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable0, RecursiveFunction0};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::One;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(n + 1);
    let mut ans = Mod::one();
    let mut pos = 0;
    let mut rec = RecursiveFunction0::new(|rec| -> Str {
        let mut res = Vec::new();
        let mut by_type = DefaultHashMap::new(0);
        while pos < n && s[pos] == b'(' {
            pos += 1;
            let inner = rec.call();
            by_type[inner.clone()] += 1;
            res.push(inner);
        }
        let mut so_far = 0;
        for v in by_type.into_values() {
            so_far += v;
            ans *= c.c(so_far, v);
        }
        res.sort();
        let mut ans = Str::new();
        ans.push(b'(');
        for r in res {
            ans.extend_from_slice(r.as_slice());
        }
        ans.push(b')');
        pos += 1;
        ans
    });
    rec.call();
    out.print_line(ans);
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
