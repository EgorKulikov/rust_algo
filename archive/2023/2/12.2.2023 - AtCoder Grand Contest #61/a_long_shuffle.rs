//{"name":"A - Long Shuffle","group":"AtCoder - AtCoder Grand Contest 061","url":"https://atcoder.jp/contests/agc061/tasks/agc061_a","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n2 1\n2 2\n5 1\n5 2\n5 3\n5 4\n5 5\n","output":"2\n1\n2\n4\n1\n5\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ALongShuffle"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;
use std::collections::HashMap;

#[test]
fn test2() {
    let mut map = HashMap::new();

    for i in 1i128.. {
        if i % 1000 == 0 {
            println!("{}", i);
        }
        for j in 1..=i {
            let s = i.pow(5) + j.pow(5);
            if let Some(&(k, l)) = map.get(&s) {
                println!("{} {} {} {} {}", i, j, k, l, s);
            } else {
                map.insert(s, (i, j));
            }
        }
    }
}

fn f(mut n: usize, k: usize) -> usize {
    assert_eq!(n % 2, 0);
    let mut mk = k;
    if mk >= n / 2 {
        mk = n - mk - 1;
    }
    mk /= 2;
    n /= 2;
    for i in (0..60).rev() {
        if n.is_set(i) && n.count_ones() > 1 {
            if mk.is_set(i) {
                mk.unset_bit(i);
            }
            n.unset_bit(i);
        }
    }
    if mk < n {
        k ^ 1
    } else {
        k
    }
    // let mut p = 1;
    // while n % (2 * p) == 0 {
    //     p *= 2;
    // }
    // if k < p || k >= n - p {
    //     k ^ 1
    // } else {
    //     k
    // }
}

fn solve(input: &mut Input) {
    let t = input.read_size();

    for _ in 0..t {
        let n = input.read_size();
        let mut k = input.read_size() - 1;

        if n % 2 != 0 {
            if k > 0 {
                k = f(n - 1, k - 1) + 1;
            }
            if k < n - 1 {
                k = f(n - 1, k);
            }
        } else {
            k = f(n, k);
        }
        out_line!(k + 1);
    }
}

#[test]
fn test() {
    use algo_lib::collections::iter_ext::IterExt;
    let mut cur = vec![1, 0];
    for i in 3..=50000 {
        let mut next = (0..i).collect_vec();
        for j in 0..i - 1 {
            next[j] = cur[j];
        }
        let mut n_cur = next.clone();
        for j in 0..i - 1 {
            n_cur[j + 1] = next[cur[j] + 1];
        }
        // println!("{i} {:?}", n_cur);
        if i % 2 == 0 {
            // let mut p = 1;
            // while i % (2 * p) == 0 {
            //     p *= 2;
            // }
            // let mut r = Vec::new();
            // for j in (0..i).step_by(2) {
            //     if n_cur[j] != j {
            //         r.push(j / 2);
            //     }
            // }
            for j in 0..i {
                assert_eq!(n_cur[j], f(i, j), "{i} {j}");
            }
            // assert_eq!(r.len() % 2, 0);
            // for l in 0..r.len() / 2 {
            //     let j = r[l];
            //     let k = r[r.len() - l - 1];
            //     assert_eq!(j + k, i - 2);
            // }
            // println!("{} {:?}", i / 2, r);
        }
        cur = n_cur;
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
