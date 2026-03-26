//{"name":"E. Race in Laurasia","group":"Codeforces - IUT Eid Salami Programming Contest 2026 - Powered by Okkhor Technology (Online Mirror)","url":"https://codeforces.com/gym/106438/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n6 5 4\n2 2 2\n3 2 2\n4 4 4\n5 4 4\n","output":"1 499122177 748683265 499122177 499122177 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization3;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _m = input.read_size();
    let k = input.read_size();
    let ilr = input.read_vec::<(usize, usize, usize)>(k);

    let mut lanes = vec![Vec::new(); n];
    for (i, l, r) in ilr {
        lanes[i - 1].push((l, r));
    }
    for i in 0..n {
        lanes[i].sort();
    }
    type Mod = ModIntF;
    let mut mem = Memoization3::new(|mem, lane: usize, pos: usize, changed: usize| -> Mod {
        let next = lanes[lane].lower_bound(&(pos, usize::MAX));
        if next > 0 {
            assert!(lanes[lane][next - 1].1 < pos);
        }
        if next == lanes[lane].len() {
            return Mod::new(1);
        }
        if lanes[lane][next].0 != pos + 1 {
            return mem.call(lane, lanes[lane][next].0 - 1, 0);
        }
        if changed == 1 {
            return Mod::new(0);
        }
        let mut res = Mod::new(0);
        let mut times = 0;
        if lane > 0 {
            let pos_up = lanes[lane - 1].lower_bound(&(pos, usize::MAX));
            if pos_up == 0 || pos > lanes[lane - 1][pos_up - 1].1 {
                times += 1;
                res += mem.call(lane - 1, pos, 1);
            }
        }
        if lane + 1 < n {
            let pos_down = lanes[lane + 1].lower_bound(&(pos, usize::MAX));
            if pos_down == 0 || pos > lanes[lane + 1][pos_down - 1].1 {
                times += 1;
                res += mem.call(lane + 1, pos, 1);
            }
        }
        if times > 0 {
            res /= times;
        }
        res
    });
    out.print_line_iter((0..n).map(|i| mem.call(i, 0, 0)));
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
