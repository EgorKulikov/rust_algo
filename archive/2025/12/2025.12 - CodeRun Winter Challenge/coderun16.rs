//{"name":"day16","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    type Mod = ModIntF;
    const LIM: usize = 3001;
    let mut mem = Memoization2d::new(LIM, LIM, |mem, v, c| -> Mod {
        if v == 0 {
            if c == 0 {
                Mod::one()
            } else {
                Mod::zero()
            }
        } else {
            let mut res = mem.call(v - 1, c) * (v - 1);
            if c != 0 {
                res += mem.call(v - 1, c - 1);
            }
            res
        }
    });

    let t = input.read_size();
    for _ in 0..t {
        let n = input.read_size();
        let q = input.read_size();
        let l = input.read_size();
        let r = input.read_size();
        let b = input.read_size_vec(q).dec();
        let c = input.read_size_vec(q).dec();

        assert!(l <= r);

        let mut a = vec![None; n];
        let mut ai = vec![None; n];
        let mut bad = false;
        for i in 0..q {
            if a[b[i]].is_some() {
                bad = true;
                break;
            } else {
                a[b[i]] = Some(c[i]);
            }
            if ai[c[i]].is_some() {
                bad = true;
                break;
            } else {
                ai[c[i]] = Some(b[i]);
            }
        }
        if bad {
            out.print_line(0);
            continue;
        }
        let mut num_cycles = 0;
        let mut num_v = 0;
        let mut done = BitSet::new(n);
        for mut i in 0..n {
            if ai[i].is_none() {
                num_v += 1;
                loop {
                    done.set(i);
                    if let Some(to) = a[i] {
                        i = to;
                    } else {
                        break;
                    }
                }
            }
        }
        for i in 0..n {
            if !done[i] {
                num_cycles += 1;
                let mut j = i;
                while !done[j] {
                    done.set(j);
                    j = a[j].unwrap();
                }
            }
        }
        if num_cycles > r {
            out.print_line(0);
            continue;
        }
        let mut ans = Mod::zero();
        for i in l.saturating_sub(num_cycles)..=r - num_cycles {
            ans += mem.call(num_v, i);
        }
        out.print_line(ans);
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
