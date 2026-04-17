//{"name":"The Language of Westeros","group":"Eolymp - Basecamp - Weekend Practice #22","url":"https://eolymp.com/en/compete/rast2dle4l13n0vnc70f076fao/problem/3","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n7 9\n2 5 6 2 9 7 4\n8 3\n1 3 2 1 1 3 1 3\n","output":"2\n1\n"},{"input":"5\n7 3\n3 2 1 3 2 2 2\n9 35\n25 33 32 9 2 28 33 5 20\n6 5\n4 4 4 2 3 2\n5 26\n18 9 5 23 4\n6 26\n14 16 25 8 19 5\n","output":"0\n12\n1\n6\n11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::qty;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n).dec();

    let qty = qty(&a);
    let mut odd = Vec::new();
    for (k, v) in qty {
        if v % 2 == 1 {
            odd.push(k);
        }
    }
    if odd.len() <= 1 {
        out.print_line(0);
        return;
    }
    odd.sort();
    if odd.len() % 2 == 0 {
        let mut cur = 0;
        for i in odd.indices().step_by(2) {
            cur += odd[i + 1] - odd[i];
        }
        let mut ans = cur;
        odd[0] += m;
        odd.rotate_left(1);
        let mut cur = 0;
        for i in odd.indices().step_by(2) {
            cur += odd[i + 1] - odd[i];
        }
        ans.minim(cur);
        out.print_line(ans);
    } else {
        let mut cur = 0;
        for i in odd.indices().skip(1).step_by(2) {
            cur += odd[i + 1] - odd[i];
        }
        let mut ans = cur;
        for i in odd.indices().skip(2).step_by(2) {
            cur -= odd[i] - odd[i - 1];
            cur += odd[i - 1] - odd[i - 2];
            ans.minim(cur);
        }
        odd[0] += m;
        odd.rotate_left(1);
        let mut cur = 0;
        for i in odd.indices().skip(1).step_by(2) {
            cur += odd[i + 1] - odd[i];
        }
        ans.minim(cur);
        for i in odd.indices().skip(2).step_by(2) {
            cur -= odd[i] - odd[i - 1];
            cur += odd[i - 1] - odd[i - 2];
            ans.minim(cur);
        }
        out.print_line(ans);
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
