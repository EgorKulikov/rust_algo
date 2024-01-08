//{"name":"cup15_g","group":"Manual","url":"","interactive":true,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"cup15_g"}}}

use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::slice_ext::reversed::ReversedSlice;
use algo_lib::io::input::Input;
use algo_lib::io::output::{err, Output};
use algo_lib::misc::random::random;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let alice_want = input.read_int();
    let mut a = input.read_int_vec(n);
    // err().print_line((n, alice_want));
    // err().print_line(&a);

    fn and(a: &mut Vec<i32>, pos: usize) {
        let res = a[pos] * a[pos + 1];
        a[pos] = res;
        a.remove(pos + 1);
    }

    fn xor(a: &mut Vec<i32>, pos: usize) {
        let res = a[pos] ^ a[pos + 1];
        a[pos] = res;
        a.remove(pos + 1);
    }

    fn read_move(input: &mut Input, a: &mut Vec<i32>) {
        let pos = input.read_size() - 1;
        let op = input.read_char();
        // let pos = random().next_bounds(1, a.len() - 1) - 1;
        // let op = if random().next(2) == 0 { '+' } else { '*' };
        // err().print_line((pos, op));
        if op == '+' {
            xor(a, pos);
        } else {
            and(a, pos);
        }
    }

    let last_want = if n % 2 == 0 {
        alice_want
    } else {
        1 - alice_want
    };
    if last_want == 0 {
        if n % 2 == 0 {
            out.print_line("Alice");
        } else {
            out.print_line("Bob");
        }
        loop {
            if a.len() == 2 {
                if a[0] == 0 || a[1] == 0 {
                    out.print_line((1, '*'));
                    and(&mut a, 0);
                } else {
                    out.print_line((1, '+'));
                    xor(&mut a, 0);
                }
                assert_eq!(a[0], 0);
                return;
            }
            if a.len() % 2 == 0 {
                out.print_line((1, '+'));
                xor(&mut a, 0);
            }
            read_move(input, &mut a);
        }
    }
    let mut total_bad = 0;
    if a[0] == 0 {
        total_bad += 1;
    }
    if a[n - 1] == 0 {
        total_bad += 1;
    }
    for (&a1, &a2) in a.consecutive_iter() {
        if a1 == 0 && a2 == 0 {
            total_bad += 1;
        }
    }
    for i in 0..n {
        if a[i] == 1 && (i == 0 || a[i - 1] == 0) {
            for j in i..n {
                if a[j] == 1 && (j == n - 1 || a[j + 1] == 0) {
                    if (j - i) % 2 == 1 {
                        total_bad += 1;
                    }
                    break;
                }
            }
        }
    }
    if total_bad > alice_want {
        if n % 2 == 1 {
            out.print_line("Alice");
        } else {
            out.print_line("Bob");
        }
        loop {
            if a.len() % 2 == 1 {
                if a[0] != 0 {
                    if a[1] == 0 {
                        out.print_line((1, '*'));
                        and(&mut a, 0);
                    } else {
                        out.print_line((1, '+'));
                        xor(&mut a, 0);
                    }
                } else if a.rev()[0] != 0 {
                    if a.rev()[1] == 0 {
                        out.print_line((a.len() - 1, '*'));
                        let pos = a.len() - 2;
                        and(&mut a, pos);
                    } else {
                        out.print_line((a.len() - 1, '+'));
                        let pos = a.len() - 2;
                        xor(&mut a, pos);
                    }
                } else {
                    out.print_line((1, '*'));
                    and(&mut a, 0);
                }
            }
            read_move(input, &mut a);
            if a.len() == 1 {
                assert_eq!(a[0], 0);
                return;
            }
        }
    }
    if n % 2 == 1 {
        out.print_line("Bob");
    } else {
        out.print_line("Alice");
    }
    loop {
        if a.len() % 2 == 0 {
            if a[0] == 0 {
                assert_eq!(a[1], 1);
                out.print_line((1, '+'));
                xor(&mut a, 0);
            } else if a.rev()[0] == 0 {
                assert_eq!(a.rev()[1], 1);
                out.print_line((a.len() - 1, '+'));
                let pos = a.len() - 2;
                xor(&mut a, pos);
            } else {
                let mut found = false;
                for i in 0..a.len() - 1 {
                    if a[i] == 0 && a[i + 1] == 0 {
                        out.print_line((i + 1, '*'));
                        and(&mut a, i);
                        found = true;
                        break;
                    }
                }
                if !found {
                    let mut num_ones = 0;
                    for i in a.indices() {
                        if a[i] == 0 {
                            if num_ones % 2 == 0 {
                                out.print_line((i - 1, '*'));
                                and(&mut a, i - 2);
                                found = true;
                                break;
                            }
                            num_ones = 0;
                        } else {
                            num_ones += 1;
                        }
                    }
                    if !found {
                        assert_eq!(num_ones % 2, 0);
                        out.print_line((a.len() - 1, '*'));
                        let pos = a.len() - 2;
                        and(&mut a, pos);
                    }
                }
            }
        }
        if a.len() == 1 {
            assert_eq!(a[0], 1);
            return;
        }
        read_move(input, &mut a);
    }
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
    // input.skip_whitespace();
    // input.peek().is_none()
    true
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    tester::stress_test(run, tester::check);
}
//END MAIN
