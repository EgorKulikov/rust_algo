//{"name":"L. Treasure Delivery","group":"Codeforces - STAR Contest 2022","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378214/problem/L","interactive":false,"timeLimit":1000,"tests":[{"input":"100 3 2\n2 10\n2 5\n4 4\n2 4\n","output":"9\n"},{"input":"8 2 1\n3 10\n7 19\n6\n","output":"19\n"},{"input":"8 2 2\n3 10\n7 19\n6 7\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LTreasureDelivery"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let _ = input.read_usize();
    let m = input.read_usize();
    let k = input.read_usize();
    let heroes = input.read_usize_pair_vec(m);
    let p = input.read_usize_vec(k);

    let mut ans = None;
    let mut bad = BitSet::new(1 << m);
    for i in 0..m {
        bad.fill(false);
        for &j in &p {
            if j % heroes[i].0 == 0 {
                let mut c_bad = usize::all_bits(m).without_bit(i);
                for l in 0..m {
                    if l != j && j % heroes[l].0 == 0 {
                        c_bad.unset_bit(l);
                    }
                }
                bad.set(c_bad, true);
            }
        }
        for j in (0..(1 << m)).rev() {
            if j.is_set(i) {
                continue;
            }
            if bad[j] {
                for l in 0..m {
                    bad.set(j.without_bit(l), true);
                }
            } else {
                let mut cand = heroes[i].1;
                for l in 0..m {
                    if j.is_set(l) {
                        cand += heroes[l].1;
                    }
                }
                ans.minim(cand);
            }
        }
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
