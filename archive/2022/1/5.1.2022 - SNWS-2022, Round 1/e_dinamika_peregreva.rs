//{"name":"E. Динамика перегрева","group":"Yandex - SNWS-2022, Round 1","url":"https://contest.yandex.ru/snws2022/contest/23957/problems/E/","interactive":false,"timeLimit":2000,"tests":[{"input":"2 2\n2 0\n-6 11\n1 1 6\n1 2 6\n","output":"12\n-25\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EDinamikaPeregreva"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let q = input.read_usize();
    let f: Vec<(i64, i64)> = input.read_vec(n);

    for _ in 0..q {
        let l = input.read_usize() - 1;
        let r = input.read_usize();
        let d = input.read_long();
        let mut ans = None;
        for (a, b) in &f[l..r] {
            ans.minim(d * *a + *b);
        }
        out_line!(ans);
    }
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
