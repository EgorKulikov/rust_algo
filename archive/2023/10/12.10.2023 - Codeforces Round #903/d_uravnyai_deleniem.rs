//{"name":"D. Уравняй делением","group":"Codeforces - Codeforces Round 903 (Div. 3)","url":"https://codeforces.com/contest/1881/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n5\n100 2 50 10 1\n3\n1 1 1\n4\n8 2 4 2\n4\n30 50 27 20\n2\n75 40\n2\n4 4\n3\n2 3 1\n","output":"YES\nYES\nNO\nYES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DUravnyaiDeleniem"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::primes::sieve::divisor_table;

type PreCalc = Vec<usize>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut qty = DefaultHashMap::<_, usize>::new();
    for mut i in a {
        while i != 1 {
            qty[data[i]] += 1;
            i /= data[i];
        }
    }
    for i in qty.into_values() {
        if i % n != 0 {
            out.print_line(false);
            return;
        }
    }
    out.print_line(true);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = divisor_table(1000001);

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
