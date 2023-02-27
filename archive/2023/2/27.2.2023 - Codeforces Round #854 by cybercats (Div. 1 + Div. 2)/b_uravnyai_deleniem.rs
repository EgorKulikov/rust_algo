//{"name":"B. Уравняй делением","group":"Codeforces - Codeforces Round #854 by cybercats (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1799/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n1\n100\n3\n1 1 1\n2\n2 1\n2\n5 5\n3\n4 3 2\n4\n3 3 4 4\n2\n2 100\n5\n5 3 6 7 8\n6\n3 3 80 3 8 3\n4\n19 40 19 55\n","output":"0\n0\n-1\n0\n2\n1 3\n2 1\n4\n3 1\n4 2\n1 3\n2 4\n6\n2 1\n2 1\n2 1\n2 1\n2 1\n2 1\n8\n5 2\n4 2\n3 2\n1 3\n1 3\n2 1\n4 1\n5 1\n4\n5 1\n3 1\n3 1\n3 1\n9\n4 2\n2 1\n1 2\n1 2\n3 2\n3 2\n1 4\n2 4\n3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BUravnyaiDeleniem"}}}

use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut a = input.read_size_vec(n);

    if a.iter().count_eq(&&a[0]) == n {
        out_line!(0);
        return;
    }
    if a.iter().count_eq(&&1) > 0 {
        out_line!(-1);
        return;
    }
    let mut ans = Vec::new();
    loop {
        let mut found = false;
        for i in 1..n {
            if a[0] != a[i] {
                found = true;
                if a[0] > a[i] {
                    a[0] = (a[0] + a[i] - 1) / a[i];
                    ans.push((1, i + 1));
                } else {
                    a[i] = (a[i] + a[0] - 1) / a[0];
                    ans.push((i + 1, 1));
                }
            }
        }
        if !found {
            break;
        }
    }
    out_line!(ans.len());
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
