//{"name":"Keylogger","group":"Kattis","url":"https://open.kattis.com/problems/keylogger","interactive":false,"timeLimit":1000,"tests":[{"input":"26\nclank\nbong\nclick\ntap\npoing\nclonk\nclack\nping\ntip\ncloing\ntic\ncling\nbing\npong\nclang\npang\nclong\ntac\nboing\nboink\ncloink\nrattle\nclock\ntoc\nclink\ntuc\n","output":"abcdefghijklmnopqrstuvwxyz\n"},{"input":"14\nbump\ntip\nwhack\nbump\nclock\nclank\npong\nboink\nwhack\npang\ntip\ntuc\ntuc\nclank\n","output":"I want pizza\n"},{"input":"10\ndink\npong\nclang\nwhack\nbump\nclick\nthumb\nclank\npang\nboing\n","output":"NO cAPS\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let presses = input.read_str_vec(n);

    let mut ans = Str::new();
    let mut capslock = false;
    let mut shift = false;

    for s in presses {
        let c = match s.as_slice() {
            b"clank" => b'a',
            b"bong" => b'b',
            b"click" => b'c',
            b"tap" => b'd',
            b"poing" => b'e',
            b"clonk" => b'f',
            b"clack" => b'g',
            b"ping" => b'h',
            b"tip" => b'i',
            b"cloing" => b'j',
            b"tic" => b'k',
            b"cling" => b'l',
            b"bing" => b'm',
            b"pong" => b'n',
            b"clang" => b'o',
            b"pang" => b'p',
            b"clong" => b'q',
            b"tac" => b'r',
            b"boing" => b's',
            b"boink" => b't',
            b"cloink" => b'u',
            b"rattle" => b'v',
            b"clock" => b'w',
            b"toc" => b'x',
            b"clink" => b'y',
            b"tuc" => b'z',
            b"whack" => b' ',
            b"bump" => {
                capslock = !capslock;
                continue;
            }
            b"pop" => {
                ans.pop();
                continue;
            }
            b"dink" => {
                shift = true;
                continue;
            }
            b"thumb" => {
                shift = false;
                continue;
            }
            _ => unreachable!(),
        };
        let c = if shift ^ capslock {
            c.to_ascii_uppercase()
        } else {
            c
        };
        ans.push(c);
    }
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
