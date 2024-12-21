//{"name":"T549656 「ALFR Round 3」C 割","group":"Luogu","url":"https://www.luogu.com.cn/problem/T549656?contestId=210959","interactive":false,"timeLimit":1000,"tests":[{"input":"7 2\nskyaqua\n","output":"y\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T549656ALFRRound3C"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let k = input.read_size();
    let s = input.read_str();

    let max = s.iter().max().unwrap();
    let count = s.iter().filter(|&c| c == max).count();
    if count % k == 0 {
        let mut last = s.iter().rposition(|c| c == max).unwrap();
        out.print(Str::from(vec![max; count / k - 1]));
        for c in (b'a'..=max).rev() {
            let qty = s.iter().skip(last).filter(|&x| x == c).count();
            for _ in 0..qty {
                out.print(c);
            }
            if qty > 0 {
                last = s.iter().rposition(|x| x == c).unwrap();
            }
        }
        out.print_line(());
    } else {
        out.print_line(Str::from(vec![max; count.div_ceil(k)]));
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
