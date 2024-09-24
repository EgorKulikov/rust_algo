//{"name":"A - Appraiser","group":"AtCoder - AtCoder Regular Contest 184","url":"https://atcoder.jp/contests/arc184/tasks/arc184_a","interactive":true,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAppraiser"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut q = input.read_size();

    let mut dsu = DSU::new(n);
    for i in 0..n / (m + 1) {
        let mut other = None;
        for j in 1..=m {
            if q == 0 {
                loop {}
            }
            q -= 1;
            out.print_line(('?', i * (m + 1) + 1, i * (m + 1) + j + 1));
            out.flush();
            if input.read_int() == 0 {
                dsu.join(i * (m + 1), i * (m + 1) + j);
            } else if let Some(other) = other {
                dsu.join(other, i * (m + 1) + j);
            } else {
                other = Some(i * (m + 1) + j);
            }
        }
    }
    let mut ans = Vec::with_capacity(m);
    for i in 0..n {
        if dsu.get(i) == i && dsu.size(i) > m {
            for j in 0..n {
                if dsu.get(j) == j && dsu.size(j) <= m - ans.len() {
                    if q == 0 {
                        loop {}
                    }
                    q -= 1;
                    out.print_line(('?', i + 1, j + 1));
                    out.flush();
                    if input.read_int() == 1 {
                        for k in 0..n {
                            if dsu.get(k) == j {
                                ans.push(k);
                            }
                        }
                    }
                }
            }
            assert_eq!(ans.len(), m);
            break;
        }
    }
    out.print_line(('!', ans.inc()));
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
