//{"name":"S1 - Kaguya Wants to Let Ishigami Pass","group":"DMOJ - Mock CCC '22 2","url":"https://dmoj.ca/problem/nccc10s1","interactive":false,"timeLimit":250,"tests":[{"input":"5 4\nTFTF\nTFFF\nTFTT\nTFFT\nTFTF\n","output":"2\n"},{"input":"3 5\nTFTFT\nTFTFT\nTFTFT\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"S1KaguyaWantsToLetIshigamiPass"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let k = input.read_usize();
    let res = input.read_table::<char>(n, k);

    let mut ans = None;
    for i in 0..(1 << k) {
        let mut cur = None;
        for j in 0..n {
            let mut mark = 0;
            for l in 0..k {
                if i.is_set(l) == (res[(j, l)] == 'T') {
                    mark += 1;
                }
            }
            cur.minim(mark);
        }
        ans.maxim(cur.unwrap());
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
