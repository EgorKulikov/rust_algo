//{"name":"K. LR String","group":"Universal Cup - The 3rd Universal Cup. Stage 23: Hong Kong","url":"https://contest.ucup.ac/contest/1885/problem/9925","interactive":false,"timeLimit":1000,"tests":[{"input":"2\nRRLLRRLL\n4\nLLLLL\nLLR\nLRLR\nR\nRLLLLLL\n3\nLLLLL\nRL\nRRL\n","output":"NO\nYES\nNO\nYES\nYES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KLRString"}}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::gen::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    let next_l = Vec::gen_back(s.len() + 1, |i, n| {
        if i >= s.len() {
            i
        } else if s[i] == b'L' {
            i
        } else {
            n[i + 1]
        }
    });
    let next_r = Vec::gen_back(s.len() + 1, |i, n| {
        if i >= s.len() {
            i
        } else if s[i] == b'R' {
            i
        } else {
            n[i + 1]
        }
    });

    let q = input.read_size();
    for _ in 0..q {
        let t = input.read_str();
        if s[0] == b'L' && t[0] == b'R' {
            out.print_line(false);
            continue;
        }
        if s[Back(0)] == b'R' && t[Back(0)] == b'L' {
            out.print_line(false);
            continue;
        }
        let mut at = 0;
        for c in t {
            if at >= s.len() {
                at += 1;
                break;
            }
            if c == b'L' {
                at = next_l[at] + 1;
            } else {
                at = next_r[at] + 1;
            }
        }
        out.print_line(at <= s.len());
    }
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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
