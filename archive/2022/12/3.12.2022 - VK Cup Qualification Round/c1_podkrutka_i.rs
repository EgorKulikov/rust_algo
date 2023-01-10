//{"name":"C1. Подкрутка I","group":"Codeforces - VK Cup 2022 - Квалификация (Engine)","url":"https://codeforces.com/contest/1769/problem/C1","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n9\n1 1 3 4 6 6 6 8 10\n6\n1 2 3 4 5 6\n5\n10 10 10 10 10\n","output":"5\n6\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"C1PodkrutkaI"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut start = -2;
    let mut end = -2;
    let mut dense = false;
    let mut ans = 1;
    let mut last_start = -2;
    for i in a {
        if i > end + 1 {
            ans.maxim(end - start + if dense { 2 } else { 1 });
            if i - end == 2 {
                if !dense {
                    start = last_start + 1;
                }
            } else {
                start = i;
            }
            dense = false;
            last_start = i;
        }
        if i == end {
            dense = true;
        }
        end = i;
    }
    ans.maxim(end - start + if dense { 2 } else { 1 });
    out_line!(ans);
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
