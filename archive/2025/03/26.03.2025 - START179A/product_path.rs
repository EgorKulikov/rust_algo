//{"name":"Product Path","group":"CodeChef - START179A","url":"https://www.codechef.com/START179A/problems/PRODPATH","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 2 2 2\n1 4 3 10\n6 10 2 5\n720720 123456 210 35\n","output":"12\n4\n105\n181661800320\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let m = input.read_long();
    let p = input.read_long();
    let q = input.read_long();

    let cost = |x: i64, y: i64| -> i64 {
        if n % x == 0 && m % y == 0 {
            x.max(n / x) * y.max(m / y)
        } else {
            0
        }
    };
    let mut ans = None;
    let mut go_out = 0;
    for ((sx, sy), (tx, ty)) in [((1, 1), (p, q)), ((p, q), (1, 1))] {
        let mut processed = FxHashSet::default();
        let mut was_out = false;
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(cost(sx, sy)), sx, sy));
        while let Some((Reverse(c), x, y)) = heap.pop() {
            if !processed.insert((x, y)) {
                continue;
            }
            if x == tx && y == ty {
                ans.minim(c);
            }
            if cost(x, y) == 0 {
                if !was_out {
                    was_out = true;
                    go_out += c;
                }
                continue;
            }
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let cx = x + dx;
                let cy = y + dy;
                if cx <= 0 || cy <= 0 {
                    continue;
                }
                heap.push((Reverse(c + cost(cx, cy)), cx, cy));
            }
        }
    }
    ans.minim(go_out);
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
