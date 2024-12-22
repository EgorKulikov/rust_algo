//{"name":"day_22","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_22"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::collections::{HashSet, VecDeque};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut data = Vec::new();
    while !input.is_empty() {
        data.push(input.read_long());
    }

    // part 1
    {
        let mut ans = 0;
        for mut d in data.copy_iter() {
            for _ in 0..2000 {
                let d1 = d * 64;
                d ^= d1;
                d %= 16777216;
                let d1 = d / 32;
                d ^= d1;
                d %= 16777216;
                let d1 = d * 2048;
                d ^= d1;
                d %= 16777216;
            }
            ans += d;
        }
        out.print_line(ans);
    }

    // part 2
    {
        let mut ans = DefaultHashMap::<_, i64>::new();
        for mut d in data.copy_iter() {
            let mut changes = VecDeque::new();
            let mut was = HashSet::new();
            let mut last = d % 10;
            for _ in 0..2000 {
                let d1 = d * 64;
                d ^= d1;
                d %= 16777216;
                let d1 = d / 32;
                d ^= d1;
                d %= 16777216;
                let d1 = d * 2048;
                d ^= d1;
                d %= 16777216;
                let cur = d % 10;
                changes.push_back(cur - last);
                if changes.len() > 4 {
                    changes.pop_front();
                }
                if was.insert(changes.clone()) {
                    ans[changes.clone()] += cur;
                }
                last = cur;
            }
        }
        out.print_line(ans.values().max());
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
