//{"name":"F. Shrink-Reverse","group":"Codeforces - Educational Codeforces Round 162 (Rated for Div. 2)","url":"https://codeforces.com/contest/1923/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"8 2\n10010010\n","output":"7\n"},{"input":"8 2\n01101000\n","output":"7\n"},{"input":"30 30\n111111111111111111111111111111\n","output":"73741816\n"},{"input":"14 1\n10110001111100\n","output":"3197\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FShrinkReverse"}}}

use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::number_ext::Power;
use algo_lib::string::hash::{HashBase, SimpleHash, StringHash};
use algo_lib::string::str::StrReader;
use std::cmp::Ordering;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let s = input.read_str();

    let ones = s.iter().filter(|&c| c == b'1').count();
    type Mod = ModInt7;
    if ones <= k {
        out.print_line(Mod::new(2).power(ones) - Mod::one());
        return;
    }

    fn compare(
        h1: &impl StringHash,
        f1: usize,
        t1: usize,
        h2: &impl StringHash,
        f2: usize,
        t2: usize,
    ) -> Ordering {
        if t1 - f1 != t2 - f2 {
            return (t1 - f1).cmp(&(t2 - f2));
        }
        if h1.hash(f1..=t1) == h2.hash(f2..=t2) {
            return Ordering::Equal;
        }
        let mut left = 0;
        let mut right = t1 - f1;
        while left < right {
            let mid = (left + right) / 2;
            if h1.hash(f1..=f1 + mid) == h2.hash(f2..=f2 + mid) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if h1.hash(f1 + left..=f1 + left) == 0 {
            Ordering::Less
        } else {
            assert_eq!(h2.hash(f2 + left..=f2 + left), 0);
            Ordering::Greater
        }
    }

    HashBase::init();
    let mut s = s.iter().map(|c| (c - b'0') as i64).collect::<Vec<_>>();
    let base_hash = SimpleHash::new(&s);
    let mut so_far = 0;
    let mut left = 0;
    let mut right = n - 1;
    for i in 0..n {
        if s[i] == 1 {
            so_far += 1;
            if so_far == k + 1 {
                left = i;
                break;
            }
        }
    }
    s.reverse();
    let rev_hash = SimpleHash::new(&s);
    let mut hash = &base_hash;
    let mut pos_one = Vec::new();
    for i in 0..n {
        if s[i] == 1 {
            pos_one.push(i);
        }
    }
    let mut use_rev = false;
    let delta = ones - k;
    for i in 0..ones - delta {
        let cand_left = pos_one[i];
        let cand_right = pos_one[i + delta];
        if compare(hash, left, right, &rev_hash, cand_left, cand_right) == Ordering::Greater {
            use_rev = true;
            left = cand_left;
            right = cand_right;
            hash = &rev_hash;
        }
    }
    if !use_rev {
        s.reverse();
    }
    let mut ans = Vec::new();
    for i in left..=right {
        ans.push(s[i]);
    }
    ans.reverse();
    let mut remaining = ones - ans.iter().count_eq(&&1);
    assert!(remaining == k - 1 && use_rev || remaining == k && !use_rev);
    for i in 0..ans.len() {
        if remaining == 0 {
            break;
        }
        if ans[i] == 0 {
            ans[i] = 1;
            remaining -= 1;
        }
    }
    while remaining > 0 {
        ans.push(1);
        remaining -= 1;
    }

    let mut res = Mod::zero();
    let mut pw = Mod::one();
    for c in ans {
        if c == 1 {
            res += pw;
        }
        pw *= Mod::new(2);
    }
    out.print_line(res);
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
                solve(&mut input, &mut output, i + 1, &pre_calc);
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
