//{"name":"Rectangular Fibonacci Spiral","group":"Eolymp - Basecamp - Weekend Practice #20","url":"https://eolymp.com/en/compete/4pe1cne4r571j089budf65juq4/problem/3","interactive":false,"timeLimit":4000,"tests":[{"input":"1\n0 0\n1\n2 2\n","output":"0\n"},{"input":"5\n0 -6\n-2 -6\n-3 -6\n-4 -6\n5 5\n4\n0 0\n1 0\n2 0\n3 0\n","output":"4\n3\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::misc::time_tracker::TimeTracker;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let xy = input.read_long_pair_vec(n);
    let q = input.read_size();
    let queries = input.read_long_pair_vec(q);
    let mut tt = TimeTracker::new();

    let mut int_x = FxHashSet::default();
    let mut int_y = FxHashSet::default();
    for (x, y) in queries.copy_iter() {
        int_x.insert(x);
        int_y.insert(y);
    }

    let mut hor = Vec::new();
    let mut ver = Vec::new();
    for (mut x, mut y) in xy {
        let mut cur_len = 1;
        let mut prev_len = 0;
        let mut dx = 1;
        let mut dy = 0;
        while cur_len < 10_000_000_000 {
            let nx = x + (cur_len * dx);
            let ny = y + (cur_len * dy);
            match (dx, dy) {
                (1, 0) => {
                    if int_y.contains(&y) {
                        hor.push((y, x, 1));
                        hor.push((y, nx, -1));
                    }
                }
                (0, 1) => {
                    if int_x.contains(&x) {
                        ver.push((x, y, 1));
                        ver.push((x, ny, -1));
                    }
                }
                (-1, 0) => {
                    if int_y.contains(&y) {
                        hor.push((y, nx + 1, 1));
                        hor.push((y, x + 1, -1));
                    }
                }
                (0, -1) => {
                    if int_x.contains(&x) {
                        ver.push((x, ny + 1, 1));
                        ver.push((x, y + 1, -1));
                    }
                }
                _ => unreachable!(),
            }
            x = nx;
            y = ny;
            let next_len = cur_len + (prev_len);
            prev_len = cur_len;
            cur_len = next_len;
            (dx, dy) = (-dy, dx);
        }
    }
    tt.milestone("built hor and ver");
    hor.sort();
    ver.sort();
    tt.milestone("sorted hor and ver");
    let mut hor_sum = DefaultHashMap::new(Vec::new());
    let mut last_y = if hor.is_empty() { 0 } else { hor[0].0 };
    let mut last = i64::MIN;
    let mut sum = 0;
    let mut res = Vec::new();
    for (y, x, delta) in hor {
        if y != last_y {
            res.push((last, sum));
            hor_sum[last_y] = res;
            last_y = y;
            sum = 0;
            last = i64::MIN;
            res = Vec::new();
        }
        if x != last {
            res.push((last, sum));
        }
        sum += delta;
        last = x;
    }
    res.push((last, sum));
    hor_sum[last_y] = res;
    tt.milestone("built hor_sum");

    let mut ver_sum = DefaultHashMap::new(Vec::new());
    let mut last_x = if ver.is_empty() { 0 } else { ver[0].0 };
    let mut last = i64::MIN;
    let mut sum = 0;
    let mut res = Vec::new();
    for (x, y, delta) in ver {
        if x != last_x {
            res.push((last, sum));
            ver_sum[last_x] = res;
            last_x = x;
            sum = 0;
            last = i64::MIN;
            res = Vec::new();
        }
        if y != last {
            res.push((last, sum));
        }
        sum += delta;
        last = y;
    }
    res.push((last, sum));
    ver_sum[last_x] = res;

    tt.milestone("built ver_sum");

    for (x, y) in queries {
        let mut ans = 0;
        let pos_x = ver_sum[x].upper_bound(&(y, i32::MAX));
        if pos_x > 0 {
            ans += ver_sum[x][pos_x - 1].1;
        }
        let pos_y = hor_sum[y].upper_bound(&(x, i32::MAX));
        if pos_y > 0 {
            ans += hor_sum[y][pos_y - 1].1;
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
