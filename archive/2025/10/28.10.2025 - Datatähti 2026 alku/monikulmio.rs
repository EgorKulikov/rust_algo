//{"name":"Monikulmio","group":"CSES - Datatähti 2026 alku","url":"https://cses.fi/637/task/B","interactive":false,"timeLimit":1000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::when;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let xy = input.read_size_pair_vec(k).dec();

    let mut ans = Arr2d::new(n, m, b'.');
    let mut first_sign = 0;
    let mut sign = 0;
    for i in 0..k {
        let (x, y) = xy[i];
        let (x1, y1) = xy[(i + 1) % k];
        ans[(x, y)] = b'*';
        let len = x.abs_diff(x1).max(y.abs_diff(y1));
        let dx = (x1 as isize - x as isize) / (len as isize);
        let dy = (y1 as isize - y as isize) / (len as isize);
        let c = when! {
            dx == 0 => b'=',
            dy == 0 => b'|',
            dx == dy => b'\\',
            dx == -dy => b'/',
        };
        let mut cx = x as isize;
        let mut cy = y as isize;
        let mut start = DefaultHashMap::default();
        let cur_sign = if x1 > x {
            1
        } else if x1 < x {
            -1
        } else {
            0
        };
        if i == 0 || sign != 0 && sign == cur_sign {
            start[x] = y;
        }
        for _ in 1..len {
            cx += dx;
            cy += dy;
            ans[(cx as usize, cy as usize)] = c;
            if cur_sign != 0 {
                start[cx as usize] = cy as usize;
            }
        }
        if first_sign == 0 {
            first_sign = cur_sign;
        }
        if cur_sign != 0 {
            sign = cur_sign;
        }
        if i == k - 1 && sign != first_sign {
            start[x1] = y1;
        }
        for (x, y) in start {
            for y in y..m {
                if ans[(x, y)] == b'.' {
                    ans[(x, y)] = b'#';
                } else if ans[(x, y)] == b'#' {
                    ans[(x, y)] = b'.';
                }
            }
        }
    }
    out.print_table(&ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
