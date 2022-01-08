//{"name":"Velocities of balls","group":"HackerEarth - January Easy '22","url":"https://www.hackerearth.com/challenges/competitive/january-easy-22/algorithm/bouncing-balls-b9c19a3d/","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n0 2\n-1 1\n2\n0 2\n1 -1\n3\n-5 1 5\n1 1 -2\n5\n8 4 0 -3 2\n-3 2 1 1 -4\n","output":"0\n0\n1\n1\n3\n4\n1\n0\n2\n3\n1\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"VelocitiesOfBalls"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::zip;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let x = input.read_int_vec(n);
    let v = input.read_int_vec(n);

    let mut ans = vec![0; n];
    let mut balls = zip!(x.into_iter(), v.into_iter(), 0..n).collect_vec();
    balls.sort_by_key(|(x, v, _)| (*x, -*v));
    let mut neg = (0..n).rev().filter(|i| balls[*i].1 < 0).collect_vec();
    let mut pos = Vec::with_capacity(n);
    for (&i, &j) in neg.consecutive_iter() {
        assert!(balls[i].1 >= balls[j].1);
    }
    for (k, (x, v, i)) in balls.iter().enumerate() {
        let mut x = *x;
        let mut v = *v;
        let or_v = v;
        if v < 0 {
            assert_eq!(neg.pop(), Some(k));
        }
        let mut neg_at = neg.len().checked_sub(1);
        let mut pos_at = pos.len().checked_sub(1);
        loop {
            if v > 0 {
                match neg_at {
                    None => break,
                    Some(j) => {
                        let (nx, nv, _) = balls[neg[j]];
                        ans[*i] += (nx - x) / (v - nv);
                        x = nx;
                        v = nv;
                        neg_at = j.checked_sub(1);
                    }
                }
            } else {
                match pos_at {
                    None => break,
                    Some(j) => {
                        let (nx, nv, _) = balls[pos[j]];
                        ans[*i] += (x - nx) / (nv - v);
                        x = nx;
                        v = nv;
                        pos_at = j.checked_sub(1);
                    }
                }
            }
        }
        if or_v > 0 {
            pos.push(k);
        }
    }
    for (&i, &j) in pos.consecutive_iter() {
        assert!(balls[i].1 <= balls[j].1);
    }
    output().print_per_line(&ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
