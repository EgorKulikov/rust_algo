//{"name":"day_15","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_15"}}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut map = Vec::new();
    loop {
        let s = input.read_line();
        if s.is_empty() {
            break;
        }
        map.push(s);
    }
    let mut path = Vec::new();
    while !input.is_empty() {
        path.push(input.read_line());
    }

    //part 1
    {
        let mut map = map.clone();
        let mut row = 0;
        let mut col = 0;
        'outer: for i in map.indices() {
            for j in map[i].indices() {
                if map[i][j] == b'@' {
                    row = i;
                    col = j;
                    break 'outer;
                }
            }
        }
        for p in &path {
            for c in p.iter() {
                match c {
                    b'^' => {
                        let mut nr = row - 1;
                        while map[nr][col] == b'O' {
                            nr -= 1;
                        }
                        if map[nr][col] == b'.' {
                            map[nr][col] = b'O';
                            map[row - 1][col] = b'@';
                            map[row][col] = b'.';
                            row -= 1;
                        }
                    }
                    b'v' => {
                        let mut nr = row + 1;
                        while map[nr][col] == b'O' {
                            nr += 1;
                        }
                        if map[nr][col] == b'.' {
                            map[nr][col] = b'O';
                            map[row + 1][col] = b'@';
                            map[row][col] = b'.';
                            row += 1;
                        }
                    }
                    b'<' => {
                        let mut nc = col - 1;
                        while map[row][nc] == b'O' {
                            nc -= 1;
                        }
                        if map[row][nc] == b'.' {
                            map[row][nc] = b'O';
                            map[row][col - 1] = b'@';
                            map[row][col] = b'.';
                            col -= 1;
                        }
                    }
                    b'>' => {
                        let mut nc = col + 1;
                        while map[row][nc] == b'O' {
                            nc += 1;
                        }
                        if map[row][nc] == b'.' {
                            map[row][nc] = b'O';
                            map[row][col + 1] = b'@';
                            map[row][col] = b'.';
                            col += 1;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        let mut ans = 0;
        for i in map.indices() {
            for j in map[i].indices() {
                if map[i][j] == b'O' {
                    ans += i * 100 + j;
                }
            }
        }
        out.print_line(ans);
    }

    //part 2
    {
        let mut map = Arr2d::gen(map.len(), map[0].len() * 2, |i, j| match map[i][j / 2] {
            b'#' => b'#',
            b'.' => b'.',
            b'@' => {
                if j % 2 == 0 {
                    b'@'
                } else {
                    b'.'
                }
            }
            b'O' => {
                if j % 2 == 0 {
                    b'['
                } else {
                    b']'
                }
            }
            _ => unreachable!(),
        });
        let mut row = 0;
        let mut col = 0;
        'outer: for i in map.rows() {
            for j in map.cols() {
                if map[i][j] == b'@' {
                    row = i;
                    col = j;
                    break 'outer;
                }
            }
        }
        let mut done = Arr2d::new(map.d1(), map.d2(), false);
        for p in &path {
            for c in p.iter() {
                match c {
                    b'^' => {
                        done.fill(false);
                        let mut cells = vec![(row, col)];
                        let mut at = 0;
                        let mut ok = true;
                        while at < cells.len() {
                            let (r, c) = cells[at];
                            match map[r - 1][c] {
                                b'#' => {
                                    ok = false;
                                    break;
                                }
                                b'[' => {
                                    if !done[r - 1][c] {
                                        done[r - 1][c] = true;
                                        cells.push((r - 1, c));
                                    }
                                    if !done[r - 1][c + 1] {
                                        done[r - 1][c + 1] = true;
                                        cells.push((r - 1, c + 1));
                                    }
                                }
                                b']' => {
                                    if !done[r - 1][c] {
                                        done[r - 1][c] = true;
                                        cells.push((r - 1, c));
                                    }
                                    if !done[r - 1][c - 1] {
                                        done[r - 1][c - 1] = true;
                                        cells.push((r - 1, c - 1));
                                    }
                                }
                                _ => {}
                            }
                            at += 1;
                        }
                        if ok {
                            for (r, c) in cells.iter_rev() {
                                map[r - 1][c] = map[r][c];
                                map[r][c] = b'.';
                            }
                            row -= 1;
                        }
                    }
                    b'v' => {
                        done.fill(false);
                        let mut cells = vec![(row, col)];
                        let mut at = 0;
                        let mut ok = true;
                        while at < cells.len() {
                            let (r, c) = cells[at];
                            match map[r + 1][c] {
                                b'#' => {
                                    ok = false;
                                    break;
                                }
                                b'[' => {
                                    if !done[r + 1][c] {
                                        done[r + 1][c] = true;
                                        cells.push((r + 1, c));
                                    }
                                    if !done[r + 1][c + 1] {
                                        done[r + 1][c + 1] = true;
                                        cells.push((r + 1, c + 1));
                                    }
                                }
                                b']' => {
                                    if !done[r + 1][c] {
                                        done[r + 1][c] = true;
                                        cells.push((r + 1, c));
                                    }
                                    if !done[r + 1][c - 1] {
                                        done[r + 1][c - 1] = true;
                                        cells.push((r + 1, c - 1));
                                    }
                                }
                                _ => {}
                            }
                            at += 1;
                        }
                        if ok {
                            for (r, c) in cells.iter_rev() {
                                map[r + 1][c] = map[r][c];
                                map[r][c] = b'.';
                            }
                            row += 1;
                        }
                    }
                    b'<' => {
                        let mut nc = col - 1;
                        while map[row][nc] == b'[' || map[row][nc] == b']' {
                            nc -= 1;
                        }
                        if map[row][nc] == b'.' {
                            for c in nc..col {
                                map[row][c] = map[row][c + 1];
                                map[row][c + 1] = b'.';
                            }
                            col -= 1;
                        }
                    }
                    b'>' => {
                        let mut nc = col + 1;
                        while map[row][nc] == b'[' || map[row][nc] == b']' {
                            nc += 1;
                        }
                        if map[row][nc] == b'.' {
                            for c in (col + 1..=nc).rev() {
                                map[row][c] = map[row][c - 1];
                                map[row][c - 1] = b'.';
                            }
                            col += 1;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        let mut ans = 0;
        for i in map.rows() {
            for j in map.cols() {
                if map[i][j] == b'[' {
                    ans += i * 100 + j;
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
