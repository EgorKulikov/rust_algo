//{"name":"S2 - Kaguya Wants to Grow the Student Council","group":"DMOJ - Mock CCC '22 2","url":"https://dmoj.ca/problem/nccc10s2","interactive":false,"timeLimit":250,"tests":[{"input":"2 3\nBAC\nABC\n","output":"2\n"},{"input":"3 8\nHGBDFCAE\nADBGHFCE\nHCFGBDAE\n","output":"3\n"},{"input":"6 8\nAHFBGDCE\nFABGCEHD\nAHDGFBCE\nDABHGCFE\nABCHFEDG\nDGABHFCE\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"S2KaguyaWantsToGrowTheStudentCouncil"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let k = input.read_usize();
    let votes: Vec<Str> = input.read_vec(n);

    if k == 1 {
        out_line!(1);
        return;
    }
    let mut can = Arr2d::new(k, k, true);
    for vote in votes {
        for i in 0..k {
            for j in i + 1..k {
                can[((vote[i] - b'A').into_usize(), (vote[j] - b'A').into_usize())] = false;
            }
        }
    }
    let mut mask = vec![0; k];
    for i in 0..k {
        for j in 0..k {
            if can[(i, j)] || can[(j, i)] {
                mask[i].set_bit(j);
            }
        }
    }
    let gen_half = |from: usize, to: usize| -> Vec<usize> {
        let mut res = Vec::with_capacity(1 << (to - from));
        res.push(usize::all_bits(from));
        for i in 1..res.capacity() {
            let at = (usize::BITS - i.leading_zeros()).into_usize() - 1;
            if (i.without_bit(at) & (mask[at + from] >> from)) == i.without_bit(at)
                && (res[i.without_bit(at)] >> from) == i.without_bit(at)
            {
                res.push((res[i.without_bit(at)] & mask[at + from]).with_bit(at + from));
            } else {
                res.push(0);
            }
        }
        res
    };
    let front = gen_half(0, k / 2);
    let back = gen_half(k / 2, k);
    let mut front_max = Vec::with_capacity(front.len());
    front_max.push(0);
    for i in 1..front.len() {
        if front[i] != 0 {
            front_max.push(i.count_ones());
        } else {
            let mut res = 0;
            for j in 0..k / 2 {
                if i.is_set(j) {
                    res.maxim(front_max[i.without_bit(j)]);
                }
            }
            front_max.push(res);
        }
    }
    let mut ans = 0;
    for i in 0..back.len() {
        if back[i] != 0 {
            ans.maxim(i.count_ones() + front_max[back[i] & usize::all_bits(k / 2)]);
        }
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
