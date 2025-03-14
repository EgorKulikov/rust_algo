//{"name":"T584566 [语言月赛 202503] 洗牌","group":"Luogu","url":"https://www.luogu.com.cn/problem/T584566?contestId=235262","interactive":false,"timeLimit":1000,"tests":[{"input":"4\nA1,B2,C3,D4,E5,F6,G7,H8\nLRRLRLRL\n","output":"D4\nC3\nB2\nE5\n"},{"input":"4\n1,2,1,2,1,2,1,2\nLRRLRLRL\n","output":"2\n1\n2\n1\n"},{"input":"10\ntLWd,V,72r,t,4o1Q,1AO,FPul,9,g,REF,m8Lb2,V23m,LbzH,Oc6a,th,6,E7u,KHSdt,vEjtU,TQy\nRRLRLRRLRLRLLRLLLRRL\n","output":"REF\nvEjtU\n9\nKHSdt\n4o1Q\nt\n72r\nOc6a\nLbzH\nV23m\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let cards = input
        .read_str()
        .split(|&c| c == b',')
        .map(|x| Str::from(x))
        .collect::<Vec<_>>();
    let s = input.read_str();

    let mut left = 0;
    let mut right = n;
    let mut deck = Vec::new();
    for c in s {
        match c {
            b'L' => {
                deck.push(cards[left].clone());
                left += 1;
            }
            b'R' => {
                deck.push(cards[right].clone());
                right += 1;
            }
            _ => unreachable!(),
        }
    }
    deck.reverse();
    out.print_per_line_iter(deck.iter_step_by(2));
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
