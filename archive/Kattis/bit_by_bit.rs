//{"name":"Bit by Bit","group":"Kattis","url":"https://open.kattis.com/problems/bitbybit","interactive":false,"timeLimit":1000,"tests":[{"input":"3\nSET 0\nCLEAR 1\nAND 2 2\n6\nSET 31\nSET 30\nCLEAR 29\nAND 29 30\nOR 29 30\nAND 30 28\n0\n","output":"??????????????????????????????01\n1?1?????????????????????????????\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    if n == 0 {
        return;
    }

    #[derive(Copy, Clone, Eq, PartialEq)]
    enum Bit {
        Zero,
        One,
        Unknown,
    }
    impl Bit {
        fn clear(&mut self) {
            *self = Self::Zero;
        }

        fn set(&mut self) {
            *self = Self::One;
        }

        fn or(&mut self, other: Self) {
            if other == Self::One {
                *self = Self::One;
            } else if other == Self::Unknown && *self == Self::Zero {
                *self = Self::Unknown;
            }
        }

        fn and(&mut self, other: Self) {
            if other == Self::Zero {
                *self = Self::Zero;
            } else if other == Self::Unknown && *self == Self::One {
                *self = Self::Unknown;
            }
        }
    }
    let mut ans = [Bit::Unknown; 32];
    for _ in 0..n {
        let t = input.read_str();
        match t.as_slice() {
            b"SET" => {
                let i = input.read_size();
                ans[i].set();
            }
            b"CLEAR" => {
                let i = input.read_size();
                ans[i].clear();
            }
            b"AND" => {
                let i = input.read_size();
                let j = input.read_size();
                ans[i].and(ans[j]);
            }
            b"OR" => {
                let i = input.read_size();
                let j = input.read_size();
                ans[i].or(ans[j]);
            }
            _ => unreachable!(),
        }
    }
    for a in ans.iter_rev() {
        match a {
            Bit::Zero => out.print("0"),
            Bit::One => out.print("1"),
            Bit::Unknown => out.print("?"),
        }
    }
    out.print_line(());
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
