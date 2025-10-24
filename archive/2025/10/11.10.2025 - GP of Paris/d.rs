//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let k = input.read_size();

    let mut m = k;
    let mut zero2 = 0;
    while m % 2 == 0 {
        m /= 2;
        zero2 += 1;
    }
    let mut zero5 = 0;
    while m % 5 == 0 {
        m /= 5;
        zero5 += 1;
    }
    let zero = zero2.max(zero5);
    let mut ans = vec![0; 20 * k - zero];
    let mut rem = k;
    let mut r = 1;
    let mut cur = 0;
    for i in ans.indices() {
        if rem >= 9 {
            ans[i] = 9;
            rem -= 9;
            cur += 9 * r;
        } else {
            ans[i] = rem;
            cur += rem * r;
            cur %= m;
            break;
        }
        r *= 10;
        r %= m;
        cur %= m;
    }
    let mut shift = 0;
    let mut r = 1;
    for i in ans.indices() {
        if ans[i] == 0 && r == 1 % m {
            shift = i;
            break;
        }
        r *= 10;
        r %= m;
    }
    assert_ne!(shift, 0);
    let mut add = 0;
    while (cur + 9 * add) % m != 0 {
        add += 1;
    }
    let mut at = 0;
    while add > 0 {
        let c = add % 10;
        add /= 10;
        assert!(ans[at] >= c);
        ans[at] -= c;
        ans[shift + at + 1] += c;
        at += 1;
    }
    let mut non_zero_found = false;
    for &x in ans.iter().rev() {
        if x != 0 {
            non_zero_found = true;
        }
        if non_zero_found {
            out.print(x);
        }
    }
    for _ in 0..zero {
        out.print(0);
    }
    out.print_line(());
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
