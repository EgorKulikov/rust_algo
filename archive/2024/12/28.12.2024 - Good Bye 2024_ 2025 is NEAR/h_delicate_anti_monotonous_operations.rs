//{"name":"H. Delicate Anti-monotonous Operations","group":"Codeforces - Good Bye 2024: 2025 is NEAR","url":"https://codeforces.com/contest/2053/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n5 8\n1 2 3 4 5\n7 5\n3 1 2 3 4 1 1\n","output":"15 0\n34 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HDelicateAntiMonotonousOperations"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let w = input.read_size();
    let mut a = input.read_size_vec(n);

    if w == 2 {
        let mut ans = a.copy_sum();
        let mut op = 0;
        for (a, b) in a.consecutive_iter() {
            if *a == 1 && *b == 1 {
                ans += 1;
                op += 1;
            }
        }
        out.print_line((ans, op));
        return;
    }
    let mut near = false;
    for (a, b) in a.consecutive_iter() {
        if a == b {
            near = true;
            break;
        }
    }
    if !near {
        out.print_line((a.copy_sum(), 0));
        return;
    }
    let mut min_moves = None;
    let ans = n * w - 1;
    for _ in 0..2 {
        let mut prefix = 0;
        while prefix < n && a[prefix] == w {
            prefix += 1;
        }
        if prefix == n {
            out.print_line((n * w, 0));
            return;
        }
        if a.copy_sum() == n * w - 1 {
            out.print_line((n * w - 1, 0));
            return;
        }
        let mut suffix = 0;
        while suffix < n && a[Back(suffix)] == w {
            suffix += 1;
        }
        fn check2(pos: usize, a: &[usize], w: usize) -> usize {
            if pos == 0 {
                return check(0, a, w);
            }
            let mut a = a.to_vec();
            let mut res = None;
            if a[pos - 1] != w {
                a[pos + 1] = w;
                let v1 = check(pos - 1, &a, w) + 1;
                res.minim(v1);
            }
            a[pos + 1] = if a[pos] == 1 { 2 } else { 1 };
            let v2 = check(pos - 1, &a, w) + 1;
            res.minim(v2);
            res.unwrap()
        }
        fn check(mut pos: usize, a: &[usize], w: usize) -> usize {
            let mut a = a.to_vec();
            let mut res = 0;
            while pos > 0 {
                a[pos] = a[pos - 1];
                a[pos + 1] = if a[pos] == 1 { 2 } else { 1 };
                res += 1;
                pos -= 1;
            }
            while pos < a.len() - 1 {
                if pos + 2 == a.len() {
                    res += 1;
                    break;
                }
                let mut num_ws = 0;
                for i in pos + 2..a.len() {
                    if a[i] == w {
                        num_ws += 1;
                    } else {
                        break;
                    }
                }
                if num_ws == 0 {
                    pos += 1;
                    res += 1;
                } else if num_ws == 1 {
                    res += 3;
                    pos += 1;
                } else if num_ws % 2 == 0 {
                    res += 3 * (num_ws / 2);
                    pos += num_ws;
                } else {
                    res += 3 * ((num_ws - 3) / 2) + 5;
                    pos += num_ws;
                }
                // if a[pos + 2] == w {
                //     if pos + 3 < a.len() && a[pos + 3] == w {
                //         res += 3;
                //         pos += 2;
                //     } else {
                //         res += 3;
                //         pos += 1;
                //     }
                // } else {
                //     pos += 1;
                //     res += 1;
                // }
            }
            res
        }
        if prefix >= 2 {
            min_moves.minim(check2(0, &a[prefix - 2..n - suffix], w));
        }
        let mut done_non_w = false;
        let mut done_w = false;
        for i in prefix..n - suffix - 1 {
            if a[i] == a[i + 1] {
                if a[i] == w && !done_w {
                    done_w = true;
                    min_moves.minim(check2(i - prefix, &a[prefix..n - suffix], w));
                }
                if a[i] != w && !done_non_w {
                    done_non_w = true;
                    min_moves.minim(check2(i - prefix, &a[prefix..n - suffix], w));
                }
            }
        }
        a.reverse();
    }
    out.print_line((ans, min_moves));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
