//{"name":"G. Одномерный пазл","group":"Codeforces - Codeforces Round 925 (Div. 3)","url":"https://codeforces.com/contest/1931/problem/G","interactive":false,"timeLimit":4000,"tests":[{"input":"11\n1 1 1 1\n1 2 5 10\n4 6 100 200\n900000 900000 900000 900000\n0 0 0 0\n0 0 566 239\n1 0 0 0\n100 0 100 0\n0 0 0 4\n5 5 0 2\n5 4 0 5\n","output":"4\n66\n0\n794100779\n1\n0\n1\n0\n1\n36\n126\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GOdnomerniiPazl"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::algebra::Zero;

type Mod = ModIntF;
type PreCalc = Combinations<Mod>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &PreCalc) {
    let c1 = input.read_size();
    let c2 = input.read_size();
    let c3 = input.read_size();
    let c4 = input.read_size();

    if c1 == 0 && c2 == 0 {
        if c3 != 0 && c4 != 0 {
            out.print_line(0);
        } else {
            out.print_line(1);
        }
        return;
    }

    let mut ans = Mod::zero();
    for i in 0..2 {
        for j in 0..2 {
            if c1 + i != c2 + j {
                continue;
            }
            ans += data.c(c2 + c4 - i, c1 - j) * data.c(c1 + c3 + i - 1, c2 + j - 1);
        }
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = Combinations::new(2000001);

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
