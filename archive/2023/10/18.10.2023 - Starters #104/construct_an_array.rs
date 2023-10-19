//{"name":"Construct An Array","group":"CodeChef - START104A","url":"https://www.codechef.com/START104A/problems/CONSTANARRAY","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 1\n1\n3 2\n1 1 1\n3 2\n1 1 1\n4 4\n1 2 3 4\n","output":"1\n1 1 1\n1 2 1\n1 2 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ConstructAnArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n);

    let mut free = (1..=m).collect::<HashSet<_>>();
    let mut ans = Vec::with_capacity(n);
    for i in a {
        if free.is_empty() {
            free = (1..=m).collect();
        }
        if free.contains(&i) {
            ans.push(i);
            free.remove(&i);
        } else {
            let cur = *free.iter().next().unwrap();
            ans.push(cur);
            free.remove(&cur);
        }
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
