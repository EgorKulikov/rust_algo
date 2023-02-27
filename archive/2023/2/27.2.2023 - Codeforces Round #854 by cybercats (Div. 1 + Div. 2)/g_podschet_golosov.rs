//{"name":"G. Подсчет голосов","group":"Codeforces - Codeforces Round #854 by cybercats (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1799/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 1 1\n1 2 3\n","output":"2\n"},{"input":"5\n2 0 1 0 2\n1 2 3 4 5\n","output":"10\n"},{"input":"5\n1 2 2 0 0\n3 5 4 3 4\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GPodschetGolosov"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    type Mod = ModIntF;

    let n = input.read_size();
    let c = input.read_size_vec(n);
    let t = input.read_size_vec(n).dec_by_one();

    let mut qty = vec![0; n];
    let mut want = vec![0; n];
    for i in 0..n {
        qty[t[i]] += 1;
        want[t[i]] += c[i];
    }

    let cc = Combinations::<Mod>::new(n + 1);
    let mut mem = Memoization3d::new(n + 1, n + 1, n + 1, |f, step, free, bad| -> Mod {
        if step == n {
            return if bad == 0 && free == 0 {
                Mod::one()
            } else {
                Mod::zero()
            };
        }
        let mut res = Mod::zero();
        for i in 0..=qty[step].min(bad).min(want[step]) {
            let cf = want[step] - i;
            if free < cf {
                continue;
            }
            res += f.call(step + 1, free - cf, bad - i) * cc.c(qty[step], i) * cc.c(free, cf);
        }
        res
    });
    let mut ans = Mod::zero();
    for i in 0..=n {
        let call = mem.call(0, n - i, i);
        if i % 2 == 0 {
            ans += call;
        } else {
            ans -= call;
        }
    }
    for i in 0..n {
        ans *= cc.c(want[t[i]], c[i]);
        want[t[i]] -= c[i];
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
