//{"name":"S3 - James's Egirl Discord Status","group":"DMOJ - BSSPC '21","url":"https://dmoj.ca/problem/bsspc21s3","interactive":false,"timeLimit":1000,"tests":[{"input":"5 2\n1 3 2 -4 3\n","output":"5\n"},{"input":"4 3\n1 2 -69 8\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"S3JamessEgirlDiscordStatus"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let a = input.read_long_vec(n);

    let mut min = vec![i64::MAX; k];
    let mut ans = 0;
    let mut sum = 0;
    for (i, a) in a.into_iter().enumerate() {
        min[i % k].minim(sum);
        ans.maxim(sum - min[i % k]);
        sum += a;
    }
    ans.maxim(sum - min[n % k]);
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
