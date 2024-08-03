//{"name":"F. Образуйте треугольники","group":"Codeforces - Pinely Round 4 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1991/problem/F","interactive":false,"timeLimit":5000,"tests":[{"input":"10 5\n5 2 2 10 4 10 6 1 5 3\n1 6\n2 7\n2 8\n5 10\n4 10\n","output":"YES\nNO\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FObrazuiteTreugolniki"}}}

use algo_lib::collections::slice_ext::backward::BackwardSlice;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_int_vec(n);

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();
        if r - l >= 50 {
            out.print_line(true);
            continue;
        }
        let cur = a[l..r].to_vec().sorted();
        let mut good = Vec::new();
        for i in 2..cur.len() {
            if cur[i] < cur[i - 1] + cur[i - 2] {
                good.push(i);
            }
        }
        if good.len() <= 1 {
            out.print_line(false);
            continue;
        }
        if good.backward()[0] - good[0] >= 3 {
            out.print_line(true);
            continue;
        }
        if good.backward()[0] < 5 {
            out.print_line(false);
            continue;
        }
        let cur = cur[good.backward()[0] - 5..=good.backward()[0]].to_vec();
        let mut found = false;
        for i in 32usize..64 {
            if i.count_ones() != 3 {
                continue;
            }
            let mut good = true;
            let mut sum_one = 0;
            let mut sum_zero = 0;
            let mut q_one = 0;
            let mut q_zero = 0;
            for j in 0..6 {
                if i.is_set(j) {
                    if q_one < 2 {
                        q_one += 1;
                        sum_one += cur[j];
                    } else {
                        if sum_one <= cur[j] {
                            good = false;
                            break;
                        }
                    }
                } else {
                    if q_zero < 2 {
                        q_zero += 1;
                        sum_zero += cur[j];
                    } else {
                        if sum_zero <= cur[j] {
                            good = false;
                            break;
                        }
                    }
                }
            }
            if good {
                found = true;
                break;
            }
        }
        out.print_line(found);
    }
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
}
//END MAIN
