//{"name":"B. Palindrome and Permutation","group":"Universal Cup - GP of India","url":"https://contest.ucup.ac/contest/3516/problem/17407","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 2 3\n4\n2 3 3 2\n6\n4 6 4 6 4 6\n10\n1 5 2 10 2 6 3 1 10 2\n","output":"8\n0\n5040\n31363200\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_utils::{factorial, PartialSums};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    let mut dsu = DSU::new(n);
    for i in 0..n / 2 {
        if a[i] != a[Back(i)] {
            dsu.union(a[i], a[Back(i)]);
        }
    }
    let mut edges = vec![0; n];
    for i in 0..n / 2 {
        if a[i] != a[Back(i)] {
            edges[dsu.find(a[i])] += 1;
        }
    }
    type Mod = ModIntF;
    let mut take = 0;
    let mut parts = Vec::new();
    for i in 0..n {
        if dsu.find(i) == i {
            if edges[i] > dsu.size(i) {
                out.print_line(factorial::<Mod>(n + 1));
                return;
            }
            if edges[i] == dsu.size(i) {
                take += dsu.size(i);
            } else {
                parts.push(dsu.size(i));
            }
        }
    }
    let c = Combinations::<Mod>::new(n + 1);
    let s = parts.partial_sums();
    let mut mem = Memoization2d::new(
        parts.len() + 1,
        n + 1,
        |mem, step, pos| -> (Mod, Mod, Mod, Mod) {
            if step == parts.len() {
                (Mod::from(pos), Mod::new(0), Mod::new(1), Mod::new(0))
            } else {
                let p = parts[step];
                let s = s[step] + take;
                let mut base_qty = Mod::new(0);
                let mut base = mem.call(step + 1, pos + p).0 * c.c(pos + p - 1, p);
                base_qty += mem.call(step + 1, pos + p).2 * c.c(pos + p - 1, p);
                if p == 1 {
                    base += mem.call(step + 1, pos).0 * (s - pos + 1);
                    base_qty += mem.call(step + 1, pos).2 * (s - pos + 1);
                } else if pos > 0 {
                    base +=
                        mem.call(step + 1, pos + p - 1).0 * c.c(pos + p - 2, p - 1) * (s - pos + 1);
                    base_qty +=
                        mem.call(step + 1, pos + p - 1).2 * c.c(pos + p - 2, p - 1) * (s - pos + 1);
                }
                let mut add = Mod::new(0);
                let mut add_qty = Mod::new(0);
                if p > 1 {
                    add +=
                        mem.call(step + 1, pos + p - 1).0 * c.c(pos + p - 2, p - 2) * (s - pos + 1);
                    add_qty +=
                        mem.call(step + 1, pos + p - 1).2 * c.c(pos + p - 2, p - 2) * (s - pos + 1);
                }
                base *= c.fact(p);
                add *= c.fact(p);
                base_qty *= c.fact(p);
                add_qty *= c.fact(p);
                if pos < s {
                    add += mem.call(step, pos + 1).1;
                    add_qty += mem.call(step, pos + 1).3;
                }
                assert_eq!(base_qty + add_qty, c.fact(n) / c.fact(s));
                (base + add, add, base_qty + add_qty, add_qty)
            }
        },
    );

    let ans = mem.call(0, take);
    assert_eq!(ans.2, c.fact(n) / c.fact(take));
    out.print_line(ans.0 * c.fact(take));
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
