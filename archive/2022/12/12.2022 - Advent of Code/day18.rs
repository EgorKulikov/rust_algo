//{"name":"day18","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day18"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out_line, scan};
use std::collections::{HashSet, VecDeque};

fn solve(input: &mut Input, _test_case: usize) {
    let mut set = HashSet::new();

    let mut mx = 0;
    let mut my = 0;
    let mut mz = 0;
    while !input.is_exhausted() {
        scan!(input, "@,@,@", x: i32, y: i32, z: i32);
        set.insert((x, y, z));
        mx.maxim(x);
        my.maxim(y);
        mz.maxim(z);
        input.skip_whitespace();
    }

    eprintln!("Max: {}, {}, {}", mx, my, mz);

    let mut ans = 0;
    /*for &(x, y, z) in &set {
        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                for dz in -1i32..=1 {
                    if dx.abs() + dy.abs() + dz.abs() != 1 {
                        continue;
                    }
                    if !set.contains(&(x + dx, y + dy, z + dz)) {
                        ans += 1;
                    }
                }
            }
        }
    }*/
    let mut outer = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((-1, -1, -1));
    while let Some((x, y, z)) = q.pop_front() {
        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                for dz in -1i32..=1 {
                    if dx.abs() + dy.abs() + dz.abs() != 1 {
                        continue;
                    }
                    let nx = x + dx;
                    let ny = y + dy;
                    let nz = z + dz;
                    if nx < -1 || nx >= mx + 2 || ny < -1 || ny >= my + 2 || nz < -1 || nz >= mz + 2
                    {
                        continue;
                    }
                    if set.contains(&(nx, ny, nz)) {
                        ans += 1;
                        continue;
                    }
                    if !outer.contains(&(nx, ny, nz)) {
                        outer.insert((nx, ny, nz));
                        q.push_back((nx, ny, nz));
                    }
                }
            }
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
