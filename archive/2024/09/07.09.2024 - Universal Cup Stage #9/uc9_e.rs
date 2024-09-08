//{"name":"uc9_e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"uc9_e"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_char_table(n, n);

    let mut ans = Vec::with_capacity(3);
    let mut good = vec![0; n];
    let mut id = 0;
    for _ in 0..3 {
        let mut last = ans.last().copied();
        let mut set = (0..n).collect_vec();
        loop {
            let mut best = None;
            for &i in &set {
                if let Some(last) = last {
                    if s[(i, last)] != '1' {
                        continue;
                    }
                }
                let mut deg = 0;
                for &j in &set {
                    if s[(i, j)] == '1' {
                        deg += 1;
                    }
                }
                best.maxim((deg, i));
            }
            if let Some((_, i)) = best {
                id += 1;
                good[i] = id;
                for &j in &set {
                    if s[(i, j)] != '1' {
                        continue;
                    }
                    good[j] = id;
                    for &k in &set {
                        if s[(j, k)] == '1' {
                            good[k] = id;
                        }
                    }
                }
                let mut new_set = Vec::new();
                for &j in &set {
                    if good[j] != id {
                        new_set.push(j);
                    }
                }
                if new_set.is_empty() {
                    ans.push(i);
                    break;
                }
                set = new_set;
            } else {
                out.print_line("NOT FOUND");
                return;
            }
            last = None;
        }
        if ans.len() == 2 {
            if s[(ans[0], ans[1])] == '1' {
                ans.swap(0, 1);
            }
        }
    }
    out.print_line(ans.inc());
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
