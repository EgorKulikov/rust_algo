//{"name":"D1. Игра на сумму (Легкая версия)","group":"Codeforces - Codeforces Round #767 (Div. 1)","url":"https://codeforces.com/contest/1628/problem/D1","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n3 3 2\n2 1 10\n6 3 10\n6 4 10\n100 1 1\n4 4 0\n69 4 20\n","output":"6\n5\n375000012\n500000026\n958557139\n0\n49735962\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1IgraNaSummuLegkayaVersiya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let k = input.read_int();

    type Mod = ModInt7;
    let mut res = vec![Mod::zero(); n + 1];
    for i in 1..=n {
        res[i] = Mod::new(k) * Mod::from_index(i);
        for j in (1..i).rev() {
            let cur = (res[j] + res[j - 1]) / Mod::new(2);
            res[j] = cur;
        }
    }
    out_line!(res[m]);
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

#[test]
fn print_table() {
    let n = 10;

    type Mod = ModInt7;
    let mut res = vec![Mod::zero(); n + 1];
    for i in 1..=n {
        res[i] = Mod::from_index(i) * Mod::new(2).power(i - 1);
        for j in (1..i).rev() {
            let cur = res[j] + res[j - 1];
            res[j] = cur;
        }
        println!("{:?}", &res[..=i]);
    }
}

fn main() {
    tester::run_tests();
}
//END MAIN
