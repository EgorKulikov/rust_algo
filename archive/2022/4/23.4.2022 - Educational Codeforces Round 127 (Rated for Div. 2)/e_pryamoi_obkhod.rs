//{"name":"E. Прямой обход","group":"Codeforces - Educational Codeforces Round 127 (Rated for Div. 2)","url":"https://codeforces.com/contest/1671/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\nBAAAAAAAABBABAB\n","output":"16\n"},{"input":"2\nBAA\n","output":"1\n"},{"input":"2\nABA\n","output":"2\n"},{"input":"2\nAAB\n","output":"2\n"},{"input":"2\nAAA\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPryamoiObkhod"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::cmp::Ordering;

fn solve(input: &mut Input, _test_case: usize) {
    let _n = input.read_usize();
    let s: Str = input.read();

    type Mod = ModIntF;
    let mut ways = vec![Mod::zero(); s.len()];
    let mut first = vec![Str::new(); s.len()];

    for i in (0..s.len()).rev() {
        if 2 * i + 1 >= s.len() {
            ways[i] = Mod::one();
            first[i] = Str::from(s[i]);
        } else {
            ways[i] = ways[2 * i + 1] * ways[2 * i + 2];
            match first[2 * i + 1].cmp(&first[2 * i + 2]) {
                Ordering::Less => {
                    ways[i] *= Mod::new(2);
                    first[i] =
                        Str::from(s[i]) + first[2 * i + 1].clone() + first[2 * i + 2].clone();
                }
                Ordering::Equal => {
                    first[i] =
                        Str::from(s[i]) + first[2 * i + 1].clone() + first[2 * i + 2].clone();
                }
                Ordering::Greater => {
                    ways[i] *= Mod::new(2);
                    first[i] =
                        Str::from(s[i]) + first[2 * i + 2].clone() + first[2 * i + 1].clone();
                }
            }
        }
    }
    out_line!(ways[0]);
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
