//{"name":"Prefix Max Xor","group":"CodeChef - START228A","url":"https://www.codechef.com/START228A/problems/PMXXOR","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n3\n1 1 1\n2\n1 2\n","output":"1\n499122179\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultTreeMap;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_unsigned_vec(n);

    let mut qty = DefaultTreeMap::new(0);
    for x in a {
        qty[x] += 1;
    }
    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(n + 1);
    let mut q0 = vec![Mod::one(); 30];
    let mut q1 = vec![Mod::zero(); 30];
    let mut total = 0;
    let q = qty.into_iter().collect::<Vec<_>>().reversed();
    for (val, q) in q {
        let mut not_take = Mod::zero();
        let mut take = Mod::zero();
        for i in 0..=q {
            if i % 2 == 0 {
                not_take += c.comb_with_rep(total, q - i);
            } else {
                take += c.comb_with_rep(total, q - i);
            }
        }
        for i in 0..30 {
            if val.is_set(i) {
                let n0 = q0[i] * not_take + q1[i] * take;
                let n1 = q0[i] * take + q1[i] * not_take;
                q0[i] = n0;
                q1[i] = n1;
            }
        }
        total += q;
    }
    let mut ans = Mod::zero();
    for i in 0..30 {
        if q0[i] + q1[i] == Mod::zero() {
            panic!();
        }
        ans += q1[i] / (q0[i] + q1[i]) * (1 << i);
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
