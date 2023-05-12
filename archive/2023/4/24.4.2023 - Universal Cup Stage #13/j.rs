//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_long();
    let mut k = input.read_long();

    if m == 1 {
        if k > 1 {
            out_line!(-1);
            return;
        }
        out_line!(vec![1; n]);
        return;
    }
    if m == 2 {
        let k = k.into_usize();
        match n {
            1 => {
                if k > 2 {
                    out_line!(-1);
                    return;
                }
                out_line!(k);
                return;
            }
            2 => {
                if k > 2 {
                    out_line!(-1);
                    return;
                }
                out_line!(k, 3 - k);
                return;
            }
            3 => {
                let mut s = ["211", "121", "221", "112", "212", "122"];
                s.sort();
                if k > s.len() {
                    out_line!(-1);
                    return;
                }
                out_line!(s[k - 1].chars().collect_vec());
            }
            4 => {
                let mut s = [
                    "1211", "2211", "1121", "2121", "1221", "2112", "1212", "2212", "1122", "2122",
                ];
                s.sort();
                if k > s.len() {
                    out_line!(-1);
                    return;
                }
                out_line!(s[k - 1].chars().collect_vec());
            }
            _ => {
                if k > 12 {
                    out_line!(-1);
                    return;
                }
                let mut s = Vec::new();
                while s.len() < n + 6 {
                    s.push('1');
                    s.push('2');
                    s.push('1');
                    s.push('1');
                    s.push('2');
                    s.push('2');
                }
                let mut t = Vec::new();
                while t.len() < n + 6 {
                    t.push('2');
                    t.push('1');
                    t.push('2');
                    t.push('2');
                    t.push('1');
                    t.push('1');
                }
                let mut ans = Vec::with_capacity(12);
                for i in 0..6 {
                    ans.push(s[i..i + n].to_vec());
                    ans.push(t[i..i + n].to_vec());
                }
                ans.sort();
                out_line!(ans[k - 1]);
            }
        }
        return;
    }
    if n == 1 {
        if k > m {
            out_line!(-1);
            return;
        }
        out_line!(k);
        return;
    }
    let mut p = Vec::with_capacity(n + 1);
    let mut cur = 1i64;
    for _ in 0..=n {
        p.push(cur);
        cur = cur.saturating_mul(m - 2);
    }
    if k > p[n - 2].saturating_mul(m - 1).saturating_mul(m) {
        out_line!(-1);
        return;
    }
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        let sl = if i == 0 {
            p[n - 2].saturating_mul(m - 1)
        } else {
            p[n - i - 1]
        };
        let mut next = (k - 1) / sl;
        k -= next * sl;
        next += 1;
        if i == 1 && next >= ans[0] {
            next += 1;
        } else if i > 1 {
            if next >= ans[i - 1].min(ans[i - 2]) {
                next += 1;
            }
            if next >= ans[i - 1].max(ans[i - 2]) {
                next += 1;
            }
        }
        ans.push(next);
    }
    out_line!(ans);
}

#[test]
fn test() {
    use algo_lib::collections::min_max::MinimMaxim;
    use algo_lib::numbers::num_traits::bit_ops::BitOps;
    for n in 1..=10 {
        let mut min = None;
        let mut at = Vec::new();
        for j in 0..(1 << n) {
            let mut cur = 0;
            for k in 0..n {
                for l in 1..n {
                    if k < l || k + l >= n || j.is_set(k - l) != j.is_set(k + l) {
                        break;
                    }
                    cur += 1;
                }
                for l in 0..n {
                    if k < l || k + l + 1 >= n || j.is_set(k - l) != j.is_set(k + l + 1) {
                        break;
                    }
                    cur += 1;
                }
            }
            if min.minim(cur) {
                at.clear();
            }
            if min == Some(cur) {
                at.push(j);
            }
        }
        println!("{}: {}", n, min.unwrap());
        for j in at {
            for k in 0..n {
                if j.is_set(k) {
                    print!("2");
                } else {
                    print!("1");
                }
            }
            println!();
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
