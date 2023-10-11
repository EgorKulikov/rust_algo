//{"name":"A. Шлемы в светлой ночи","group":"Codeforces - Codeforces Round 902 (Div. 1, based on COMPFEST 15 - Final Round)","url":"https://codeforces.com/contest/1876/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n6 3\n2 3 2 1 1 3\n4 3 2 6 3 6\n1 100000\n100000\n1\n4 94\n1 4 2 3\n103 96 86 57\n","output":"16\n100000\n265\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AShlemiVSvetloiNochi"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let p = input.read_size();
    let a = input.read_size_vec(n);
    let b = input.read_size_vec(n);

    let people = b.into_iter().zip(a.into_iter()).collect_vec().sorted();
    let mut ans = p;
    let mut rem = n - 1;
    for (b, a) in people {
        if b >= p {
            ans += rem * p;
            break;
        }
        let cur = rem.min(a);
        ans += cur * b;
        rem -= cur;
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
