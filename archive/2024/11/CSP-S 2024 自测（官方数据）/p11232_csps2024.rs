//{"name":"P11232 [CSP-S 2024] 超速检测（官方数据）","group":"Luogu","url":"https://www.luogu.com.cn/problem/P11232?contestId=209925","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n5 5 15 3\n0 3 0\n12 4 0\n1 1 4\n5 5 -2\n6 4 -4\n2 5 8 9 15\n","output":"3 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P11232CSPS2024"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let _l = input.read_real();
    let v = input.read_real();
    let cars = input.read_vec::<(Real, Real, Real)>(n);
    let p = input.read_real_vec(m).sorted();

    let mut segments = Vec::new();
    for (d, vi, a) in cars {
        let seg = if a > 0. {
            if vi <= v {
                let t = (v - vi) / a;
                let from = d + vi * t + 0.5 * a * t * t;
                Some((p.upper_bound(&from), m))
            } else {
                Some((p.lower_bound(&d), m))
            }
        } else if a < 0. {
            if vi > v {
                let t = (vi - v) / (-a);
                Some((
                    p.lower_bound(&d),
                    p.lower_bound(&(d + vi * t + 0.5 * a * t * t)),
                ))
            } else {
                None
            }
        } else {
            if vi > v {
                Some((p.lower_bound(&d), m))
            } else {
                None
            }
        };
        if let Some((f, t)) = seg {
            if f < t {
                segments.push((f, t));
            }
        }
    }
    segments.sort();
    let qty = segments.len();
    let mut ans = 0;
    let mut to = None;
    for (f, t) in segments {
        if to.is_none() || to.unwrap() <= f {
            ans += 1;
            to = None;
        }
        to.minim(t);
    }
    out.print_line((qty, m - ans));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    Real::set_epsilon(1e-12);

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
