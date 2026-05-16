//{"name":"C - Team Flag Colors","group":"AtCoder - AtCoder Weekday Contest 0068 Beta","url":"https://atcoder.jp/contests/awc0068/tasks/awc0068_c","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3\n1 2 10\n3 4 20\n2 3 10\n","output":"1\n"},{"input":"5 4\n1 2 7\n2 3 8\n1 3 7\n4 5 8\n","output":"2\n"},{"input":"10 9\n1 2 10\n2 3 20\n4 5 10\n6 7 30\n8 9 30\n1 3 40\n5 6 20\n7 4 50\n9 10 50\n","output":"2\n"},{"input":"15 16\n1 2 1\n3 4 2\n5 6 3\n7 8 4\n9 10 5\n11 12 6\n13 14 7\n2 3 8\n6 7 9\n10 11 5\n1 4 1\n8 5 3\n12 9 6\n4 5 8\n14 15 7\n1 8 2\n","output":"3\n"},{"input":"2 1\n1 2 1000000000\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut dsu = DSU::new(n);
    let mut colors = vec![None; n];
    for _ in 0..m {
        let u = input.read_size() - 1;
        let v = input.read_size() - 1;
        let c = input.read_int();
        dsu.union(u, v);
        colors[dsu.find(u)] = Some(c);
    }
    let mut ans = FxHashSet::default();
    for i in 0..n {
        if let Some(c) = colors[dsu.find(i)] {
            ans.insert(c);
        }
    }
    out.print_line(ans.len());
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
