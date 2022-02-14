//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::mem;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    let mut map: DefaultMap<usize, Vec<usize>> = DefaultMap::new();
    map[0].push(0);
    for (i, a) in a.into_iter().enumerate() {
        map[a].push(i);
    }
    let mut v = map.into_iter().collect_vec();
    v.sort_unstable();
    let v = v.into_iter().map(|(_, v)| v).collect_vec();
    let dist = |mut f: usize, mut t: usize| {
        if f > t {
            mem::swap(&mut f, &mut t);
        }
        (t - f).min(n - (t - f))
    };
    let mut ans = vec![0];
    for (last, next) in v.consecutive_iter() {
        let mut next_ans = Vec::with_capacity(next.len());
        for &i in next {
            let mut res = None;
            for (&p, &a) in last.iter().zip(ans.iter()) {
                res.minim(dist(p, i) + a);
            }
            next_ans.push(res.unwrap());
        }
        if next.len() == 1 {
            ans = next_ans;
        } else {
            ans = Vec::with_capacity(next.len());
            for i in 0..next.len() {
                let mut res = None;
                if i + 1 < next.len() {
                    res.minim(next_ans[i + 1] + n - (next[i + 1] - next[i]));
                } else {
                    res.minim(next_ans[0] + next[i] - next[0]);
                }
                if i > 0 {
                    res.minim(next_ans[i - 1] + n - (next[i] - next[i - 1]));
                } else {
                    res.minim(next_ans[next.len() - 1] + next[next.len() - 1] - next[0]);
                }
                ans.push(res.unwrap());
            }
        }
    }
    out_line!(ans.into_iter().min());
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
