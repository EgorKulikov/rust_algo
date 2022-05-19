//{"name":"e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"e"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let q = input.read_usize();
    let s: Str = input.read();
    let queries = input.read_usize_pair_vec(q);

    if queries.iter().all(|&(from, _)| from == 1) {
        assert!(false);
        let mut ids = vec![Vec::new(); n];
        for (i, (_, to)) in queries.into_iter().enumerate() {
            ids[to].push(i);
        }
        let mut res = vec![0; q];
        let mut stack = VecDeque::new();
        let mut ans = 0i64;
        let mut last = 0;
        for (i, c) in s.iter().enumerate() {
            if c == b'(' {
                stack.push_back(last);
                last = 0;
            } else {
                if let Some(v) = stack.pop_back() {
                    last = v + 1;
                } else {
                    last = 0;
                }
            }
            ans += last;
            for &j in &ids[i] {
                res[j] = ans;
            }
        }
        output().print_per_line(&res);
        return;
    }
    if q > 200 {
        let mut ss = Vec::with_capacity(n + 1);
        let mut doubles = Vec::with_capacity(n + 1);
        let mut cur = 0i64;
        ss.push(cur);
        let mut cur_doubles = 0;
        doubles.push(cur_doubles);
        for i in 0..n - 1 {
            if s[i] == b')' && s[i + 1] == b'(' {
                cur += 1;
            }
            ss.push(cur);
        }
        for i in 0..n {
            if i > 0 && s[i] == b')' && s[i - 1] == b')' {
                cur_doubles += 1;
            }
            doubles.push(cur_doubles);
        }
        cur += 1;
        ss.push(cur);
        for (mut from, mut to) in queries {
            from -= 1;
            let mut ans = 0;
            if from > 0 && s[from] == b'(' && s[from - 1] == b'(' {
                if to >= from + 2 {
                    ans += 1;
                }
                from += 3;
            }
            while from < n && s[from] == b')' {
                from += 1;
            }
            if to < n && s[to - 1] == b')' && s[to] == b')' {
                if to >= from + 2 {
                    ans += 1;
                }
                to -= 3;
            }
            while to > 0 && s[to - 1] == b'(' {
                to -= 1;
            }
            let delta = if to > from { ss[to] - ss[from] } else { 0 };
            if to > from {
                ans += doubles[to] - doubles[from];
            }
            ans += delta * (delta + 1) / 2;
            out_line!(ans);
        }
        return;
    }
    for (mut from, to) in queries {
        from -= 1;
        let mut stack = VecDeque::new();
        let mut ans = 0i64;
        let mut last = 0;
        for &c in &s[from..to] {
            if c == b'(' {
                stack.push_back(last);
                last = 0;
            } else {
                if let Some(v) = stack.pop_back() {
                    last = v + 1;
                } else {
                    last = 0;
                }
            }
            ans += last;
        }
        out_line!(ans);
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
