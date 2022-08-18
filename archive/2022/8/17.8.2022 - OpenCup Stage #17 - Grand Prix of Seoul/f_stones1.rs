//{"name":"F. Stones 1","group":"Yandex - Stage 17: Grand Prix of Seoul","url":"https://official.contest.yandex.com/opencupXXII/contest/39021/problems/F/","interactive":false,"timeLimit":1000,"tests":[{"input":"4\nWBWB\n6 4 5 3\n","output":"5\n"},{"input":"8\nWBBWBWBB\n6 4 8 2 5 3 1 5\n","output":"13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FStones1"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s: Str = input.read();
    let a = input.read_long_vec(n);

    let mut vals = Vec::new();
    let mut leftmost = true;
    let mut cur_max = 0;
    for i in 1..n {
        if s[i] != s[i - 1] {
            if !leftmost {
                vals.push(cur_max);
            }
            leftmost = false;
            cur_max = 0;
        }
        cur_max.maxim(a[i]);
    }
    vals.sort_by_key(|&it| -it);
    let vals_len = vals.len();
    out_line!(vals.into_iter().take((vals_len + 1) / 2).sum::<i64>());
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
