//{"name":"F. Сумма и произведение","group":"Codeforces - Codeforces Round 891 (Div. 3)","url":"https://codeforces.com/contest/1857/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n3\n1 3 2\n4\n3 2\n5 6\n3 1\n5 5\n4\n1 1 1 1\n1\n2 1\n6\n1 4 -2 3 3 3\n3\n2 -8\n-1 -2\n7 12\n","output":"1 1 0 0\n6\n1 1 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSummaIProizvedenie"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::integer_sqrt::IntegerSqrt;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut qty = DefaultHashMap::<_, usize>::new();
    for i in a {
        qty[i] += 1;
    }

    let q = input.read_size();
    for _ in 0..q {
        let b = -input.read_long();
        let c = input.read_long();

        let d = b * b - 4 * c;
        if d < 0 {
            out_line!(0);
            continue;
        }
        if let Some(d) = d.sqrt() {
            let x1 = (-b - d) / 2;
            let x2 = (-b + d) / 2;
            if x1 != x2 {
                out_line!(qty[x1] * qty[x2]);
            } else if qty[x1] > 0 {
                out_line!(qty[x1] * (qty[x1] - 1) / 2);
            } else {
                out_line!(0);
            }
        } else {
            out_line!(0);
        }
    }
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
