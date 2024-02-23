//{"name":"uc23_k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"uc23_k"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let m = input.read_size();
    let mut a = input.read_long_vec(m);
    let mut b = input.read_long_vec(m);

    let sum_a = a.iter().sum::<i64>();
    let sum_b = b.iter().sum::<i64>();
    a[0] += (sum_b - sum_a).max(0);
    b[0] += (sum_a - sum_b).max(0);
    let mut num_pairs = 0;
    let mut num_pair_types = 0;
    let mut max_a_in_pair = 0;
    let mut max_b_in_pair = 0;
    for i in 0..m {
        let j = m - 1 - i;
        let cur = a[i].min(b[j]);
        if cur > 0 {
            num_pairs += cur;
            num_pair_types += 1;
            max_a_in_pair.maxim(i);
            max_b_in_pair.maxim(j);
            a[i] -= cur;
            b[j] -= cur;
        }
    }
    let mut add = 0;
    let mut at = 0;
    for i in (0..m).rev() {
        at.maxim(m - i);
        while at < m {
            if b[at] != 0 {
                let cur = a[i].min(b[at]);
                add += cur;
                a[i] -= cur;
                b[at] -= cur;
                if a[i] == 0 {
                    break;
                }
            }
            at += 1;
        }
    }
    if add > 0 {
        out.print_line(num_pairs + add);
        return;
    }
    let mut max_a_free = 0;
    for i in (1..m).rev() {
        if a[i] != 0 {
            max_a_free = i;
            break;
        }
    }
    let mut max_b_free = 0;
    for i in (1..m).rev() {
        if b[i] != 0 {
            max_b_free = i;
            break;
        }
    }
    if max_a_in_pair + max_b_free >= m || max_b_in_pair + max_a_free >= m {
        out.print_line(num_pairs);
        return;
    }
    if num_pair_types > 1 {
        out.print_line(num_pairs - 1);
        return;
    }
    out.print_line(0);
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
    //    tester::stress_test();
}
//END MAIN
