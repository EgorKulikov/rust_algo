//{"name":"E. Финальный отсчёт","group":"Codeforces - Codeforces Round 927 (Div. 3)","url":"https://codeforces.com/contest/1932/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n42\n5\n12345\n2\n99\n4\n0005\n27\n456480697259671309012631002\n","output":"46\n13715\n108\n5\n507200774732968121125145546\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EFinalniiOtschyot"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut ans = Vec::with_capacity(n);
    let mut sum = 0;
    for c in s {
        sum += (c - b'0') as u64;
        ans.push(sum);
    }
    ans.reverse();
    let mut carry = 0;
    for i in 0..n {
        carry += ans[i];
        ans[i] = carry % 10;
        carry /= 10;
    }
    while carry > 0 {
        ans.push(carry % 10);
        carry /= 10;
    }
    while ans.last() == Some(&0) {
        ans.pop();
    }
    ans.reverse();
    for i in ans {
        out.print(i);
    }
    out.print_line(());
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
