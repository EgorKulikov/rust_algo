//{"name":"Count Transformations","group":"Eolymp - Basecamp - Blitz Round #4","url":"https://eolymp.com/en/compete/7mk1e6onrl4pb69590dkne46j4/problem/2","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n3\n176\n3\n123\n4\n1002\n4\n9275\n10\n1248124811\n20\n11111111111111111111\n","output":"8\n32\n16\n4\n48593\n319786\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::algebra::One;
use algo_lib::string::str::StrReader;
use algo_lib::value;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let x = input.read_str();

    value!(Modulo: u32 = 999983);
    type Mod = ModInt<Modulo>;
    let mut ans = Mod::one();
    for c in x {
        if c == b'0' {
            continue;
        }
        let mut digit = (c - b'0') as u32;
        while digit % 2 == 0 {
            digit /= 2;
        }
        let mut cur = 0;
        while digit < 10 {
            cur += 1;
            digit *= 2;
        }
        ans *= cur;
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
