//{"name":"Xor Subsequences","group":"HackerEarth - April Circuits '23","url":"https://www.hackerearth.com/challenges/competitive/april-circuits-23/algorithm/xor-subsequences-f7fc94a5/","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 2 3 4\n","output":"1 3 4 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"XorSubsequences"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_unsigned_vec(n);

    let mut base = Vec::new();
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        let mut cur = a[i];
        for &j in &base {
            cur.minim(j ^ cur);
        }
        if cur > 0 {
            base.push(cur);
            base.sort();
            base.reverse();
        }
        if base.len() == i + 1 {
            ans.push((1 << base.len()) - 1);
        } else {
            ans.push(1 << base.len());
        }
    }
    out_line!(ans);
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
