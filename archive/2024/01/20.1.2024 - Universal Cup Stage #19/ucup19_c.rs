//{"name":"ucup19_c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ucup19_c"}}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::Str;
use std::collections::HashSet;
use std::iter::once;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let p = input.read_size_vec(n - 1).dec();
    let q = input.read_size_vec(m - 1).dec();

    let k = *p.iter().min().unwrap();
    let mut left = (0..n)
        .map(|i| {
            if i < k {
                once(i).collect::<HashSet<_>>()
            } else {
                HashSet::new()
            }
        })
        .collect::<Vec<_>>();
    for i in 0..n - 1 {
        if left[i].len() % 2 == 1 {
            let x = *left[i].iter().next().unwrap();
            left[i].remove(&x);
            left[p[i]].insert(x);
        }
    }
    let mut right = (0..m)
        .map(|i| {
            if i < k {
                once(i).collect::<HashSet<_>>()
            } else {
                HashSet::new()
            }
        })
        .collect::<Vec<_>>();
    for i in 0..m - 1 {
        if right[i].len() % 2 == 1 {
            let x = *right[i].iter().next().unwrap();
            right[i].remove(&x);
            right[q[i]].insert(x);
        }
    }
    let mut left_at = vec![0; k];
    let mut right_at = vec![0; k];
    for i in 0..n {
        for &j in &left[i] {
            left_at[j] = i;
        }
    }
    for i in 0..m {
        for &j in &right[i] {
            right_at[j] = i;
        }
    }
    let mut ans = Str::from(vec![b' '; k]);
    if left[n - 1].len() % 2 == 1 {
        let mut x = *left[n - 1].iter().next().unwrap();
        left[n - 1].remove(&x);
        loop {
            ans[x] = b'R';
            let r = right_at[x];
            right[r].remove(&x);
            if right[r].len() % 2 == 0 {
                break;
            }
            let y = *right[r].iter().next().unwrap();
            right[r].remove(&y);
            ans[y] = b'B';
            let l = left_at[y];
            left[l].remove(&y);
            x = *left[l].iter().next().unwrap();
            left[l].remove(&x);
        }
    }
    for i in 0..n {
        while !left[i].is_empty() {
            let mut x = *left[i].iter().next().unwrap();
            left[i].remove(&x);
            loop {
                ans[x] = b'R';
                let r = right_at[x];
                right[r].remove(&x);
                let y = *right[r].iter().next().unwrap();
                right[r].remove(&y);
                ans[y] = b'B';
                let l = left_at[y];
                left[l].remove(&y);
                if left[l].len() % 2 == 0 {
                    break;
                }
                x = *left[l].iter().next().unwrap();
                left[l].remove(&x);
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
    let test_type = TestType::MultiNumber;
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
    //    tester::stress_test();
}
//END MAIN
