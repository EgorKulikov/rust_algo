//{"name":"coderun_431","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"coderun_431"}}}

use algo_lib::collections::slice_ext::backward::BackwardSlice;
use algo_lib::geometry::point::Point;
use algo_lib::geometry::polygon::ConvexHull;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_vec::<Point<i64>>(n);

    out.set_bool_output(BoolOutput::YesNo);

    if n > 10 {
        for (a, b) in [(p[0], p[1]), (p[2], p[3]), (p[4], p[5])] {
            let line = a.line(b);
            let mut on_line = Vec::new();
            let mut not_on_line = Vec::new();
            for &p in &p {
                if line.contains(p) {
                    on_line.push(p);
                } else {
                    not_on_line.push(p);
                }
            }
            if on_line.len() >= n - 2 {
                if on_line.len() == n - 2 {
                    on_line.sort();
                    let mut pp = vec![
                        on_line[0],
                        on_line.backward()[0],
                        not_on_line[0],
                        not_on_line[1],
                    ];
                    let hull = pp.convex_hull();
                    if hull.points.len() == 4 {
                        out.print_line(true);
                        return;
                    }
                }
                out.print_line(false);
                return;
            }
        }
        out.print_line(true);
    } else {
        for i in 0..n {
            for j in 0..i {
                for k in 0..j {
                    for l in 0..k {
                        let mut pp = vec![p[i], p[j], p[k], p[l]];
                        let hull = pp.convex_hull();
                        if hull.points.len() == 4 {
                            out.print_line(true);
                            return;
                        }
                    }
                }
            }
        }
        out.print_line(false);
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
