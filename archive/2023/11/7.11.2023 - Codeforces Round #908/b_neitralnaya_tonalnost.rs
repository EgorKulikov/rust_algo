//{"name":"B. Нейтральная тональность","group":"Codeforces - Codeforces Round 908 (Div. 1)","url":"https://codeforces.com/contest/1893/problem/B","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n2 1\n6 4\n5\n5 5\n1 7 2 4 5\n5 4 1 2 7\n1 9\n7\n1 2 3 4 5 6 7 8 9\n3 2\n1 3 5\n2 4\n10 5\n1 9 2 3 8 1 4 7 2 9\n7 8 5 4 6\n2 1\n2 2\n1\n6 1\n1 1 1 1 1 1\n777\n","output":"6 5 4\n1 1 7 7 2 2 4 4 5 5\n9 8 7 7 6 5 4 3 2 1\n1 3 5 2 4\n1 9 2 3 8 8 1 4 4 7 7 2 9 6 5\n2 2 1\n777 1 1 1 1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BNeitralnayaTonalnost"}}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_vec(n);
    let mut b = input.read_int_vec(m).sorted();

    let mut ans = Vec::new();
    for &i in &a {
        while let Some(&last) = b.last() {
            if last < i {
                break;
            }
            b.pop();
            ans.push(last);
        }
        ans.push(i);
    }
    b.reverse();
    ans.extend_from_slice(&b);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
