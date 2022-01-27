//{"name":"C. Раскрась середину","group":"Codeforces - Codeforces Round #768 (Div. 1)","url":"https://codeforces.com/contest/1630/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n1 2 1 2 7 4 7\n","output":"2\n"},{"input":"13\n1 2 3 2 1 3 3 4 5 5 5 4 7\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRaskrasSeredinu"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n).dec_by_one();

    let mut last = vec![n; n];
    for (i, &a) in a.iter().enumerate() {
        last[a] = i;
    }
    let mut at = 0;
    let mut ans = n;
    while at < n {
        ans -= 1;
        if last[a[at]] != at {
            let mut cur_start = at + 1;
            let mut cur_end = last[a[at]];
            let mut next_end = cur_end;
            loop {
                ans -= 1;
                for i in cur_start..cur_end {
                    next_end.maxim(last[a[i]]);
                }
                if next_end == cur_end {
                    break;
                }
                cur_start = cur_end + 1;
                cur_end = next_end;
            }
            at = cur_end + 1;
        } else {
            at += 1;
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
}
//END MAIN
