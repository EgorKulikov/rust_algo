//{"name":"P3 - Weaker Data","group":"DMOJ - DMOPC '21 Contest 8","url":"https://dmoj.ca/problem/dmopc21c8p3","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n","output":"2 1 4 3 5\n"},{"input":"4 100\n","output":"-1\n"},{"input":"10 20\n","output":"1 2 3 7 9 5 4 6 8 10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P3WeakerData"}}}

use algo_lib::collections::treap_map::TreapSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut k = input.read_usize();

    let mut cur = 0;
    let mut can = Vec::with_capacity(n);
    for i in 0..n {
        can.push(cur);
        cur += (i / 2) * ((i + 1) / 2);
    }
    if cur < k {
        out_line!(-1);
        return;
    }
    let mut rem = TreapSet::new();
    for i in 0..n {
        rem.insert(i);
    }
    let mut ans = vec![0; n];
    for i in 1..=n {
        let r = n - i;
        let mut left = 0;
        let mut right = r / 2;
        while left < right {
            let mid = (left + right + 1) >> 1;
            if mid * (r - mid) <= k {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        k -= left * (r - left);
        let v = *rem.get_at(left).unwrap();
        rem.remove(&v);
        ans[v] = i;
    }
    out_line!(ans);
    /*for i in (0..n).rev() {
        let mut left = 0;
        let mut right = i;
        while left < right {
            let mid = (left + right) >> 1;
        }
    }*/
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
