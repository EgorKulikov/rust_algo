//{"name":"uc9_b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"uc9_b"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let x = input.read_long();
    let y_max = input.read_unsigned();

    let mut id = vec![None; 15];
    let mut j = 0;
    let mut ys = Vec::new();
    for i in 0..15 {
        if y_max.is_set(i) {
            id[i] = Some(j);
            ys.push(i);
            j += 1;
        }
    }
    const MOD: i64 = 1_000_000_007;
    let mut mem = Arr2d::new(32, 1 << j, 0i64);
    mem[(0, 0)] = 1;
    for x_bit in (0..45).rev() {
        for y_id in (0..=j).rev() {
            if y_id == j {
                for carry in (0..32).rev() {
                    for y_mask in usize::iter_all(j) {
                        if (carry % 2 == 1) == x.is_set(x_bit) {
                            mem[(carry, y_mask)] = mem[(carry / 2, y_mask)] % MOD;
                        } else {
                            mem[(carry, y_mask)] = 0;
                        }
                    }
                }
            } else {
                for carry in 0..31 {
                    for y_mask in usize::iter_all(j) {
                        if ys[y_id] <= x_bit
                            && (n >> (x_bit - ys[y_id]) & 1) != 0
                            && (y_mask >> y_id & 1) != 0
                        {
                            mem[(carry, y_mask)] +=
                                mem[(carry + 1, y_mask)] + mem[(carry + 1, y_mask - (1 << y_id))];
                        }
                    }
                }
            }
        }
    }
    let mut ans = Vec::with_capacity(1 << j);
    for y_mask in usize::iter_all(j) {
        ans.push(mem[(0, y_mask)] % MOD);
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
