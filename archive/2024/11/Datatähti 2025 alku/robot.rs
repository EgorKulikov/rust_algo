//{"name":"Robot","group":"CSES - Datatähti 2025 alku","url":"https://cses.fi/524/task/B","interactive":false,"timeLimit":1000,"tests":[{"input":"20\n**.*......*.R*...*..\n","output":"4 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Robot"}}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::cmp::Ordering;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let s = input.read_str();

    let mut pos = s.iter().find_eq(b'R').unwrap();
    let mut stars = s
        .iter()
        .enumerate()
        .filter(|(_, c)| *c == b'*')
        .map(|(i, _)| i)
        .collect::<BTreeSet<_>>();
    let mut ans = 0;
    let mut coins = 0;
    loop {
        let left = stars.floor(&pos).copied();
        let right = stars.ceil(&pos).copied();

        match (left, right) {
            (Some(l), Some(r)) => match (pos - l).cmp(&(r - pos)) {
                Ordering::Less => {
                    ans += pos - l;
                    pos = l;
                    stars.remove(&l);
                }
                Ordering::Greater => {
                    ans += r - pos;
                    pos = r;
                    stars.remove(&r);
                }
                Ordering::Equal => {
                    break;
                }
            },
            (Some(l), None) => {
                ans += pos - l;
                pos = l;
                stars.remove(&l);
            }
            (None, Some(r)) => {
                ans += r - pos;
                pos = r;
                stars.remove(&r);
            }
            (None, None) => break,
        }
        coins += 1;
    }
    out.print_line((ans, coins));
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
