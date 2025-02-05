//{"name":"Martian DNA","group":"Kattis","url":"https://open.kattis.com/problems/martiandna","interactive":false,"timeLimit":2000,"tests":[{"input":"5 2 2\n0 1 1 0 1\n0 1\n1 1\n","output":"2\n"},{"input":"13 4 3\n1 1 3 2 0 1 2 0 0 0 0 3 1\n0 2\n2 1\n1 2\n","output":"7\n"},{"input":"5 3 1\n1 2 0 1 2\n0 2\n","output":"impossible\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let r = input.read_size();
    let dna = input.read_size_vec(n);
    let mut need = vec![0; k];
    for _ in 0..r {
        let b = input.read_size();
        let q = input.read_size();
        need[b] = q;
    }

    let mut ans = None;
    let mut qty = vec![0; k];
    let mut at = 0;
    let mut bad = r;
    for i in 0..n {
        qty[dna[i]] += 1;
        if qty[dna[i]] == need[dna[i]] {
            bad -= 1;
        }
        if bad > 0 {
            continue;
        }
        while at < i {
            if qty[dna[at]] > need[dna[at]] {
                qty[dna[at]] -= 1;
                at += 1;
            } else {
                break;
            }
        }
        ans.minim(i - at + 1);
    }
    if let Some(ans) = ans {
        out.print_line(ans);
    } else {
        out.print_line("impossible");
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

//START MAIN
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
//END MAIN
