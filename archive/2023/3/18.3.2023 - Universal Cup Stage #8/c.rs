//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::rational::Rational;
use algo_lib::out_line;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let pts = input.read_long_pair_vec(n);

    let mut dist = Arr2d::new(2 * n - 1, 2 * n - 1, 0);
    let mut sz = vec![0; 2 * n - 1];
    let mut enabled = vec![false; 2 * n - 1];
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        sz[i] = 1;
        enabled[i] = true;
        for j in 0..i {
            let dx = pts[i].0 - pts[j].0;
            let dy = pts[i].1 - pts[j].1;
            let d = dx * dx + dy * dy;
            dist[(i, j)] = d;
            dist[(j, i)] = d;
            heap.push(Reverse((Rational::new(d, 1), j, i)));
        }
    }

    let mut next = n;
    while let Some(Reverse((_, i, j))) = heap.pop() {
        if !enabled[i] || !enabled[j] {
            continue;
        }
        enabled[i] = false;
        enabled[j] = false;
        enabled[next] = true;
        sz[next] = sz[i] + sz[j];
        for k in 0..next {
            if enabled[k] {
                let d = dist[(i, k)] + dist[(j, k)];
                dist[(next, k)] = d;
                dist[(k, next)] = d;
                heap.push(Reverse((
                    Rational::new(d, (sz[k] * sz[next]).into_i64()),
                    k,
                    next,
                )));
            }
        }
        out_line!(sz[next]);
        next += 1;
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
