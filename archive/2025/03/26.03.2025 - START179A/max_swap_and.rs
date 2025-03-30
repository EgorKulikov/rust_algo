//{"name":"Max Swap And","group":"CodeChef - START179A","url":"https://www.codechef.com/START179A/problems/SWAPAND","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\n5 4 2\n6 2 15\n2\n1 2\n4 8\n4\n1 2 3 4\n5 6 7 8\n1\n11\n8\n2\n5 1\n0 4\n","output":"6\n0\n4\n19\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);

    let mut dsu = DSU::new(2 * n);
    let mut ans = 0i64;
    for i in (0..30).rev() {
        let mut both = true;
        let mut can = true;
        for j in 0..n {
            if !a[j].is_set(i) && !b[j].is_set(i) {
                can = false;
                break;
            }
            if !a[j].is_set(i) || !b[j].is_set(i) {
                both = false;
            }
        }
        if !can {
            continue;
        }
        if both {
            ans += 1 << (i + 1);
            continue;
        }
        let mut ndsu = dsu.clone();
        let mut first = None;
        for j in 0..n {
            if !a[j].is_set(i) || !b[j].is_set(i) {
                if let Some((id, val)) = first {
                    let c_val = a[j].is_set(i);
                    let i1 = 2 * id + val as usize;
                    let j1 = 2 * j + c_val as usize;
                    let i2 = i1 ^ 1;
                    let j2 = j1 ^ 1;
                    ndsu.union(i1, j1);
                    ndsu.union(i2, j2);
                } else {
                    first = Some((j, a[j].is_set(i)));
                }
            }
        }
        let mut good = true;
        for i in 0..n {
            if ndsu.find(2 * i) == ndsu.find(2 * i + 1) {
                good = false;
                break;
            }
        }
        if good {
            dsu = ndsu;
            ans += 1 << i;
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
