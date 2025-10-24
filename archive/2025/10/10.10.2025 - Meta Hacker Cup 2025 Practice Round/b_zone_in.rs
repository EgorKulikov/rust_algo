//{"name":"B: Zone In","group":"Meta Coding Competitions - Meta Hacker Cup 2025 Practice Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2025/practice-round/problems/B","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n5 5 1\n.....\n.....\n.....\n.....\n.....\n7 7 2\n.......\n.......\n..#.#..\n..#....\n.......\n.......\n.......\n7 7 1\n.......\n.......\n..#.#..\n..#....\n.......\n.......\n.......\n9 20 1\n....................\n....####....####....\n..##....#..#....##..\n.#.......##.......#.\n.#......#..#......#.\n.#......#..#......#.\n..#....#....#....#..\n..####.......####...\n....................\n10 20 3\n....................\n....................\n..........####......\n.........#..........\n.........#..........\n.......######.......\n.........#..........\n.........#..........\n.........#..........\n.........#..........\n","output":"Case #1: 9\nCase #2: 0\nCase #3: 10\nCase #4: 12\nCase #5: 8\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"zone_in_.*input[.]txt"},"output":{"type":"file","fileName":"zone_in_output.txt","pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::run_parallel::run_parallel;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let r = input.read_size();
    let c = input.read_size();
    let s = input.read_size();
    let mut grid = input.read_char_table(r, c);
    drop(input);

    let mut cur = Vec::new();
    let mut next = Vec::new();
    for (i, j) in grid.indices() {
        if grid[(i, j)] == b'#' {
            cur.push((i, j));
        } else if i == 0 || i == r - 1 || j == 0 || j == c - 1 {
            next.push((i, j));
            grid[(i, j)] = b'#';
        }
    }
    for _ in 0..s {
        for (x, y) in cur {
            for (nx, ny) in D4::iter(x, y, r, c) {
                if grid[(nx, ny)] == b'.' {
                    grid[(nx, ny)] = b'#';
                    next.push((nx, ny));
                }
            }
        }
        cur = next;
        next = Vec::new();
    }
    let mut ans = 0;
    for (i, j) in grid.indices() {
        if grid[(i, j)] == b'.' {
            let mut queue = vec![(i, j)];
            let mut cur = 0;
            grid[(i, j)] = b'#';
            while let Some((x, y)) = queue.pop() {
                cur += 1;
                for (nx, ny) in D4::iter(x, y, r, c) {
                    if grid[(nx, ny)] == b'.' {
                        grid[(nx, ny)] = b'#';
                        queue.push((nx, ny));
                    }
                }
            }
            ans.maxim(cur);
        }
    }
    out.print_line((format!("Case #{}:", test_case), ans));
}

pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    let pre_calc = ();
    let is_exhausted = run_parallel(input, &mut output, true, pre_calc, solve);
    output.flush();
    is_exhausted
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let paths = std::fs::read_dir(".").unwrap();
    let mut result = None;
    let mut last_accessed = None;
    let re = regex::Regex::new("zone_in_.*input[.]txt").unwrap();
    for path in paths {
        let path = path.unwrap();
        let cur_accessed = path.metadata().unwrap().accessed().unwrap();
        let path = path.path();
        let cur_name = path.file_name().unwrap().to_str().unwrap();
        if re.is_match(cur_name) {
            if last_accessed.is_none() || cur_accessed > last_accessed.unwrap() {
                result = Some(cur_name.to_string());
                last_accessed = Some(cur_accessed);
            }
        }
    }
    let in_file = std::fs::File::open(result.unwrap()).unwrap();
    let input = algo_lib::io::input::Input::file(in_file);
    let out_file = std::fs::File::create("zone_in_output.txt").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);
    run(input, output);
}
