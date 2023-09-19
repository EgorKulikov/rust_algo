//{"name":"A. МЕХанизированный массив","group":"Codeforces - CodeTON Round 6 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1870/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"9\n5 3 3\n4 7 5\n4 2 28\n12 10 6\n57 51 122\n200 1 200\n2 2 1\n3 2 1\n4 7 10\n","output":"7\n-1\n57\n-1\n2007\n39800\n1\n2\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMEKhanizirovanniiMassiv"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let x = input.read_size();

    if k > x + 1 || k > n {
        out_line!(-1);
        return;
    }

    let ans = k * (k - 1) / 2 + (n - k) * if x == k { x - 1 } else { x };
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
