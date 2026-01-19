//{"name":"E. The Robotic Rush","group":"Codeforces - Codeforces Round 1074 (Div. 4)","url":"https://codeforces.com/contest/2185/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n2 1 3\n0 1\n2\nLRR\n2 3 3\n2 4\n1 3 5\nLRL\n3 2 3\n1 3 7\n9 6\nRRL\n","output":"2 2 1\n0 0 0\n3 2 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();

    let a = input.read_int_vec(n).sorted();
    let b = input.read_int_vec(m).sorted();

    let mut death = DefaultHashMap::new(Vec::new());
    for i in 0..n {
        let pos = b.lower_bound(&a[i]);
        if pos != m {
            death[b[pos] - a[i]].push(i);
        }
        if pos != 0 {
            death[b[pos - 1] - a[i]].push(i);
        }
    }
    let mut pos = 0;
    let mut dead = FxHashSet::default();
    let mut ans = Vec::with_capacity(k);
    for _ in 0..k {
        let dir = input.read_char();
        match dir {
            b'L' => {
                pos -= 1;
            }
            b'R' => {
                pos += 1;
            }
            _ => unreachable!(),
        }
        for i in death[pos].drain(..) {
            dead.insert(i);
        }
        ans.push(n - dead.len());
    }
    out.print_line(ans);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
