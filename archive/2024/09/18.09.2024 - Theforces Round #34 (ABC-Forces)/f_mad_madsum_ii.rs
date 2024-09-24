//{"name":"F. Mad MAD Sum II","group":"Codeforces - Theforces Round #34 (ABC-Forces)","url":"https://codeforces.com/gym/105350/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n1 2\n4\n4 2 2 4\n8\n5 2 1 2 1 5 2 1\n","output":"0\n10\n40\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMadMADSumII"}}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::collections::{BTreeSet, HashMap};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut last = HashMap::new();
    let mut ans = 0;
    let mut cur = 0;
    let mut pos = BTreeSet::new();
    pos.insert(n);
    for i in (0..n).rev() {
        if let Some(&next) = last.get(&a[i]) {
            let prev_val = pos.floor(&next).map(|&p| a[p]).unwrap_or(0);
            if prev_val < a[i] {
                while let Some(&p) = pos.ceil(&next) {
                    if p == n || a[i] < a[p] {
                        cur += (a[i] - prev_val) * (p - next);
                        break;
                    }
                    let next_next = *pos.next(&p).unwrap();
                    cur -= (a[p] - prev_val) * (next_next - p);
                    pos.remove(&p);
                }
                pos.insert(next);
            }
        }
        last.insert(a[i], i);
        ans += cur;
    }
    out.print_line(ans);
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
