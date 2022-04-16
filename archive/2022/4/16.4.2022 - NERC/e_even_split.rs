//{"name":"e_even_split","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"e_even_split"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let l = input.read_int();
    let n = input.read_usize();
    let a = input.read_int_vec(n);

    let mut left = 1;
    let mut right = l / n.into_i32();
    while left < right {
        let mid = (left + right + 1) / 2;
        let mut end = 0;
        let mut good = true;
        for &i in &a {
            if end > i {
                good = false;
                break;
            }
            end += mid;
            end.maxim(i);
        }
        if end > l {
            good = false;
        }
        if good {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    let min = left;

    let mut left = (l + n.into_i32() - 1) / n.into_i32();
    let mut right = l;
    while left < right {
        let mid = (left + right) / 2;
        let mut end = 0;
        let mut good = true;
        for &i in &a {
            end += mid;
            if end < i {
                good = false;
                break;
            }
            end.minim(i + mid);
        }
        if end < l {
            good = false;
        }
        if good {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    let max = left;

    let mut from = 0;
    let mut to = 0;
    let mut f_v = Vec::with_capacity(n + 1);
    let mut t_v = Vec::with_capacity(n + 1);
    f_v.push(0);
    t_v.push(0);
    for &i in &a {
        to.minim(i);
        assert!(from <= to);
        from += min;
        to += max;
        from.maxim(i);
        assert!(from <= to);
        f_v.push(from);
        t_v.push(to);
    }
    assert!(f_v[n] <= l);
    assert!(t_v[n] >= l);
    let mut ans = Vec::with_capacity(n + 1);
    ans.push(l);
    let mut last = l;
    for i in (0..n).rev() {
        let was_last = last;
        last -= max;
        last.maxim(f_v[i]);
        assert!(last <= t_v[i]);
        assert!(was_last - last <= max);
        assert!(was_last - last >= min);
        ans.push(last);
    }
    ans.reverse();
    for (&f, &t) in ans.consecutive_iter() {
        out_line!(f, t);
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
}
//END MAIN
