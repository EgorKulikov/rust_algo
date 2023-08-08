//{"name":"D. Сильные вершины","group":"Codeforces - Codeforces Round 891 (Div. 3)","url":"https://codeforces.com/contest/1857/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4\n3 1 2 4\n4 3 2 1\n5\n1 2 4 1 2\n5 2 3 3 1\n2\n1 2\n2 1\n3\n0 2 1\n1 3 2\n3\n5 7 4\n-2 -3 -6\n","output":"1\n4\n2\n3 5\n1\n2\n3\n1 2 3\n2\n2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSilnieVershini"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);

    let d = a
        .into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| a - b)
        .collect_vec();
    let m = *d.iter().max().unwrap();
    out_line!(d.iter().count_eq(&&m));
    out_line!(d
        .into_iter()
        .enumerate()
        .filter_map(|(i, d)| if d == m { Some(i + 1) } else { None })
        .collect_vec());
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
