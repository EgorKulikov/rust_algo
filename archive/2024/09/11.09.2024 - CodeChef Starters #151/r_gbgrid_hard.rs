//{"name":"RGB Grid (Easy)","group":"CodeChef - START151A","url":"https://www.codechef.com/START151A/problems/RGBGRID_SUB1","interactive":false,"timeLimit":3000,"tests":[{"input":"1 3 1000000007\n","output":"2\n"},{"input":"2 1 1000000007\n","output":"0\n"},{"input":"4 40 998244353\n","output":"211047164\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"RGBGridEasy"}}}

use algo_lib::dynamic_value;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::misc::value::DynamicValue;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_utils::powers;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let p = input.read_int();

    dynamic_value!(Module: i32 = p);
    type Mod = ModInt<i32, Module>;
    let p3 = powers(Mod::new(3), n * m);
    let limit = 5.power(n);
    let shift = 5.power(n - 1);
    let mut mem = Memoization3d::new(m + 1, n + 1, limit, |mem, row, col, mask| {
        if row == m {
            Mod::zero()
        } else if col == n {
            mem.call(row + 1, 0, mask)
        } else {
            let mut res = Mod::zero();
            for i in 0..3 {
                if col >= 2 {
                    let middle = mask % 5;
                    if middle != 0 && middle != 4 {
                        let top = mask / 5 % 5;
                        if top == 0 && i == 2 || top == 4 && i == 0 {
                            res += p3[(m - row - 1) * n + n - col - 1];
                            continue;
                        }
                    }
                }
                let top = mask / shift;
                if row >= 2 && i != 1 {
                    if top == 1 && i == 2 || top == 3 && i == 0 {
                        res += p3[(m - row - 1) * n + n - col - 1];
                        continue;
                    }
                }
                let next = if i != 1 {
                    i * 2
                } else {
                    if row == 0 {
                        2
                    } else {
                        match top {
                            0 => 1,
                            1..=3 => 2,
                            4 => 3,
                            _ => unreachable!(),
                        }
                    }
                };
                res += mem.call(row, col + 1, (mask * 5 + next) % limit);
            }
            res
        }
    });
    out.print_line(mem.call(0, 0, 0));
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
