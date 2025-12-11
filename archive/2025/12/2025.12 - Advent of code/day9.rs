//{"name":"day9","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::cur_next::cur_next;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::str_scan;
use algo_lib::string::str::StrReader;
use std::collections::VecDeque;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut pts = Vec::new();
    for s in input.read_lines() {
        str_scan!(&s, "@,@", x: i64, y: i64);
        pts.push((x, y));
    }

    let mut ans = 0;
    for i in pts.indices() {
        for j in 0..i {
            let dx = pts[i].0 - pts[j].0;
            let dy = pts[i].1 - pts[j].1;
            ans.maxim((dx.abs() + 1) * (dy.abs() + 1));
        }
    }
    out.print_line(ans);
    let (x, y) = pts.clone().detuple();
    let mut x = x.sorted();
    x.dedup();
    let mut y = y.sorted();
    y.dedup();
    let mut grid = Arr2d::new(x.len(), y.len(), b'.');
    for (i, j) in cur_next(pts.len()) {
        let x0 = x.lower_bound(&pts[i].0);
        let y0 = y.lower_bound(&pts[i].1);
        grid[(x0, y0)] = b'R';
        let x1 = x.lower_bound(&pts[j].0);
        let y1 = y.lower_bound(&pts[j].1);
        for x in x0.min(x1)..=x0.max(x1) {
            for y in y0.min(y1)..=y0.max(y1) {
                if grid[(x, y)] == b'.' {
                    grid[(x, y)] = b'G';
                }
            }
        }
    }
    for (i, j) in grid.indices() {
        if grid[(i, j)] != b'.' {
            continue;
        }
        let mut queue = VecDeque::new();
        let mut has_edge = false;
        queue.push_back((i, j));
        grid[(i, j)] = b' ';
        for i in 0.. {
            if i == queue.len() {
                break;
            }
            let (r, c) = queue[i];
            if r == 0 || c == 0 || r + 1 == x.len() || c + 1 == y.len() {
                has_edge = true;
            }
            for (nr, nc) in D4::iter(r, c, x.len(), y.len()) {
                if grid[(nr, nc)] == b'.' {
                    grid[(nr, nc)] = b' ';
                    queue.push_back((nr, nc));
                }
            }
        }
        if !has_edge {
            for (r, c) in queue {
                grid[(r, c)] = b'G';
            }
        }
    }
    let mut ans2 = 0;
    for i in pts.indices() {
        for j in 0..i {
            let dx = pts[i].0 - pts[j].0;
            let dy = pts[i].1 - pts[j].1;
            let x0 = x.lower_bound(&pts[i].0);
            let y0 = y.lower_bound(&pts[i].1);
            let x1 = x.lower_bound(&pts[j].0);
            let y1 = y.lower_bound(&pts[j].1);
            let mut good = true;
            'outer: for x in x0.min(x1)..=x0.max(x1) {
                for y in y0.min(y1)..=y0.max(y1) {
                    if grid[(x, y)] == b' ' {
                        good = false;
                        break 'outer;
                    }
                }
            }
            if good {
                ans2.maxim((dx.abs() + 1) * (dy.abs() + 1));
            }
        }
    }
    out.print_line(ans2);
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
