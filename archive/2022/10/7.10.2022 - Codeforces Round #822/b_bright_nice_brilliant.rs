//{"name":"B. Bright, Nice, Brilliant","group":"Codeforces - Codeforces Round #822 (Div. 2)","url":"http://codeforces.com/contest/1734/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1\n2\n3\n","output":"1\n1\n1 1\n1\n1 1\n1 0 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBrightNiceBrilliant"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();

    let mut br = Arr2d::new(n, n, 0);
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        let min = *br.row(i).take(i + 1).min().unwrap();
        let mut cur = Vec::with_capacity(i + 1);
        for j in 0..=i {
            if br[(i, j)] == min {
                cur.push(1);
                for k in i..n {
                    for l in j..=(j + k - i) {
                        br[(k, l)] += 1;
                    }
                }
            } else {
                cur.push(0);
            }
        }
        ans.push(cur);
    }
    output().print_per_line(&ans);
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
