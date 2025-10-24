//{"name":"T596594 [语言月赛 202504] 礼堂预约","group":"Luogu","url":"https://www.luogu.com.cn/problem/T596594?contestId=240317","interactive":false,"timeLimit":1000,"tests":[{"input":"5\nP 20250419 A\nO 20250419 A\nC 20250419 A\nC 20250419 E\nC 20250420 A\n","output":"20250422\n20250419\n20250420\n20250419\n20250421\n"},{"input":"6\nP 20280228 M\nP 20280228 M\nP 20290228 A\nP 20290228 A\nP 20991231 E\nP 20991231 E\n","output":"20280228\n20280229\n20290228\n20290301\n20991231\n21000101\n"},{"input":"5\nC 20250419 E\nC 20250420 A\nP 20250419 A\nO 20250419 A\nC 20250419 A\n","output":"20250419\n20250420\n20250422\n20250419\n20250421\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::io::input::{Input, Readable};
use algo_lib::io::output::{Output, Writable};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    #[derive(Hash, Eq, PartialEq, Copy, Clone)]
    struct Date {
        year: i32,
        month: i32,
        day: i32,
    }
    impl Date {
        fn next(&mut self) {
            self.day += 1;
            let last = match self.month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => {
                    if self.year % 4 == 0 && self.year % 100 != 0 || self.year % 400 == 0 {
                        29
                    } else {
                        28
                    }
                }
                _ => unreachable!(),
            };
            if self.day > last {
                self.day = 1;
                self.month += 1;
            }
            if self.month > 12 {
                self.month = 1;
                self.year += 1;
            }
        }
    }
    impl Readable for Date {
        fn read(input: &mut Input) -> Self {
            let date = input.read_int();
            Self {
                year: date / 10000,
                month: (date / 100) % 100,
                day: date % 100,
            }
        }
    }
    impl Writable for Date {
        fn write(&self, output: &mut Output) {
            output.print(format!("{}{:02}{:02}", self.year, self.month, self.day));
        }
    }

    let n = input.read_size();
    let mut events = input.read_vec::<(u8, Date, u8)>(n);

    fn priority(c: u8) -> i32 {
        match c {
            b'P' => 0,
            b'C' => 1,
            b'O' => 2,
            _ => unreachable!(),
        }
    }
    let mut maps = DefaultHashMap::new(FxHashMap::<Date, usize>::default());
    for mut i in 0..n {
        loop {
            if let Some(&id) = maps[events[i].2].get(&events[i].1) {
                let p_i = priority(events[i].0);
                let p_id = priority(events[id].0);
                if p_i > p_id || p_i == p_id && i < id {
                    maps[events[i].2].insert(events[i].1, i);
                    i = id;
                }
                events[i].1.next();
            } else {
                maps[events[i].2].insert(events[i].1, i);
                break;
            }
        }
    }

    for i in 0..n {
        out.print_line(events[i].1);
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
