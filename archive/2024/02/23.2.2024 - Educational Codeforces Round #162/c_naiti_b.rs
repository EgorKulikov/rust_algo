//{"name":"C. Найти B","group":"Codeforces - Educational Codeforces Round 162 (Rated for Div. 2)","url":"https://codeforces.com/contest/1923/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"1\n5 4\n1 2 1 4 5\n1 5\n4 4\n3 4\n1 3\n","output":"YES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNaitiB"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let c = input.read_size_vec(n);

    let s = c.partial_sums();
    let ones = c
        .iter()
        .map(|&x| if x == 1 { 1 } else { 0 })
        .collect::<Vec<_>>()
        .partial_sums();

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();

        out.print_line(l + 1 != r && ones[r] - ones[l] + r - l <= s[r] - s[l]);
    }
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
