//{"name":"G. Weighing Elimination","group":"Codeforces - Constructor Open Cup 2023","url":"https://constructor2023.contest.codeforces.com/group/sVRDLercWX/contest/431163/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n0 1\n4\n0 1 2 3\n5\n0 0 1 2 1\n5\n0 2 0 0 1\n4\n0 1 1 1\n","output":"YES\n2 1\nNO\nYES\n3 2 4 1 5\nNO\nYES\n1 2 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GWeighingElimination"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let c = input.read_size_vec(n);

    if c[0] > 0 {
        out_line!(false);
        return;
    }
    let mut ans = vec![0; n];
    let mut pos = n;
    for (i, &c) in c.iter().enumerate().rev() {
        if c != 0 {
            if c > pos {
                out_line!(false);
                return;
            }
            pos -= c;
            ans[pos] = i + 1;
        }
    }
    if pos != 1 {
        out_line!(false);
        return;
    }
    ans[0] = 1;
    let mut pos = n;
    let mut max = 0;
    for (i, &c) in c.iter().enumerate().skip(1).rev() {
        if c == 0 {
            while ans[pos - 1] != 0 {
                pos -= 1;
                if ans[pos] < max {
                    out_line!(false);
                    return;
                }
                max = 0;
            }
            pos -= 1;
            ans[pos] = i + 1;
            max.maxim(i + 1);
        }
    }
    let mut cc = vec![0; n];
    let mut cur = ans[0];
    for &i in ans.iter().skip(1) {
        if i > cur {
            cur = i;
        }
        cc[cur - 1] += 1;
    }
    if cc != c {
        out_line!(false);
        return;
    }
    out_line!(true);
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
