//{"name":"P3 - Interplanetary Travel","group":"DMOJ - Singularity Cup","url":"https://dmoj.ca/problem/scp3","interactive":false,"timeLimit":3000,"tests":[{"input":"4 4 1\n5 2 7 9\n2 1\n3 2\n4 3\n4 2\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P3InterplanetaryTravel"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let p = input.read_size() - 1;
    let mut d = input.read_long_vec(n);
    let locks = input.read_size_pair_vec(m).dec();

    let val = d[p];
    let locks = locks.iter().map(|&(a, b)| (d[a], d[b])).collect::<Vec<_>>();
    d.sort();
    let p = d.lower_bound(&val);
    let locks = locks
        .iter()
        .map(|&(a, b)| (d.lower_bound(&b), d.lower_bound(&a)))
        .collect::<Vec<_>>();
    let mut num_locks = vec![0; n];
    let mut unlock = vec![Vec::new(); n];
    for (a, b) in locks {
        num_locks[b] += 1;
        unlock[a].push(b);
    }
    let mut ans = 0;
    let mut left = p;
    let mut right = p;
    let mut unlocked = BTreeSet::new();
    for i in 0..n {
        if num_locks[i] == 0 {
            unlocked.insert(i);
        }
    }
    for &i in &unlock[p] {
        num_locks[i] -= 1;
        if num_locks[i] == 0 {
            unlocked.insert(i);
        }
    }
    while left != 0 || right != n - 1 {
        let to_left = unlocked.range(..left).next_back().copied();
        let to_right = unlocked.range(right + 1..).next().copied();
        let left_dist = to_left.map(|i| d[left] - d[i]).unwrap_or(i64::MAX);
        let right_dist = to_right.map(|i| d[i] - d[right]).unwrap_or(i64::MAX);
        let mut to_unlock = Vec::new();
        if left_dist < right_dist {
            to_unlock.push(to_left.unwrap());
            left = to_left.unwrap();
            ans.maxim(left_dist);
        } else {
            to_unlock.push(to_right.unwrap());
            right = to_right.unwrap();
            ans.maxim(right_dist);
        }
        while let Some(p) = to_unlock.pop() {
            for &i in &unlock[p] {
                num_locks[i] -= 1;
                if num_locks[i] == 0 {
                    unlocked.insert(i);
                    if i >= left && i <= right {
                        to_unlock.push(i);
                    }
                }
            }
        }
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
