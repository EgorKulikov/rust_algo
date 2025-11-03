//{"name":"vampire_escape_part_2","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::real::{Real, RealReader};
use std::io::Write;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let h = input.read_real();
    let w = input.read_real();
    let n = input.read_size();
    let l = input.read_real();
    let spots = input.read_vec::<(Real, Real, Real, Real, Real)>(n);

    let mut poi = vec![Real::zero(), l];
    let mut add = |dx: Real, v: Real| {
        if v == Real::zero() {
            return;
        }
        let val = dx / v;
        if val < Real::zero() || val > l {
            return;
        }
        poi.push(val);
    };
    for i in 0..n {
        let (xi, yi, vxi, vyi, ri) = spots[i];
        add(-xi, vxi);
        add(w - xi, vxi);
        add(-yi, vyi);
        add(h - yi, vyi);
        add(ri - xi, vxi);
        add(w - ri - xi, vxi);
        for j in 0..i {
            let (xj, _, vxj, _, rj) = spots[j];
            add(xj - xi - ri - rj, vxi - vxj);
            add(xi - xj - ri - rj, vxj - vxi);
        }
    }
    poi.sort();
    poi.dedup();
    let mut ans = Real::zero();
    for i in 0..poi.len() - 1 {
        let mid = (poi[i] + poi[i + 1]) / 2;
        let mut segments = Vec::new();
        for j in 0..n {
            let (xi, yi, vxi, vyi, ri) = spots[j];
            let x = xi + vxi * mid;
            let y = yi + vyi * mid;
            if x >= Real::zero() && x <= w && y >= Real::zero() && y <= h {
                segments.push((x - ri, x + ri));
            }
        }
        segments.sort();
        let mut left = Real::zero();
        let mut found = false;
        for (from, to) in segments {
            if from > left {
                found = true;
                break;
            }
            left.maxim(to);
        }
        if left < w {
            found = true;
        }
        if found {
            ans += poi[i + 1] - poi[i];
        }
    }
    write!(out, "Case #{}: ", test_case).unwrap();
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
