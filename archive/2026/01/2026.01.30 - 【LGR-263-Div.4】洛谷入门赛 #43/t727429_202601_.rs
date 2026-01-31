//{"name":"T727429 [语言月赛 202601] 题库管理","group":"Luogu","url":"https://www.luogu.com.cn/problem/T727429?contestId=304401","interactive":false,"timeLimit":1000,"tests":[{"input":"3 10 6\n1 2 1\n1 3 1\n1 4 2\n2 2 0\n2 2 1\n2 2 2\n","output":"5\n3\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::treap::treap_map::TreapSet;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();

    let mut id = vec![0; m + 1];
    let mut sets = Vec::with_gen(n, |i| {
        if i == 0 {
            unsafe { TreapSet::with_gen(m, |i| i + 1) }
        } else {
            TreapSet::new()
        }
    });
    for _ in 0..k {
        let t = input.read_int();
        match t {
            1 => {
                let u = input.read_size();
                let v = input.read_size();
                sets[id[u]].remove(&u);
                id[u] = v;
                sets[v].insert(u);
            }
            2 => {
                let u = input.read_size() - 1;
                let v = input.read_size();
                out.print_line(sets[v].get_at(u));
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
