//{"name":"DRM Messages","group":"Kattis","url":"https://open.kattis.com/problems/drmmessages","interactive":false,"timeLimit":1000,"tests":[{"input":"EWPGAJRB\n","output":"ABCD\n"},{"input":"UEQBJPJCBUDGBNKCAHXCVERXUCVK\n","output":"ACMECNACONTEST\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRMMessages"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::vec_ext::gen::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input
        .read_str()
        .iter()
        .map(|c| c as usize - 'A' as usize)
        .collect::<Vec<_>>();

    let mut halfs = Vec::new();
    for half in a.chunks_exact(a.len() / 2) {
        let sum = half.iter().sum::<usize>() % 26;
        halfs.push(half.iter().copied().map(|c| (c + sum) % 26).collect_vec());
    }
    out.print_line(Str::from(Vec::gen(a.len() / 2, |i, _| {
        let c = (halfs[0][i] + halfs[1][i]) % 26;
        (c + 'A' as usize) as u8
    })));
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
