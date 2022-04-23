//{"name":"F - Ignore Operations","group":"AtCoder - Monoxer Programming Contest 2022（AtCoder Beginner Contest 249）","url":"https://atcoder.jp/contests/abc249/tasks/abc249_f","interactive":false,"timeLimit":2000,"tests":[{"input":"5 1\n2 4\n2 -3\n1 2\n2 1\n2 -3\n","output":"3\n"},{"input":"1 0\n2 -1000000000\n","output":"-1000000000\n"},{"input":"10 3\n2 3\n2 -1\n1 4\n2 -1\n2 5\n2 -9\n2 2\n1 -6\n2 5\n2 -3\n","output":"15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FIgnoreOperations"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::BTreeSet;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let k = input.read_usize();
    let mut op = input.read_long_pair_vec(n);

    op.reverse();
    let mut skipped = 0;
    let mut sum = 0;
    let mut set = BTreeSet::new();
    let mut ans = None;
    for (i, (t, y)) in op.into_iter().enumerate() {
        if t == 1 {
            ans.maxim(y + sum);
            skipped += 1;
            if skipped > k {
                break;
            }
            if skipped + set.len() > k {
                let &(val, key) = set.iter().rev().next().unwrap();
                sum += val;
                set.remove(&(val, key));
            }
        } else {
            if y >= 0 {
                sum += y;
            } else {
                set.insert((y, i));
                if skipped + set.len() > k {
                    let &(val, key) = set.iter().rev().next().unwrap();
                    sum += val;
                    set.remove(&(val, key));
                }
            }
        }
    }
    if skipped <= k {
        ans.maxim(sum);
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
