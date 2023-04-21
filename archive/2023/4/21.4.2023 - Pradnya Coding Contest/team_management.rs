//{"name":"Team Management","group":"CodeChef - PRAD2023","url":"https://www.codechef.com/PRAD2023/problems/TEA_MAN","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n1\n3\n","output":"1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TeamManagement"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

type Mod = ModInt7;

fn solve(input: &mut Input, _test_case: usize, f: &[Mod]) {
    let n = input.read_size();

    out_line!(f[n]);
}

#[test]
fn test() {
    use algo_lib::numbers::mod_utils::Combinations;
    let c = Combinations::<Mod>::new(21);

    for n in 1..=20 {
        let mut ans = Mod::zero();
        for i in 0..=n / 2 {
            ans += c.c(n - i, i);
        }
        println!("{} {}", n, ans);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let mut f = vec![Mod::one(), Mod::one()];
    for i in 2..=1000000 {
        let res = f[i - 1] + f[i - 2];
        f.push(res);
    }
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1, &f);
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
