//{"name":"D2. Игра на сумму (Сложная версия)","group":"Codeforces - Codeforces Round #767 (Div. 1)","url":"https://codeforces.com/contest/1628/problem/D2","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n3 3 2\n2 1 10\n6 3 10\n6 4 10\n100 1 1\n4 4 0\n69 4 20\n","output":"6\n5\n375000012\n500000026\n958557139\n0\n49735962\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D2IgraNaSummuSlozhnayaVersiya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::interpolation::Interpolation;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::invertable::Invertable;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let ok = input.read_int();

    type Mod = ModInt7;
    let k = n + 1;
    let n = m - 1;
    let mut values = Vec::with_capacity(n + 1);
    let mut c = Mod::new(2).inv().unwrap();
    for k in 1..=(n + 1) {
        values
            .push(Mod::from_index(n + 1) * c * Mod::new(2) + (Mod::one() - Mod::from_index(k)) * c);
        c *= Mod::new(2);
    }
    let interpolation = Interpolation::new(values);
    let mut ans = interpolation.calculate(Mod::from_index(k - 1));
    ans *= Mod::new(ok);
    ans /= Mod::new(2).power(k - 2);
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
