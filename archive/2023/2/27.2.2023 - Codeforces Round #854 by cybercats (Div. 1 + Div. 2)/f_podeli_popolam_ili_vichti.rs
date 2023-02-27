//{"name":"F. Подели пополам или вычти","group":"Codeforces - Codeforces Round #854 by cybercats (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1799/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n3 2 1 1\n9 3 5\n2 1 2 0\n1000000000 1\n5 3 1 1\n2 8 3 19 3\n6 9 4 2\n1 2 3 4 5 6\n3 10 3 3\n1 2 3\n5 1 0 0\n999999999 999999999 999999999 999999999 999999999\n5 5 4 3\n5 9 10 7 4\n","output":"11\n500000001\n23\n6\n0\n4999999995\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FPodeliPopolamIliVichti"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let b = input.read_size();
    let mut k1 = input.read_size();
    let mut k2 = input.read_size();
    let mut a = input.read_size_vec(n);

    a.sort();
    a.reverse();
    let mut v1 = Vec::new();
    // let mut v2 = Vec::new();
    for i in 0..n {
        if k1 == 0 {
            if k2 == 0 {
                break;
            }
            a[i] = a[i].saturating_sub(b);
            k2 -= 1;
        } else {
            if k2 == 0 {
                a[i] = (a[i] + 1) / 2;
                k1 -= 1;
                continue;
            }
            if a[i] >= 2 * b {
                a[i] = (a[i] + 1) / 2;
                a[i] -= b;
                k1 -= 1;
                k2 -= 1;
            // } else if a[i] >= b {
            //     v1.push(a[i]);
            //     a[i] = 0;
            } else {
                // v2.push((a[i] / 2, a[i]));
                v1.push(a[i]);
                a[i] = 0;
            }
        }
    }
    let base: usize = a.into_iter().sum();
    /*let add = if k2 >= v1.len() {
        for i in v1 {
            v2.push((i - b, i - b));
            k2 -= 1;
        }
        v2.sort();
        v2.reverse();
        let mut ans = 0;
        for (i, j) in v2 {
            if k1 > 0 && i == j {
                k1 -= 1;
            } else if k2 > 0 {
                k2 -= 1;
            } else if k1 > 0 {
                k1 -= 1;
                ans += j - i;
            } else {
                ans += j;
            }
        }
        ans
    } else {
        let mut best = None;
        for i in 0..=k2 {
            let mut v2 = v2.clone();
            for (j, &v) in v1.iter().enumerate() {
                if j < i || v1.len() - j <= k2 - i {
                    v2.push((v - b, v - b))
                } else {
                    v2.push((v / 2, v));
                }
            }
            v2.sort();
            v2.reverse();
            let mut ans = 0;
            let mut k1 = k1;
            for (i, j) in v2 {
                if k1 > 0 {
                    k1 -= 1;
                    ans += j - i;
                } else {
                    ans += j;
                }
            }
            best.minim(ans);
        }
        best.unwrap()
    };*/
    let add = {
        let mut best = None;
        for i in 0..=k1.min(k2) {
            if v1.len() + i < k1 + k2 {
                continue;
            }
            let mut cur: usize = v1.iter().skip(i).sum();
            let mut c = Vec::with_capacity(k1 + k2 - i);
            for j in i..k1 + k2 - i {
                c.push((v1[j] / 2, v1[j].min(b)));
            }
            c.sort_by_key(|&(a, b)| a.into_isize() - b.into_isize());
            for j in 0..k2 - i {
                cur -= c[j].1;
            }
            for j in k2 - i..k1 + k2 - 2 * i {
                cur -= c[j].0;
            }
            best.minim(cur);
        }
        best.unwrap()
    };
    out_line!(base + add);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
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
