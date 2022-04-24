//{"name":"L. Exam Preparation","group":"Codeforces - Practice Round","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378187/problem/L","interactive":false,"timeLimit":2000,"tests":[{"input":"2 3 1\n","output":"7\n"},{"input":"3 4 3\n","output":"35\n"},{"input":"3 4 5\n","output":"35\n"},{"input":"11 7 0\n","output":"1\n"},{"input":"616 1337 42\n","output":"673979761\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LExamPreparation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let a = input.read_usize();
    let b = input.read_usize();
    let k = input.read_usize();

    type Mod = ModIntF;
    let mut ans = Mod::zero();
    let c: Combinations<Mod> = Combinations::new(a.max(b) + 1);
    for i in 0..=k.min(a).min(b) {
        ans += c.c(a, i) * c.c(b, i);
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
