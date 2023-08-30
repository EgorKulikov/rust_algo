//{"name":"G. Now I Know You Are Blind Man, But You Gotta See This","group":"Codeforces - Aleppo Collegiate Programming Contest 2023 V.2","url":"https://codeforces.com/gym/104544/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n3\n0 1 2\n","output":"7\n"},{"input":"2\n4\n0 3 1 3\n5\n1 3 2 3 2\n","output":"12\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GNowIKnowYouAreBlindManButYouGottaSeeThis"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut qty = DefaultHashMap::<_, usize>::new();
    for i in a {
        qty[i] += 1;
    }
    type Mod = ModInt7;
    let mut ans = Mod::zero();
    let mut mult = Mod::one();
    let mut rem = n;
    for i in 0.. {
        rem -= qty[i];
        ans += Mod::from_index(i) * mult * Mod::new(2).power(rem);
        if qty[i] == 0 {
            break;
        }
        mult *= Mod::new(2).power(qty[i]) - Mod::one();
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
