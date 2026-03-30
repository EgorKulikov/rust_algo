//{"name":"Closed Subsets","group":"Eolymp - Basecamp - Weekend Practice #21 ","url":"https://eolymp.com/en/compete/idp1eoul596rpbbmp0mfbbo94g/problem/4","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n1 2 3 4 5\n","output":"9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::misc::time_tracker::TimeTracker;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::bit_ops::{BitIter, BitOps};
use algo_lib::numbers::num_utils::powers;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut tt = TimeTracker::new();

    let limit = n.highest_bit() + 1;
    let p3 = 3.power(limit);
    let p = powers(3, limit);
    let mut qty = vec![0i32; p3];
    for i in 0..n {
        let mut id = 0;
        for j in 0..limit {
            if a[i].is_set(j) {
                id += p[j];
            }
        }
        qty[id] += 1;
    }
    tt.milestone("base");
    let id = Vec::with_gen(1 << limit, |i| {
        let mut id = 0;
        for j in 0..limit {
            if i.is_set(j) {
                id += p[j];
            }
        }
        id
    });
    for i in 0..limit {
        // for k in (0..p3).step_by(p[i] * 3) {
        for kk in usize::iter_all(limit - i - 1) {
            let k = p[i] * 3 * id[kk];
            for l in k..k + p[i] {
                qty[l + 2 * p[i]] = qty[l] + qty[l + p[i]];
            }
        }
    }
    tt.milestone("fill");

    type Mod = ModIntF;
    let mut ans = Mod::new(0);
    let p2 = powers(Mod::new(2), n + 1);
    for i in usize::iter_all(limit) {
        for j in BitIter::new(i) {
            if j == i {
                ans += p2[qty[id[i]] as usize] - 1;
            } else {
                let qmin = qty[id[i]];
                let qmax = qty[id[j]];
                let rem = qty[2 * id[i] - id[j]] - qmin - qmax;
                ans += (p2[qmin as usize] - 1) * (p2[qmax as usize] - 1) * p2[rem as usize];
            }
        }
    }
    tt.milestone("calc");
    out.print_line(ans);
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
