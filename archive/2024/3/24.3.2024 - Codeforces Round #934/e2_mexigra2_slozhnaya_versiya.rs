//{"name":"E2. MEX игра - 2 (сложная версия)","group":"Codeforces - Codeforces Round 934 (Div. 1)","url":"https://codeforces.com/contest/1943/problem/E2","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 4\n4 5\n2 1000000000\n1000000000 1000000000 1000000000\n3 2\n2 3 100 1\n1 1\n2 2\n3 1\n1 1 1 1\n","output":"2\n1\n3\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E2MEXIgra2SlozhnayaVersiya"}}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let m = input.read_size();
    let k = input.read_long();
    let f = input.read_long_vec(m + 1);

    let mut left = 0;
    let mut right = m + 1;
    while left < right {
        let mid = (left + right + 1) / 2;
        let f = f[..mid].to_vec().sorted();

        let mut can = false;
        let mut sum = 0;
        let mut max = 0;
        for i in 1..f.len() {
            sum += f[i];
            max += (i as i64) * k;
            if sum <= max {
                can = true;
                break;
            }
        }
        if can {
            right = mid - 1;
        } else {
            left = mid;
        }
    }
    out.print_line(left);
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
