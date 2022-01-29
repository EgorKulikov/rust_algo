//{"name":"C - The Majority","group":"AtCoder - AtCoder Regular Contest 134","url":"https://atcoder.jp/contests/arc134/tasks/arc134_c","interactive":false,"timeLimit":2000,"tests":[{"input":"2 2\n3 1\n","output":"2\n"},{"input":"2 1\n1 100\n","output":"0\n"},{"input":"20 100\n1073813 90585 41323 52293 62633 28788 1925 56222 54989 2772 36456 64841 26551 92115 63191 3603 82120 94450 71667 9325\n","output":"313918676\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTheMajority"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::num_utils::factorial;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let k = input.read_usize();
    let mut a = input.read_usize_vec(n);

    type Mod = ModIntF;
    let sum = k + a.iter().skip(1).sum::<usize>();
    if a[0] < sum {
        out_line!(0);
        return;
    }
    a[0] -= sum;
    let mut ans = Mod::one();
    let f = factorial::<Mod>(k - 1);
    for i in a {
        for j in i + 1..i + k {
            ans *= Mod::from_index(j);
        }
        ans /= f;
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
