//{"name":"ucup19_e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ucup19_e"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_5d::Memoization5d;
use algo_lib::misc::recursive_function::Callable5;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_u64();

    let mut mem = Memoization5d::new(
        61,
        2,
        2,
        2,
        2,
        |mem,
         bit,
         carry_a_plus_b,
         carry_n_plus_b,
         a_non_zero,
         b_non_zero|
         -> Option<(usize, usize)> {
            if bit == 60 {
                if carry_a_plus_b != carry_n_plus_b || a_non_zero == 0 || b_non_zero == 0 {
                    None
                } else {
                    Some((0, 0))
                }
            } else {
                let bit_n = ((n >> bit) & 1) as usize;
                for bit_a in 0..2 {
                    for bit_b in 0..2 {
                        let bit_a_plub_b = bit_a ^ bit_b ^ carry_a_plus_b;
                        let new_carry_a_plus_b = (bit_a + bit_b + carry_a_plus_b) >> 1;
                        let bit_n_plus_b = bit_n ^ bit_b ^ carry_n_plus_b;
                        let new_carry_n_plus_b = (bit_n + bit_b + carry_n_plus_b) >> 1;
                        if bit_a_plub_b == (bit_a ^ bit_n_plus_b) {
                            if let Some((a, b)) = mem.call(
                                bit + 1,
                                new_carry_a_plus_b,
                                new_carry_n_plus_b,
                                a_non_zero | bit_a,
                                b_non_zero | bit_b,
                            ) {
                                return Some((a * 2 + bit_a, b * 2 + bit_b));
                            }
                        }
                    }
                }
                None
            }
        },
    );
    out.print_line(mem.call(0, 0, 0, 0, 0));
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
