//{"name":"witch_s_cauldron_part_2","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::eol::EolStr;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};
use algo_lib::{scan, str_scan};
use std::io::Write;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let base = input.read_line_vec(n);
    let recipes = (0..m).map(|_| {
        scan!(input, "@= @; brew for @ minutes\n", name: EolStr, ingridients: EolStr, time: i64);
        let mut list = Vec::new();
        for token in ingridients.trim_ascii().split(|x| *x == b',') {
            let token = token.trim_ascii();
            str_scan!(token, "@(@)", name: EolStr, qty: i64);
            list.push((Str::from(name.unwrap().trim_ascii()), qty));
        }
        (Str::from(name.unwrap().trim_ascii()), list, time)
    }).collect::<Vec<_>>();

    let mut mem = Memoization::new(|rec, name: Str| -> i64 {
        for i in 0..n {
            if base[i] == name {
                return 0;
            }
        }
        let mut res = None;
        for i in 0..m {
            if recipes[i].0 == name {
                let mut cur = 0;
                for (ing_name, qty) in &recipes[i].1 {
                    let sub = rec.call(ing_name.clone());
                    cur += sub * (*qty);
                }
                cur += recipes[i].2;
                res.minim(cur);
            }
        }
        res.unwrap()
    });
    writeln!(
        out,
        "Case #{}: {}",
        test_case,
        mem.call(Str::from(b"witch's brew"))
    )
    .unwrap();
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
