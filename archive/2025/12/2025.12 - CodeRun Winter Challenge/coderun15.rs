//{"name":"coderun15","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_utils::powers;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();
    let t = input.read_str();

    #[derive(Clone)]
    enum Comp {
        Less(usize),
        More(usize),
        Equal,
    }
    impl Comp {
        fn trunc(self, lim: usize) -> Self {
            match self {
                Comp::Less(pos) => {
                    if pos >= lim {
                        Comp::Equal
                    } else {
                        Comp::Less(pos)
                    }
                }
                Comp::More(pos) => {
                    if pos >= lim {
                        Comp::Equal
                    } else {
                        Comp::More(pos)
                    }
                }
                Comp::Equal => Comp::Equal,
            }
        }
    }
    type Mod = ModIntF;
    let mut cmp = Memoization2d::new(s.len() + 1, t.len() + 1, |mem, start_s, pos_t| -> Comp {
        if s.len() == start_s {
            Comp::Equal
        } else if pos_t == t.len() || t[pos_t] < s[start_s] {
            Comp::More(pos_t)
        } else if t[pos_t] > s[start_s] {
            Comp::Less(pos_t)
        } else {
            mem.call(start_s + 1, pos_t + 1)
        }
    });
    let pw = powers(Mod::from(2), s.len() + 1);
    let mut mem = Memoization2d::new(
        s.len() + 1,
        t.len() + 1,
        |mem, pos_s, pos_t| -> (Mod, Mod) {
            if pos_t == t.len() {
                (Mod::zero(), Mod::zero())
            } else if pos_s == 0 {
                (Mod::one(), Mod::from(pos_t))
            } else {
                let mut count = Mod::zero();
                let mut res = Mod::zero();
                for i in 0..pos_s {
                    match cmp.call(i, pos_t).trunc(pos_t + pos_s - i) {
                        Comp::Less(pos) => {
                            count += pw[i.saturating_sub(1)];
                            res += pw[i.saturating_sub(1)] * pos;
                        }
                        Comp::More(_) => {}
                        Comp::Equal => {
                            let (call_count, call_res) = mem.call(i, pos_t + pos_s - i);
                            count += call_count;
                            res += call_res;
                        }
                    }
                }
                (count, res)
            }
        },
    );
    let mut ans = Mod::zero();
    for i in t.indices() {
        let (count, res) = mem.call(s.len(), i);
        ans += count * t.len() - res;
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
