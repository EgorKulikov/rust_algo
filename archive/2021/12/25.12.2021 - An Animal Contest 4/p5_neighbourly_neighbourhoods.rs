//{"name":"P5 - Neighbourly Neighbourhoods","group":"DMOJ - An Animal Contest 4","url":"https://dmoj.ca/problem/aac4p5","interactive":false,"timeLimit":1000,"tests":[{"input":"5 7 2 1\n2 3\n","output":"2 5\n3 2\n4 3\n5 4\n2 3\n2 4\n3 4\n"},{"input":"5 2 3 3\n1 2\n2 3\n3 4\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P5NeighbourlyNeighbourhoods"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let x = input.read();
    let q = input.read();
    let required = input.read_vec::<(usize, usize)>(q).dec_by_one();

    let mut dsu = DSU::new(n);
    let mut ans = HashSet::new();
    for (x, y) in required {
        let x = dsu.get(x);
        let y = dsu.get(y);
        dsu.join(x, y);
    }
    // eprintln!("{}", ans.len());
    if dsu.count() < x {
        out_line!(-1);
        return;
    }
    let mut roots = dsu.iter().collect_vec();
    roots.sort_by_key(|i| dsu.size(*i));
    roots.reverse();
    let mut at = 1;
    while dsu.count() > x {
        dsu.join(roots[0], roots[at]);
        at += 1;
    }
    // eprintln!("{}", ans.len());
    let parts = dsu.parts();
    for part in parts.iter() {
        if part.len() == 1 {
            continue;
        }
        for i in 1..part.len() {
            ans.insert((part[i - 1], part[i]));
        }
        ans.insert((*part.last().unwrap(), part[0]));
    }
    if ans.len() > m {
        out_line!(-1);
        return;
    }
    for i in 0..parts.len() {
        if ans.len() == m {
            break;
        }
        for j in 0..i {
            if ans.len() == m {
                break;
            }
            for k in parts[i].iter().cloned() {
                if ans.len() == m {
                    break;
                }
                for l in parts[j].iter().cloned() {
                    if ans.len() == m {
                        break;
                    }
                    ans.insert((k, l));
                }
            }
        }
    }
    for part in parts {
        if ans.len() == m {
            break;
        }
        for i in part.iter().cloned() {
            if ans.len() == m {
                break;
            }
            for j in part.iter().cloned() {
                if i == j {
                    continue;
                }
                if !ans.contains(&(i, j)) {
                    ans.insert((i, j));
                }
                if ans.len() == m {
                    break;
                }
            }
        }
        // eprintln!("{}", ans.len());
    }
    if ans.len() < m {
        out_line!(-1);
    } else {
        let ans = ans.into_iter().map(|(a, b)| (a + 1, b + 1)).collect_vec();
        output().print_per_line(&ans);
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
