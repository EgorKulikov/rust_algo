//{"name":"C. World War 1","group":"Codeforces - Coding Challenge Alpha V - by AlgoRave","url":"https://codeforces.com/gym/105005/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n1\n3 4\n5 4\n3 -5 2 9\n13\n3 5\n39 3\n12 -23 43\n","output":"4\n32\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CWorldWar1"}}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let k = input.read_size();
    let l = input.read_size();
    let n = input.read_size();
    let r = input.read_size();
    let m = input.read_size();
    let pw = input.read_int_vec(m);

    let from = if r + k < l { l - (r + k) } else { 0 };
    let to = if l + n + k < r + m {
        (l + n + k).max(r) - r
    } else {
        m
    };
    if from > to {
        out.print_line(pw.into_iter().sum::<i32>());
        return;
    }
    let killable = pw[from..to].to_vec().sorted();
    let mut shots = n;
    let mut delta = 0;
    for i in killable.into_iter().rev() {
        if i <= 0 || shots == 0 {
            break;
        }
        delta += i;
        shots -= 1;
    }
    out.print_line(pw.into_iter().sum::<i32>() - delta);
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
    //    tester::stress_test();
}
//END MAIN
