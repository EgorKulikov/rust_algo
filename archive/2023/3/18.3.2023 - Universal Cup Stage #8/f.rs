//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::legacy_fill::LegacyFill;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::random::random;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let s = input.read_char_table(n, m);

    let mut dist = vec![0; n];
    let mut cur = vec![0; 4];
    let r = random();
    let mut ans = (0..n).collect_vec();
    let mut mask = BitSet::new(n);
    while ans.len() > 1 {
        dist.legacy_fill(0);
        for i in 0..n {
            mask.set(i, r.next(2) == 0);
            if mask[i] {
                dist[i] = k;
            }
        }
        let tot = mask.count_ones();
        for i in 0..m {
            cur.legacy_fill(0);
            for j in 0..n {
                if mask[j] {
                    let c = s[(j, i)] as usize - 'A' as usize;
                    cur[c] += 1;
                }
            }
            for &j in &ans {
                let c = s[(j, i)] as usize - 'A' as usize;
                dist[j] += tot - cur[c];
            }
        }
        ans = ans
            .into_iter()
            .filter(|&i| dist[i] == k * tot)
            .collect_vec();
    }
    out_line!(ans[0] + 1);
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
