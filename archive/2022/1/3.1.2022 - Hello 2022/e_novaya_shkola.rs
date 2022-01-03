//{"name":"E. Новая школа","group":"Codeforces - Hello 2022","url":"https://codeforces.com/contest/1621/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n1 1\n30\n3\n25 16 37\n4 2\n9 12 12 6\n2\n4 5\n3\n111 11 11\n","output":"101\n00100\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ENovayaShkola"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::rational::Rational;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let mut a: Vec<u64> = input.read_vec(n);

    a.sort_unstable();
    a.reverse();
    a.resize_with(m, || unreachable!());
    a.reverse();

    let mut groups = Vec::with_capacity(m);
    for _ in 0..m {
        let k = input.read_usize();
        let b: Vec<u64> = input.read_vec(k);
        let s = b.iter().sum::<u64>();
        groups.push((Rational::new(s, k.into_u64()), s, k, b));
    }
    let mut ratio = groups.iter().map(|(r, _, _, _)| *r).collect_vec();
    ratio.sort();
    let mut same = BitSet::new(m);
    let mut up = BitSet::new(m);
    let mut down = BitSet::new(m);
    for i in 0..m {
        if ratio[i] <= Rational::new(a[i], 1) {
            same.set(i, true);
        }
        if i > 0 && ratio[i] <= Rational::new(a[i - 1], 1) {
            up.set(i, true);
        }
        if i < m - 1 && ratio[i] <= Rational::new(a[i + 1], 1) {
            down.set(i, true);
        }
    }
    fn count(b: &BitSet) -> Vec<usize> {
        let mut res = Vec::with_capacity(b.len() + 1);
        res.push(0);
        for i in 0..b.len() {
            res.push(*res.last().unwrap() + if b[i] { 1 } else { 0 });
        }
        res
    }
    let c_same = count(&same);
    let c_up = count(&up);
    let c_down = count(&down);
    let mut ans = Str::new();
    for (r, s, k, b) in groups {
        let pos = ratio.as_slice().lower_bound(&r);
        for v in b {
            let cr = Rational::new(s - v, (k - 1).into_u64());
            let mut n_pos = ratio.as_slice().lower_bound(&cr);
            if n_pos > pos {
                n_pos -= 1;
            }
            let l = pos.min(n_pos);
            let r = pos.max(n_pos) + 1;
            if c_same[l] != l || c_same[m] - c_same[r] != m - r || cr > Rational::new(a[n_pos], 1) {
                ans += b'0';
                continue;
            }
            if n_pos < pos && c_down[pos] - c_down[n_pos] != pos - n_pos {
                ans += b'0';
                continue;
            }
            if n_pos > pos && c_up[n_pos + 1] - c_up[pos + 1] != n_pos - pos {
                ans += b'0';
                continue;
            }
            ans += b'1';
        }
    }
    out_line!(ans);
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
