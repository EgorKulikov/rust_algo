//{"name":"F. Сдвиг и разворот","group":"Codeforces - Codeforces Round 913 (Div. 3)","url":"https://codeforces.com/contest/1907/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"11\n5\n3 2 1 5 4\n5\n1 1 2 1 1\n4\n3 7 10 5\n5\n1 2 3 4 5\n2\n5 1\n3\n3 4 1\n5\n4 1 3 4 4\n3\n5 1 1\n4\n2 5 5 4\n5\n2 2 1 1 2\n2\n5 5\n","output":"3\n2\n-1\n0\n1\n1\n3\n1\n2\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSdvigIRazvorot"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::composite_slicelike::SlicelikeZip;
use algo_lib::string::string_algorithms::z_algorithm::ZAlgorithm;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut ans = None;
    let mut b = a.clone().sorted();
    for j in 0..2 {
        let z = b.chain(a.as_slice()).chain(a.as_slice()).z_algorithm();
        for i in 0..n {
            if z[i + n] >= n {
                if j == 0 {
                    ans.minim((i + 2).min((n - i) % n));
                } else {
                    ans.minim(i.min(n - i) + 1);
                }
            }
        }
        b.reverse();
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
