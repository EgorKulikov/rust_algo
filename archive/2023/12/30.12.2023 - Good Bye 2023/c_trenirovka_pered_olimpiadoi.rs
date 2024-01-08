//{"name":"C. Тренировка перед олимпиадой","group":"Codeforces - Good Bye 2023","url":"https://codeforces.com/contest/1916/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1\n31\n6\n6 3 7 2 5 4\n3\n3 10 11\n5\n7 13 11 19 1\n","output":"31\n6 8 16 18 22 26\n3 12 24\n7 20 30 48 50\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTrenirovkaPeredOlimpiadoi"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut sum = a[0];
    let mut odd = a[0] % 2;
    let mut ans = vec![a[0]];
    for i in a.into_iter().skip(1) {
        sum += i;
        odd += i % 2;
        let delta = odd / 3 + odd % 3 % 2;
        ans.push(sum - delta);
    }
    out.print_line(ans);
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
