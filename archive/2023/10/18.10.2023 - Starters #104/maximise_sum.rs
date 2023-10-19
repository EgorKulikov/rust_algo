//{"name":"Maximise Sum","group":"CodeChef - START104A","url":"https://www.codechef.com/START104A/problems/MAXIMISESUM","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\n2 3\n3\n5 1 7\n4\n5 5 5 5\n","output":"5\n17\n20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MaximiseSum"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut from_left = Vec::with_capacity(n);
    let mut val = a[0];
    for &i in &a {
        val.maxim(i);
        from_left.push(val);
    }
    let mut from_right = Vec::with_capacity(n);
    val = a[n - 1];
    for &i in a.iter().rev() {
        val.maxim(i);
        from_right.push(val);
    }
    from_right.reverse();
    let ans: i64 = from_left
        .into_iter()
        .zip(from_right.into_iter())
        .map(|(l, r)| l.min(r))
        .sum();
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
