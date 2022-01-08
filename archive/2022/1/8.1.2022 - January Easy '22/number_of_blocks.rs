//{"name":"Number of blocks","group":"HackerEarth - January Easy '22","url":"https://www.hackerearth.com/challenges/competitive/january-easy-22/algorithm/complete-journey-bf38d697/","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4 6 9\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NumberOfBlocks"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    let mut at = vec![Vec::new(); 100001];
    for (k, i) in a.into_iter().enumerate() {
        let mut j = 1;
        while j * j <= i {
            if i % j == 0 {
                at[j].push(k);
                at[i / j].push(k);
            }
            j += 1;
        }
    }
    let mut dsu = DSU::new(n);
    let mut ans = 0;
    for (i, v) in at.into_iter().enumerate().rev() {
        if v.is_empty() {
            continue;
        }
        let base = v[0];
        for j in v {
            if dsu.join(base, j) {
                ans += i;
            }
        }
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
