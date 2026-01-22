//{"name":"Permutation Encryption","group":"Kattis","url":"https://open.kattis.com/problems/permutationencryption","interactive":false,"timeLimit":1000,"tests":[{"input":"1 1\nFour score and seven years ago\n2 2 1\nour fathers brough forth on this continent a new nation,\n5 1 3 2 5 4\nconceived in liberty and dedicated to the proposition\n10 5 10 8 1 3 6 4 7 2 9\nthat all men are created equal.\n0\n","output":"'Four score and seven years ago'\n'uo rafhtre srbuohgf rohto  nhtsic noitentna n wen taoi,n'\n'cnoeciev di nilbreyt na dddeciaet dt ohtep orpsotiino  '\n' mltaatlh rece ea nr luaeedqta   .      '\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    if n == 0 {
        return;
    }
    let p = input.read_size_vec(n).dec();
    let mut s = input.read_line();

    while s.len() % n != 0 {
        s.push(b' ');
    }
    let mut t = Str::from(vec![b' '; s.len()]);
    for i in s.indices() {
        t[i] = s[i / n * n + p[i % n]];
    }
    output!(out, "'{}'", t);
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
