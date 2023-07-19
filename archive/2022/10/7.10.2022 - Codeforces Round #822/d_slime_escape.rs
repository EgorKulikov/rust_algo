//{"name":"D. Slime Escape","group":"Codeforces - Codeforces Round #822 (Div. 2)","url":"http://codeforces.com/contest/1734/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n7 4\n-1 -2 -3 6 -2 -3 -1\n3 1\n232 -500 -700\n7 4\n-1 -2 -4 6 -2 -4 -1\n8 4\n-100 10 -7 6 -2 -3 6 -10\n8 2\n-999 0 -2 3 4 5 6 7\n7 3\n7 3 3 4 2 1 1\n","output":"YES\nYES\nNO\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSlimeEscape"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize() - 1;
    let a = input.read_long_vec(n);

    fn process<'a>(it: impl Iterator<Item = &'a i64>) -> VecDeque<(i64, i64)> {
        let mut cur = 0;
        let mut min = 0;
        let mut res = VecDeque::new();
        for &i in it {
            cur += i;
            min.minim(cur);
            if cur >= 0 {
                res.push_back((-min, cur));
                min = 0;
                cur = 0;
            }
        }
        res.push_back((-min, cur));
        res
    }

    let mut right = process(a.iter().skip(k + 1));
    let mut left = process(a.iter().take(k).rev());
    let mut cur = a[k];
    while !left.is_empty() && !right.is_empty() {
        let &(l_req, l_add) = left.front().unwrap();
        if cur >= l_req {
            left.pop_front();
            cur += l_add;
            continue;
        }
        let &(r_req, r_add) = right.front().unwrap();
        if cur >= r_req {
            right.pop_front();
            cur += r_add;
            continue;
        }
        out_line!(false);
        return;
    }
    out_line!(true);
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
