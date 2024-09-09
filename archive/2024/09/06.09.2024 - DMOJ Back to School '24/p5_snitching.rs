//{"name":"P5 - Snitching","group":"DMOJ - Back to School '24","url":"https://dmoj.ca/problem/bts24p5","interactive":false,"timeLimit":3000,"tests":[{"input":"10 6\n3 2 3 1 1 7 7 10 9 2\n1 7 1\n6 10 3\n4 6 1\n4 10 5\n3 10 1\n5 10 2\n","output":"3\n-1\n1\n-1\n3\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P5Snitching"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n).dec();
    let queries = input.read_vec::<(usize, usize, usize)>(q).dec();

    let mut pos = vec![Vec::new(); n];
    for (i, &a) in a.iter().enumerate() {
        pos[a].push(i);
    }
    const BUBEN: usize = 500;
    let mut kth = vec![n; BUBEN];
    let mut at = Vec::with_capacity(n);
    for i in 0..n {
        at.push(pos[i].len());
    }
    let mut ans = vec![None; q];
    let mut q_at = vec![Vec::new(); n];
    for (id, (l, r, k)) in queries.into_iter().enumerate() {
        q_at[l].push((r, k, id));
    }
    let mut big = Vec::new();
    for i in (0..n).rev() {
        at[a[i]] -= 1;
        for j in 0..BUBEN.min(pos[a[i]].len() - at[a[i]]) {
            kth[j].minim(pos[a[i]][at[a[i]] + j]);
        }
        if pos[a[i]].len() == at[a[i]] + BUBEN + 1 {
            big.push(a[i]);
        }
        for &(r, k, id) in &q_at[i] {
            let res = if k <= BUBEN {
                kth[k - 1]
            } else {
                let mut res = n;
                for &j in &big {
                    if at[j] + k <= pos[j].len() {
                        res.minim(pos[j][at[j] + k - 1]);
                    }
                }
                res
            };
            if res <= r {
                ans[id] = Some(a[res] + 1);
            }
        }
    }
    out.print_per_line(&ans);
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
