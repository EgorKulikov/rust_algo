//{"name":"uc8_i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"uc8_i"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::rational::Rational;
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let x = input.read_long_table(m, n);

    let sums = x.rows().map(|r| x.row(r).sum::<i64>()).collect::<Vec<_>>();
    let mut t = Vec::with_capacity(m);
    for i in 0..m {
        t.push(sums[i] - sums[0]);
        if i > 0 && t[i] == 0 {
            out.print_line((1..=n).collect_vec());
            return;
        }
    }
    let mut v = Vec::with_capacity(n);
    let mut has = vec![HashSet::new(); m];
    for i in 0..m {
        for j in 0..n {
            has[i].insert(x[(i, j)]);
        }
    }
    for i in 0..n {
        for j in 0..n {
            if x[(1, j)] < x[(0, i)] {
                continue;
            }
            let cand_v = Rational::new(x[(1, j)] - x[(0, i)], t[1]);
            let mut good = true;
            for k in 2..m {
                let delta = cand_v * Rational::new(t[k], 1);
                if delta.den() != 1 || !has[k].contains(&(x[(0, i)] + delta.num())) {
                    good = false;
                    break;
                }
            }
            if good {
                v.push(cand_v);
                break;
            }
        }
        assert_eq!(v.len(), i + 1);
    }
    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| (v[i], x[(0, i)]));
    let mut r = vec![0; n];
    for i in 0..n {
        r[order[i]] = i + 1;
    }
    out.print_line(r);
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
