//{"name":"H. Sweet Remainders!","group":"Universal Cup - GP of St. Petersburg","url":"https://contest.ucup.ac/contest/3384/problem/17168","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1 5\n0\n2 3\n1 1\n5 4\n0 1 1 1 1\n6 2\n1 0 0 0 0 1\n3 2\n1 1 1\n4 10\n3 3 1 1\n","output":"YES\nYES\n1 2\nYES\n2 1\n3 1\n4 1\n1 5\nYES\n1 2\n2 3\n3 4\n4 5\n5 6\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_long();
    let mut a = input.read_long_vec(n);

    if n == 1 {
        out.print_line(a[0] <= 0);
        return;
    }
    let mut is_special = BitSet::new(n);
    for i in 0..n {
        if a[i] == -1 {
            a[i] = 1;
            is_special.set(i);
        } else if a[i] == 0 {
            a[i] = m;
        }
    }
    let sum = a.copy_sum();
    if sum > (2 * n - 2) as i64
        || is_special.count_ones() == 0 && (sum - (2 * n - 2) as i64) % m != 0
    {
        out.print_line(false);
        return;
    }
    let mut delta = (2 * n - 2) as i64 - sum;
    for i in 0..n {
        if is_special[i] {
            a[i] += delta;
            delta = 0;
        }
    }
    if delta > 0 {
        assert_eq!(delta % m, 0);
        a[0] += delta;
    }
    let mut ones = Vec::new();
    let mut others = Vec::new();
    for i in 0..n {
        if a[i] == 1 {
            ones.push(i);
        } else {
            others.push(i);
        }
    }
    let mut ans = Vec::new();
    for _ in 0..n - 2 {
        let one = ones.pop().unwrap();
        let other = others.pop().unwrap();
        ans.push((one, other));
        a[other] -= 1;
        if a[other] == 1 {
            ones.push(other);
        } else {
            others.push(other);
        }
    }
    assert_eq!(ones.len(), 2);
    assert!(others.is_empty());
    ans.push((ones[0], ones[1]));
    out.print_line(true);
    out.print_per_line(&ans.inc());
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
        _ => {
            unreachable!();
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
