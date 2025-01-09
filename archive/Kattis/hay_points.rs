//{"name":"Hay Points","group":"Kattis","url":"https://open.kattis.com/problems/haypoints","interactive":false,"timeLimit":1000,"tests":[{"input":"7 2\nadminister 100000\nspending 200000\nmanage 50000\nresponsibility 25000\nexpertise 100\nskill 50\nmoney 75000\nthe incumbent will administer the spending of kindergarden milk money\nand exercise responsibility for making change he or she will share\nresponsibility for the task of managing the money with the assistant\nwhose skill and expertise shall ensure the successful spending exercise\n.\nthis individual must have the skill to perform a heart transplant and\nexpertise in rocket science\n.\n","output":"700150\n150\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HayPoints"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read_size();
    let n = input.read_size();
    let words: Vec<(Str, i64)> = input.read_vec(m);

    let value = words.into_iter().collect::<DefaultHashMap<_, _>>();
    for _ in 0..n {
        let mut ans = 0;
        for w in input.iter_str() {
            if w.as_slice() == b"." {
                break;
            }
            ans += value[w];
        }
        out.print_line(ans);
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
