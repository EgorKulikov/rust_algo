//{"name":"E - Blackout 2","group":"AtCoder - freee Programming Contest 2022（AtCoder Beginner Contest 264）","url":"https://atcoder.jp/contests/abc264/tasks/abc264_e","interactive":false,"timeLimit":3000,"tests":[{"input":"5 5 10\n2 3\n4 10\n5 10\n6 9\n2 9\n4 8\n1 7\n3 6\n8 10\n1 8\n6\n3\n5\n8\n10\n2\n7\n","output":"4\n4\n2\n2\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBlackout2"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::legacy_fill::LegacyFill;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let m = input.read_usize();
    let e = input.read_usize();
    let edges = input.read_usize_pair_vec(e).dec_by_one();
    let q = input.read_usize();
    let x = input.read_usize_vec(q).dec_by_one();

    let mut broken = BitSet::new(e);
    for &i in &x {
        broken.set(i, true);
    }
    let mut unelectrified_count = vec![0; n + m];
    unelectrified_count[0..n].legacy_fill(1);
    let mut dsu = DSU::new(n + m);
    let mut count = 0;
    let mut ans = Vec::with_capacity(q);
    for i in 0..e {
        if !broken[i] {
            let (mut a, mut b) = edges[i];
            a = dsu.get(a);
            b = dsu.get(b);
            if a == b {
                continue;
            }
            if unelectrified_count[a] == 0 {
                count += unelectrified_count[b];
                unelectrified_count[b] = 0;
            } else if unelectrified_count[b] == 0 {
                count += unelectrified_count[a];
                unelectrified_count[a] = 0;
            } else {
                let total = unelectrified_count[a] + unelectrified_count[b];
                unelectrified_count[a] = total;
                unelectrified_count[b] = total;
            }
            dsu.join(a, b);
        }
    }
    for i in x.into_iter().rev() {
        ans.push(count);
        let (mut a, mut b) = edges[i];
        a = dsu.get(a);
        b = dsu.get(b);
        if a == b {
            continue;
        }
        if unelectrified_count[a] == 0 {
            count += unelectrified_count[b];
            unelectrified_count[b] = 0;
        } else if unelectrified_count[b] == 0 {
            count += unelectrified_count[a];
            unelectrified_count[a] = 0;
        } else {
            let total = unelectrified_count[a] + unelectrified_count[b];
            unelectrified_count[a] = total;
            unelectrified_count[b] = total;
        }
        dsu.join(a, b);
    }
    ans.reverse();
    output().print_per_line(&ans);
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
