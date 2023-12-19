//{"name":"A - Shuffle and mod K","group":"AtCoder - AtCoder Grand Contest 065","url":"https://atcoder.jp/contests/agc065/tasks/agc065_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n0 1 2\n","output":"6\n"},{"input":"7 123\n11 34 56 0 32 100 78\n","output":"638\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AShuffleAndModK"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::collections::slice_ext::reversed::ReversedSlice;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_long();
    let a = input.read_long_vec(n).sorted();

    let mut max_same = 0;
    let mut cur_same = 1;
    let mut b = Vec::new();
    for (&i, &j) in a.consecutive_iter() {
        if i == j {
            cur_same += 1;
        } else {
            if max_same.maxim(cur_same) {
                b.clear();
            }
            if max_same == cur_same {
                b.push(i);
            }
            cur_same = 1;
        }
    }
    if max_same.maxim(cur_same) {
        b.clear();
    }
    if max_same == cur_same {
        b.push(a.rev()[0]);
    }
    let overs = (n - max_same) as i64;
    let mut delta = k;
    for (&i, &j) in b.consecutive_iter() {
        delta.minim(k - (j - i));
    }
    delta.minim(b.rev()[0] - b[0]);
    let ans = overs * k - delta;
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
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
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
    tester::stress_test(run, tester::check);
}
//END MAIN
