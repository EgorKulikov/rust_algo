//{"name":"level_5","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::rational::Rational;
use algo_lib::scan;
use std::iter::repeat_n;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    scan!(input, "@,@ @", x: i64, y: i64, tl: i64);
    scan!(input, "@,@", ax: i64, ay: i64);

    fn go(pos: i64) -> Vec<i64> {
        let mut ans = vec![1; pos.abs() as usize + 1];
        ans[0] = 0;
        ans[Back(0)] = 0;
        for i in 0..5.min(ans.len() - 1) {
            ans[i + 1].maxim(5 - i as i64);
            ans[Back(i)].maxim(5 - i as i64);
        }
        if pos < 0 {
            ans.iter_mut().for_each(|x| *x = -(*x));
        }
        ans
    }
    fn build_pos(x_ans: &[i64], sx: i64) -> Vec<Rational<i64>> {
        let mut px = vec![Rational::new(sx, 1)];
        for i in x_ans.indices() {
            let cx = px[Back(0)];
            if x_ans[i] == 0 {
                px.push(cx);
            } else if x_ans[i] > 0 {
                for _ in 0..x_ans[i] {
                    px.push(px[Back(0)] + Rational::new(1, x_ans[i]));
                }
            } else {
                for _ in 0..-x_ans[i] {
                    px.push(px[Back(0)] - Rational::new(1, -x_ans[i]));
                }
            }
        }
        px
    }
    fn path(
        x0: i64,
        y0: i64,
        x1: i64,
        y1: i64,
    ) -> (Vec<i64>, Vec<i64>, Vec<(Rational<i64>, Rational<i64>)>, i64) {
        let mut x_ans = go(x1 - x0);
        let mut y_ans = go(y1 - y0);
        let tx = x_ans
            .copy_map(|x| if x != 0 { x.abs() } else { 1 })
            .sum::<i64>();
        let ty = y_ans
            .copy_map(|y| if y != 0 { y.abs() } else { 1 })
            .sum::<i64>();
        if tx < ty {
            x_ans.extend(repeat_n(0, (ty - tx) as usize));
        }
        if ty < tx {
            y_ans.extend(repeat_n(0, (tx - ty) as usize));
        }
        let px = build_pos(&x_ans, x0);
        let py = build_pos(&y_ans, y0);
        assert_eq!(px.len(), py.len());
        assert_eq!(px[Back(0)], Rational::new(x1, 1));
        assert_eq!(py[Back(0)], Rational::new(y1, 1));
        (
            x_ans,
            y_ans,
            px.iter().cloned().zip(py.iter().cloned()).collect(),
            tx.max(ty),
        )
    }
    let check = |path: &[(Rational<i64>, Rational<i64>)]| {
        for i in path.indices() {
            let dx = (path[i].0) - Rational::new(ax, 1);
            let dy = (path[i].1) - Rational::new(ay, 1);
            if dx.abs() < Rational::new(3, 1) && dy.abs() < Rational::new(3, 1) {
                return false;
            }
        }
        true
    };

    let mut ans_x = Vec::new();
    let mut ans_y = Vec::new();
    let mut best = i64::MAX;
    let mut process = |pts: &[(i64, i64)]| {
        let mut len = 0;
        let mut px = Vec::new();
        let mut py = Vec::new();
        for i in 1..pts.len() {
            let (cpx, cpy, path, cl) = path(pts[i - 1].0, pts[i - 1].1, pts[i].0, pts[i].1);
            if !check(&path) {
                return;
            }
            len += cl;
            px.extend(cpx);
            py.extend(cpy);
        }
        px.push(0);
        py.push(0);
        len += 1;
        if len < best {
            best = len;
            ans_x = px;
            ans_y = py;
        }
    };
    process(&[(0, 0), (x, y)]);
    for (x0, y0) in &[
        (ax + 3, ay + 3),
        (ax - 3, ay + 3),
        (ax + 3, ay - 3),
        (ax - 3, ay - 3),
    ] {
        process(&[(0, 0), (*x0, *y0), (x, y)]);
        for (x1, y1) in &[
            (ax + 3, ay + 3),
            (ax - 3, ay + 3),
            (ax + 3, ay - 3),
            (ax - 3, ay - 3),
        ] {
            process(&[(0, 0), (*x0, *y0), (*x1, *y1), (x, y)]);
        }
    }
    assert!(best <= tl);
    out.print_line(ans_x);
    out.print_line(ans_y);
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
