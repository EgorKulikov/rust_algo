//{"name":"Linear Algebra: Concepts and Methods","group":"Baekjoon Online Judge","url":"https://www.acmicpc.net/contest/problem/1238/8","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n","output":"NO\n"},{"input":"10\n","output":"YES\n5 4 8 2 10 1 9 3 7 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LinearAlgebraConceptsAndMethods"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::numbers::primes::sieve::primality_table;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let is_prime = primality_table(n);
    if is_prime[n - 1] {
        out.print_line(false);
        // eprintln!("n = {n} fail");
        return;
    }
    if n % 2 == 0 && n > 10 {
        if is_prime[n - 1] {
            out.print_line(false);
            return;
        }
        let mut ans = Vec::with_capacity(n / 2);
        ans.push(1);
        for i in (2..=n - 1).rev().step_by(4) {
            ans.push(i);
        }
        for i in (2..=n - 3).rev().step_by(4) {
            ans.push(i);
        }
        ans.reverse();
        ans.push(n);
        for i in (2..n).step_by(4) {
            ans.push(i);
        }
        for i in (4..n).step_by(4) {
            ans.push(i);
        }
        out.print_line(true);
        out.print_line(ans);
        return;
    }
    let mut ans = Vec::new();
    let mut used = BitSet::new(n);
    let mut rec = RecursiveFunction::new(|f, step| {
        if step == n {
            return true;
        }
        for i in 1..=n {
            if used[i - 1] {
                continue;
            }
            let mut min = i;
            let mut max = i;
            let mut good = true;
            for j in (0..step).rev() {
                min.minim(ans[j]);
                max.maxim(ans[j]);
                if min != i && max != i {
                    break;
                }
                if is_prime[max - min] {
                    good = false;
                    break;
                }
            }
            if good {
                ans.push(i);
                used.set(i - 1);
                if f.call(step + 1) {
                    return true;
                }
                ans.pop();
                used.unset(i - 1);
            }
        }
        false
    });
    if rec.call(0) {
        out.print_line(true);
        out.print_line(ans);
    } else {
        out.print_line(false);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
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
    if false {
        true
    } else {
        input.skip_whitespace();
        input.peek().is_none()
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    tester::stress_test();
}
//END MAIN
