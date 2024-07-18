//{"name":"ucup21e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ucup21e"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();

    if 2 * k < m {
        out.print_line(0);
        return;
    }
    type Mod = ModInt7;
    let mut mid_ways = Memoization2d::new(m + 1, m + 1, |mem, len, delta| -> Mod {
        if len == 0 {
            Mod::one()
        } else if k == delta {
            Mod::zero()
        } else {
            let mut res = mem.call(len - 1, delta + 1) * Mod::from_index(k - delta);
            if len > 1 {
                res += mem.call(len - 2, delta + 1)
                    * Mod::from_index(k - delta)
                    * Mod::from_index(len - 1);
            }
            res
        }
    });
    let mut ways = Vec::with_capacity(n + 1);
    let mut pow = Mod::one();
    for i in 0..=n {
        ways.push(Mod::from_index(i + 1) * pow);
        pow *= Mod::from_index(k);
    }
    ways.reverse();
    let mut cur_ways = Mod::one();
    let mut ans = Mod::zero();
    let mut no_intersection = mid_ways.call(m, 0);
    for i in 1..=m / 2 {
        cur_ways *= Mod::from_index(k - i + 1);
        let cur = cur_ways * mid_ways.call(m - 2 * i, i);
        no_intersection -= cur;
        let t = m - i;
        let mut add = Mod::zero();
        let mut sign = Mod::one();
        for len in (m..=n).step_by(t) {
            add += ways[len] * sign;
            sign = -sign;
        }
        ans += add * cur;
    }
    ans += no_intersection * ways[m];
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
    //    tester::stress_test();
}
//END MAIN
