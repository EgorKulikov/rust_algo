//{"name":"E - Trapezium","group":"AtCoder - AtCoder Beginner Contest 418","url":"https://atcoder.jp/contests/abc418/tasks/abc418_e","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n0 2\n0 5\n1 0\n2 1\n2 4\n","output":"3\n"},{"input":"8\n0 1\n1 3\n2 3\n3 1\n0 2\n1 0\n2 0\n3 2\n","output":"22\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let pts = input.read_long_pair_vec(n);

    let mut segs = DefaultHashMap::new(0usize);
    let mut dirs = DefaultHashMap::new(0usize);
    for i in 0..n {
        let (x1, y1) = pts[i];
        for j in 0..i {
            let (x2, y2) = pts[j];
            let mut dx = x1 - x2;
            let mut dy = y1 - y2;
            if dx < 0 || dx == 0 && dy < 0 {
                dx = -dx;
                dy = -dy;
            }
            segs[(dx, dy)] += 1;
            let g = gcd(dx, dy);
            dx /= g;
            dy /= g;
            dirs[(dx, dy)] += 1;
        }
    }
    let mut ans = 0;
    for v in dirs.into_values() {
        ans += v * (v - 1) / 2;
    }
    let mut par = 0;
    for v in segs.into_values() {
        par += v * (v - 1) / 2;
    }
    assert_eq!(par % 2, 0);
    ans -= par / 2;
    out.print_line(ans);
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
