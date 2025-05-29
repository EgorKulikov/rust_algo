//{"name":"GCD and XOR 2 (Easy)","group":"CodeChef - START188A","url":"https://www.codechef.com/START188A/problems/GCDXOR2","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n3 2\n4 1\n4 14\n7 5\n10 20\n60 1800\n","output":"1\n7\n0\n6\n13\n401643732\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::dynamic_value;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::mod_int::{ModInt, ModIntF};
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let d = input.read_int();

    dynamic_value!(DModulo: u32 = d as u32);
    type DMod = ModInt<DModulo>;

    let free = n - d.highest_bit() - 1;
    let modulo = 1 << (d.highest_bit() + 1);
    let mut limit = 1;
    for _ in 0..free.min(30) {
        limit *= 2;
    }
    let rem = (DMod::new(2).power(free) - DMod::one()).val();

    type Mod = ModIntF;
    let pw = Mod::new(2).power(free) - Mod::one();
    let mut ans = Mod::zero();

    for i in 0..modulo - d {
        let xor = i ^ (i + d);

        if d % xor != 0 {
            continue;
        }
        let mut delta = rem;
        for j in 0..d.min(limit) {
            let a = i + j * modulo;
            let b = a + d;
            if gcd(a, b) == xor {
                ans +=
                    (pw - Mod::new_signed(j) - Mod::new(delta)) / Mod::new_signed(d) + Mod::one();
            }
            if delta == 0 {
                delta = d as u32 - 1;
            } else {
                delta -= 1;
            }
        }
    }
    out.print_line(ans - Mod::one());
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
