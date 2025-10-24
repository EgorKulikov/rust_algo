//{"name":"C. Voldemort’s Forbidden Words","group":"Toph","url":"https://toph.co/arena?contest=diu-code-trap-fall-2024-r#!/p/67f37555cf762cd9aec91ecd","interactive":false,"timeLimit":1000,"tests":[{"input":"accbadbac\n2\nbad\ncab\n","output":"1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::qty;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();
    let n = input.read_size();
    let t = input.read_str_vec(n);

    let mut q = qty(&s);
    // let mut non_zero = false;
    for t in t {
        let cur = qty(&t);
        let mut ans = usize::MAX;
        for (&c, &cnt) in cur.iter() {
            ans.minim(q[c] / cnt);
        }
        out.print_line(ans);
        // if ans > 0 {
        //     non_zero = true;
        // }
        for (c, cnt) in cur {
            q[c] -= cnt * ans;
        }
    }
    if n == 0 {
        out.print_line("No curse from Voldemort!");
    }
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
