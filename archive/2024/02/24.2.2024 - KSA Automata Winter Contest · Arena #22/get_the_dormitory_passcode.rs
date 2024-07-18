//{"name":"Get the Dormitory Passcode","group":"Baekjoon Online Judge","url":"https://www.acmicpc.net/contest/problem/1238/4","interactive":true,"timeLimit":2000,"tests":[{"input":"2\n\n4\n\n6\n","output":"? 1 3\n\n? 2 4\n\n! 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GetTheDormitoryPasscode"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    if n == 1 {
        out.print_line("? 1");
        out.print_line(('!', input.read::<Mod>()));
        return;
    }

    type Mod = ModIntF;

    let queries = Arr2d::generate(n, n, |i, j| {
        let mut base = 2 * i * n + 2 * j;
        if i >= 2 && i == j {
            base += 1;
        }
        Mod::from_index(base)
    });
    let mut res = Vec::with_capacity(n);
    for i in 0..n {
        out.print("? ");
        out.print_iter(queries.row(i));
        out.print_line(());
        res.push(input.read::<Mod>());
    }
    let mut sum = res[1] - res[0];
    let mut ans = vec![Mod::zero(); n];
    for i in 2..n {
        ans[i] = res[i] - res[0] - sum * Mod::from_index(i);
    }
    sum /= Mod::from_index(2 * n);
    for i in 2..n {
        sum -= ans[i];
    }
    let mut rem = res[0];
    for i in 2..n {
        rem -= ans[i] * Mod::from_index(2 * i);
    }
    rem /= Mod::new(2);
    ans[1] = rem;
    ans[0] = sum - rem;
    out.print_line(('!', ans));
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
    if true {
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
    //    tester::stress_test();
}
//END MAIN
