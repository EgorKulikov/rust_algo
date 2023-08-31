//{"name":"E. Непересекающиеся подперестановки","group":"Codeforces - Educational Codeforces Round 154 (Rated for Div. 2)","url":"https://codeforces.com/contest/1861/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"10 3\n","output":"71712\n"},{"input":"2 2\n","output":"2\n"},{"input":"1337 42\n","output":"524933698\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ENeperesekayushchiesyaPodperestanovki"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::num_utils::powers;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    type Mod = ModIntF;

    let pw = powers(Mod::from_index(k), n + 1);

    let mut mem = Memoization2d::new(n + 1, k + 1, |f, n, l| {
        if n == 0 {
            if l == k {
                return (Mod::one(), Mod::zero());
            }
            return (Mod::zero(), Mod::zero());
        }
        let val = if l == k {
            let (v, _) = f.call(n, 0);
            v + pw[n]
        } else {
            let (v, _) = f.call(n - 1, l + 1);
            let (_, s) = f.call(n - 1, l);
            v * Mod::from_index(k - l) + s
        };
        let sum = if l == 0 {
            Mod::zero()
        } else {
            val + if l > 1 {
                let (_, s) = f.call(n, l - 1);
                s
            } else {
                Mod::zero()
            }
        };
        (val, sum)
    });
    out_line!(mem.call(n, 0).0);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
