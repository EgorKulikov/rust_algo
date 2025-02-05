//{"name":"Pig Latin","group":"Kattis","url":"https://open.kattis.com/problems/piglatin","interactive":false,"timeLimit":3000,"tests":[{"input":"i cant speak pig latin\n","output":"iyay antcay eakspay igpay atinlay\n"},{"input":"the quick brown fox\njumps over the lazy dog\nand ordinary foxes\ndont jump over lazy dogs\n","output":"ethay uickqay ownbray oxfay\numpsjay overyay ethay azylay ogday\nandyay ordinaryyay oxesfay\nontday umpjay overyay azylay ogsday\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::eol::EolVec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::concat::StrConcat;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s: EolVec<Str> = input.read();

    let mut ans = Vec::with_capacity(s.len());
    for w in s.unwrap() {
        let vowel_index = w
            .iter()
            .position(|c| match *c {
                b'a' | b'e' | b'i' | b'o' | b'u' | b'y' => true,
                _ => false,
            })
            .unwrap();
        if vowel_index == 0 {
            ans.push(Str::from(w.str_concat(b"yay")));
        } else {
            ans.push(Str::from(
                w[vowel_index..]
                    .str_concat(&w[..vowel_index])
                    .str_concat(b"ay"),
            ));
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
