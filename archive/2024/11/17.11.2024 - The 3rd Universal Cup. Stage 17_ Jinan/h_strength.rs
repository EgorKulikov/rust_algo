//{"name":"H. Strength","group":"Universal Cup - The 3rd Universal Cup. Stage 17: Jinan","url":"https://contest.ucup.ac/contest/1843/problem/9555","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n0 2147483646\n10 100\n671232353 1232363\n123001006660996 3122507962333010\n100019990010301090 44519984489341188\n","output":"2147483647\n55\n0\n1919810\n114514\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HStrength"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let x = input.read_long();
    let z = input.read_long();

    #[inline]
    fn round(x: usize) -> usize {
        if x < 5 {
            0
        } else {
            10
        }
    }

    let mut ans = 0;
    let mut zero = 1i64;
    let mut one = 0;
    let mut both = 0;
    let mut exact_zero = 1i64;
    let mut exact_one = 0;
    let mut exact_both = 0;
    let mut xc = x;
    let mut zc = z;
    for _ in 0..19 {
        let cur = (xc % 10) as usize;
        xc /= 10;
        let z_cur = (zc % 10) as usize;
        zc /= 10;
        let mut n_zero = 0;
        let mut n_one = 0;
        let mut n_both = 0;
        let mut n_exact_zero = 0;
        let mut n_exact_one = 0;
        let mut n_exact_both = 0;
        for j in 0..=9 {
            for (way, qty, exact_qty) in [
                (1, zero, exact_zero),
                (2, one, exact_one),
                (3, both, exact_both),
            ] {
                let mut can = 0i32;
                if way.is_set(0) {
                    can.set_bit(j);
                    can.set_bit(round(j));
                }
                if way.is_set(1) {
                    can.set_bit(j + 1);
                    can.set_bit(round(j) + 1);
                    can.set_bit(round(j + 1));
                    can.set_bit(round(j));
                }
                if can.is_set(cur) {
                    if can.is_set(10 + cur) {
                        n_both += qty;
                        if j < z_cur {
                            n_exact_both += qty;
                        } else if j == z_cur {
                            n_exact_both += exact_qty;
                        }
                    } else {
                        n_zero += qty;
                        if j < z_cur {
                            n_exact_zero += qty;
                        } else if j == z_cur {
                            n_exact_zero += exact_qty;
                        }
                    }
                } else if can.is_set(10 + cur) {
                    n_one += qty;
                    if j < z_cur {
                        n_exact_one += qty;
                    } else if j == z_cur {
                        n_exact_one += exact_qty;
                    }
                }
            }
        }
        zero = n_zero;
        one = n_one;
        both = n_both;
        exact_zero = n_exact_zero;
        exact_one = n_exact_one;
        exact_both = n_exact_both;
    }
    ans += exact_zero + exact_one + exact_both;
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
