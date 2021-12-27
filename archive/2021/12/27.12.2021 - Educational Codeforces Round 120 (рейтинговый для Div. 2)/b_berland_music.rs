//{"name":"B. Berland Music","group":"Codeforces - Educational Codeforces Round 120 (рейтинговый для Div. 2)","url":"http://codeforces.com/contest/1622/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n1 2\n10\n3\n3 1 2\n111\n8\n2 3 1 8 5 4 7 6\n01110001\n","output":"2 1\n3 1 2\n1 6 5 8 3 2 4 7\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBerlandMusic"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::permutation::Permutation;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let p = input.read_usize_vec(n);
    let s: Str = input.read();

    let mut ans = (0..n).collect_vec();
    ans.sort_by_key(|i| (s[*i], p[*i]));
    let p = Permutation::new(ans);
    let mut ans = p.inv();
    ans.set_base(1);
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
