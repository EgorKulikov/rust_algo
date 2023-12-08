//{"name":"L. Playing with stones","group":"CPython.uz - CPython Programming Contest #3","url":"https://cpython.uz/competitions/contests/contest/326/problem/L","interactive":false,"timeLimit":1000,"tests":[{"input":"3 3 4 4 5 5\n","output":"1\n"},{"input":"4 4 8 8 12 12\n","output":"0\n"},{"input":"1 10 1 10 1 10\n","output":"580000005\n"},{"input":"5 15 2 9 35 42\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LPlayingWithStones"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::number_iterator::iterate_with_base;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let l1 = input.read_int();
    let r1 = input.read_int();
    let l2 = input.read_int();
    let r2 = input.read_int();
    let l3 = input.read_int();
    let r3 = input.read_int();

    let mut bad = 0i64;

    let i1 = iterate_with_base(l1.min(r1), r1.max(l1), 2);
    let i2 = iterate_with_base(l2.min(r2), r2.max(l2), 2);
    let i3 = iterate_with_base(l3.min(r3), r3.max(l3), 2);
    for &(p1, l1, _) in &i1 {
        for &(p2, l2, _) in &i2 {
            for &(p3, l3, _) in &i3 {
                let mut ways = 1i64;
                for i in 0..30 {
                    let mut free = 0;
                    let mut val = 0;
                    if i < l1 {
                        free += 1;
                    } else {
                        val ^= (p1 >> (i - l1)) & 1;
                    }
                    if i < l2 {
                        free += 1;
                    } else {
                        val ^= (p2 >> (i - l2)) & 1;
                    }
                    if i < l3 {
                        free += 1;
                    } else {
                        val ^= (p3 >> (i - l3)) & 1;
                    }
                    if free == 0 && val == 1 {
                        ways = 0;
                        break;
                    } else if free > 1 {
                        ways *= 1 << (free - 1);
                    }
                }
                bad += ways;
            }
        }
    }
    type Mod = ModInt7;
    let bad = Mod::new_from_wide(bad);
    let all = Mod::new(r1 - l1 + 1) * Mod::new(r2 - l2 + 1) * Mod::new(r3 - l3 + 1);
    let good = all - bad;
    out.print_line(good / all);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
    tester::stress_test(run, tester::check);
}
//END MAIN
