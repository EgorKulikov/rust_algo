//{"name":"The Last Sweet Game","group":"CodeChef - COSS2023","url":"https://www.codechef.com/COSS2023/problems/TLSG","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1\n2\n5\n12\n","output":"Alice\nBob\nBob\nBob\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TheLastSweetGame"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = BitSet;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &PreCalc) {
    let n = input.read();

    out.print_line(if data[n] { "Bob" } else { "Alice" });
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = BitSet::new(40001);
    pre_calc.set(0);
    for i in 2..=40000 {
        let mut j = 1;
        while j * j < i {
            if !pre_calc[i - j * j] {
                pre_calc.set(i);
                break;
            }
            j += 1;
        }
    }

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
            for i in 0usize..t {
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
