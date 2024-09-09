//{"name":"P3 - Tournament","group":"DMOJ - Back to School '24","url":"https://dmoj.ca/problem/bts24p3","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 4 3\n","output":"6 6 9 9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P3Tournament"}}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n - 1).dec();

    let we = n * (n - 1) / 2 - a.iter().sum::<usize>();
    let mut b = Vec::new();
    let mut cur = vec![we];
    cur.extend(a.iter().copied());
    b.push(cur);
    while b.last().unwrap().len() != 1 {
        let last = b.last().unwrap();
        let mut next = Vec::with_capacity(last.len() >> 1);
        for i in last.indices().step_by(2) {
            next.push(last[i].max(last[i + 1]));
        }
        b.push(next);
    }
    let mut cur = 0;
    let mut ans = Vec::with_capacity(n);
    for b in &b {
        for i in b.indices().skip(2).step_by(2) {
            let delta = b[i].abs_diff(b[i + 1]);
            cur += delta * delta;
        }
    }
    for i in 0..n {
        b[0][i] = we;
        let mut add = 0;
        let mut pos = i;
        for j in 0..b.len() - 1 {
            let delta = b[j][pos].abs_diff(b[j][pos ^ 1]);
            add += delta * delta;
            b[j + 1][pos >> 1] = b[j][pos].max(b[j][pos ^ 1]);
            pos >>= 1;
        }
        ans.push(cur + add);
        if i + 1 < n {
            let mut pos = i + 1;
            let mut at = 0;
            while pos % 2 == 0 {
                let delta = b[at][pos].abs_diff(b[at][pos + 1]);
                cur -= delta * delta;
                pos >>= 1;
                at += 1;
            }
            b[0][i] = b[0][i + 1];
            let mut pos = i;
            let mut at = 0;
            while pos % 2 == 1 {
                let delta = b[at][pos].abs_diff(b[at][pos - 1]);
                cur += delta * delta;
                b[at + 1][pos >> 1] = b[at][pos - 1].max(b[at][pos]);
                pos >>= 1;
                at += 1;
            }
        }
    }
    out.print_line(ans);
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
