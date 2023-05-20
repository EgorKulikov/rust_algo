//{"name":"E. Задачка на комбинаторику","group":"Codeforces - Educational Codeforces Round 148 (Rated for Div. 2)","url":"https://codeforces.com/contest/1832/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"5 8 2 3 100 2\n","output":"1283\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EZadachkaNaKombinatoriku"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut a1 = input.read_long();
    let x = input.read_long();
    let y = input.read_long();
    let m = input.read_long();
    let k = input.read_size();

    type Mod = ModIntF;
    let mut b = vec![Mod::zero(); k + 1];
    let mut ans = 0;
    for j in 0..n {
        b[0] += Mod::new_from_wide(a1);
        for i in (1..=k).rev() {
            let bb = b[i - 1];
            b[i] += bb;
        }
        ans ^= b[k].val().into_i64() * (j + 1).into_i64();
        a1 = (a1 * x + y) % m;
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
