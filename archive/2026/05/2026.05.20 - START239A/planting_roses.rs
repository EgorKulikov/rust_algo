//{"name":"Planting Roses","group":"CodeChef - START239A","url":"https://www.codechef.com/START239A/problems/ROSEPL","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3 4 2\n3 1 2\n4 2 2\n4 5 3 8\n4 20 3\n4 12 6 14\n5 1000000000000 123456789\n1 11 111 1111 11111\n","output":"3\n2\n15\n12345\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut m = input.read_size() + 1;
    let k = input.read_size();
    let mut a = input.read_size_vec(n);

    let mut full = 0;
    for i in 0..n {
        full += a[i] / k;
        a[i] %= k;
    }
    a.sort();
    a.reverse();
    let mut ans = 0;
    let can_full = (m / (k + 1)).min(full);
    full -= can_full;
    ans += can_full * k;
    m -= can_full * (k + 1);
    if full > 0 {
        ans += m.saturating_sub(1);
    } else {
        for i in a {
            if m <= i {
                ans += m.saturating_sub(1);
                break;
            } else {
                ans += i;
                m -= i + 1;
            }
        }
    }
    out.print_line(ans);
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
