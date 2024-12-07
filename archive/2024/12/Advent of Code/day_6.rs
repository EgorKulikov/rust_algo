//{"name":"day_6","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_6"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::isize::{ISize, IsizeOps};
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut data = Vec::new();

    while !input.is_empty() {
        data.push(input.read_line());
    }

    let mut r = 0;
    let mut c = 0;
    for i in data.indices() {
        for j in data[i].indices() {
            if data[i][j] == b'^' {
                r = i as isize;
                c = j as isize;
            }
        }
    }

    // part 1
    {
        let mut r = ISize(r);
        let mut c = ISize(c);
        let mut dr = -1;
        let mut dc = 0;
        let mut pos = FxHashSet::default();
        loop {
            pos.insert((r, c));
            let nr = r + dr;
            let nc = c + dc;

            if !data.fits(nr) || !data[nr].fits(nc) {
                break;
            }
            if data[nr][nc] == b'#' {
                (dr, dc) = (dc, -dr);
            } else {
                (r, c) = (nr, nc);
            }
        }
        out.print_line(pos.len());
    }

    // part 2
    {
        let mut ans = 0;
        for i in data.indices() {
            for j in data[i].indices() {
                if data[i][j] == b'.' {
                    data[i][j] = b'#';
                    let mut cycle = false;
                    let mut r = r;
                    let mut c = c;
                    let mut dr = -1;
                    let mut dc = 0;
                    let mut pos = DefaultHashMap::<_, usize>::default();
                    loop {
                        pos[(r, c)] += 1;
                        if pos[(r, c)] >= 5 {
                            cycle = true;
                            break;
                        }
                        let nr = r + dr;
                        let nc = c + dc;

                        if nr < 0
                            || nc < 0
                            || nr >= data.len() as isize
                            || nc >= data[0].len() as isize
                        {
                            break;
                        }
                        if data[nr as usize][nc as usize] == b'#' {
                            (dr, dc) = (dc, -dr);
                        } else {
                            (r, c) = (nr, nc);
                        }
                    }
                    if cycle {
                        ans += 1;
                    }
                    data[i][j] = b'.';
                }
            }
        }
        out.print_line(ans);
    }
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
