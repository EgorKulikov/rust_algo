//{"name":"Count the Chaos","group":"Toph","url":"https://toph.co/p/count-the-chaos","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n1 2\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CountTheChaos"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = input.read_vec::<usize>(n).dec_by_one();

    let mut ft: FenwickTree<u64> = FenwickTree::new(n);
    let mut ans = 0u64;
    for i in a {
        ans += ft.get(i, n);
        ft.add(i, 1);
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
