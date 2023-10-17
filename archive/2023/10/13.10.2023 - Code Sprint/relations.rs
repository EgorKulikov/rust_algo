//{"name":"Relations","group":"CodeChef - COSS2023","url":"https://www.codechef.com/COSS2023/problems/RLTN","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 1\n2 2\n3 3\n4\n1 1\n1 2\n2 1\n2 2\n6\n1 2\n2 3\n1 3\n2 2\n3 3\n2 1\n7\n1 2\n2 3\n3 4\n4 3\n2 1\n2 4\n1 1\n","output":"Yes 0 0\nYes 2 0\nNo 2 2\nNo 4 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Relations"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_pair_vec(n).dec();

    let mut present = Arr2d::new(n, n, false);
    for &(i, j) in &a {
        present[(i, j)] = true;
    }
    let mut ans1 = true;
    let mut ans2 = 0;
    let mut ans3 = 0;
    for &(i, j) in &a {
        if i == j {
            continue;
        }
        if !present[(i, i)] || !present[(j, j)] {
            ans1 = false;
        }
        if present[(j, i)] {
            ans2 += 1;
        }
        for &(k, l) in &a {
            if j == k && l != j && l != i && present[(i, l)] {
                ans3 += 1;
            }
        }
    }
    out.set_bool_output(BoolOutput::YesNo);
    out.print_line((ans1, ans2, ans3));
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
