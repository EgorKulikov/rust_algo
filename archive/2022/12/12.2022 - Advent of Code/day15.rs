//{"name":"day15","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day15"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out_line, scan};
// use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    // scan!(input, "y=@", ty: i32);
    scan!(input, "max=@", bound: i64);
    // let mut cannot = HashSet::new();
    // let mut beacons = HashSet::new();
    let mut sensors = Vec::new();
    while !input.is_exhausted() {
        scan!(
            input,
            "Sensor at x=@, y=@: closest beacon is at x=@, y=@",
            x: i64,
            y: i64,
            bx: i64,
            by: i64,
        );
        sensors.push((x, y, bx, by));
        /*if by == ty {
            beacons.insert(bx);
        }
        let dist = (x - bx).abs() + (y - by).abs();
        if dist < (y - ty).abs() {
            continue;
        }
        let spread = dist - (y - ty).abs();
        for i in x - spread..=x + spread {
            cannot.insert(i);
        }*/
        input.skip_whitespace();
    }
    // out_line!(cannot.len() - beacons.len());
    sensors.sort_by_key(|&(x, ..)| x);
    for y in 0.. {
        let mut x = 0;
        for &(px, py, bx, by) in &sensors {
            let dist = (px - bx).abs() + (py - by).abs();
            if dist < (py - y).abs() {
                continue;
            }
            let spread = dist - (py - y).abs();
            if x >= px - spread {
                x.maxim(px + spread + 1);
            }
        }
        if x <= bound {
            out_line!(x * bound + y);
            return;
        }
    }
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
