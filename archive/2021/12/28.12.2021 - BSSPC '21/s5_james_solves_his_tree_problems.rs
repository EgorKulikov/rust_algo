//{"name":"S5 - James Solves His Tree Problems","group":"DMOJ - BSSPC '21","url":"https://dmoj.ca/problem/bsspc21s5","interactive":false,"timeLimit":2000,"tests":[{"input":"8\nL\nD 8 7\nD 4 8\nD 4 6\nD 4 5\nL\nD 1 3\nD 1 2\n","output":"12\n1 2\n2 11\n11 12\n1 3\n3 5\n5 4\n4 6\n6 10\n4 8\n8 7\n7 9\n"},{"input":"3\nD 1 2\nD 2 3\nD 1 3\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"S5JamesSolvesHisTreeProblems"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::{Input, Readable};
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let q = input.read_usize();
    enum Operation {
        EatLeaves,
        DestroyEdge(usize, usize),
    }
    impl Readable for Operation {
        fn read(input: &mut Input) -> Self {
            match input.read_char() {
                'L' => Operation::EatLeaves,
                'D' => Operation::DestroyEdge(input.read_usize() - 1, input.read_usize() - 1),
                _ => unreachable!(),
            }
        }
    }
    let ops: Vec<Operation> = input.read_vec(q);

    let mut x = 0;
    for op in ops.iter() {
        if let Operation::DestroyEdge(a, b) = op {
            x.maxim(*a + 1);
            x.maxim(*b + 1);
        }
    }
    let mut joiners = Vec::new();
    if x == 0 {
        x = 1;
        joiners.push(0);
    }
    let mut deg = vec![0usize; x];
    for op in ops.iter() {
        if let Operation::DestroyEdge(a, b) = op {
            deg[*a] += 1;
            deg[*b] += 1;
        }
    }
    let mut add = vec![0; x];
    let mut ls = 0;
    let mut edges = Vec::new();
    let mut trailing_ls = 0;
    let mut pre_trailing_ls = 0;
    for op in ops {
        match op {
            Operation::EatLeaves => {
                ls += 1;
                trailing_ls += 1;
            }
            Operation::DestroyEdge(a, b) => {
                pre_trailing_ls = trailing_ls;
                trailing_ls = 0;
                if deg[a] > 1 && deg[b] > 1 {
                    out_line!(-1);
                    return;
                }
                if deg[a] == 1 {
                    if add[a] == ls {
                        add[a] = 0;
                    } else {
                        add[a] = ls;
                    }
                }
                if deg[b] == 1 {
                    if add[b] == ls {
                        add[b] = 0;
                    } else {
                        add[b] = ls;
                    }
                }
                if deg[a] == 1 && deg[b] == 1 {
                    if add[a] > add[b] {
                        joiners.push(a);
                    } else {
                        joiners.push(b);
                    }
                }
                deg[a] -= 1;
                deg[b] -= 1;
                if deg[a] == 1 {
                    add[a] = ls;
                }
                if deg[b] == 1 {
                    add[b] = ls;
                }
                edges.push((a, b));
            }
        }
    }
    if joiners.len() > 1 {
        for (j, i) in joiners[0..joiners.len() - 2].iter().cloned().enumerate() {
            add[i] = 0;
            if j == 0 {
                edges.push((i, joiners[joiners.len() - 2]));
            } else {
                edges.push((i, joiners[j - 1]));
            }
            if j == joiners.len() - 3 {
                edges.push((i, joiners[joiners.len() - 1]));
            }
        }
        if joiners.len() == 2 {
            edges.push((joiners[0], joiners[1]));
        }
        add[joiners[joiners.len() - 2]] = 0;
        add[joiners[joiners.len() - 1]] = 0;
        let path_len = joiners.len();
        if path_len > pre_trailing_ls
            && path_len
                >= pre_trailing_ls
                    + if trailing_ls == 0 {
                        0
                    } else if trailing_ls == 1 {
                        2
                    } else {
                        2 * trailing_ls - 1
                    }
        {
            // add[joiners[joiners.len() - 2]] = 2 * ls;
        } else {
            add[joiners[joiners.len() - 2]] =
                ls + if trailing_ls <= 1 { 0 } else { trailing_ls - 2 } - path_len;
            if path_len <= pre_trailing_ls || trailing_ls >= 2 {
                add[joiners[joiners.len() - 2]] += 1;
            }
            // add[joiners[joiners.len() - 2]] = 2 * ls;
        }
        // add[joiners[joiners.len() - 2]] = 2 * ls;
    } else {
        add[joiners[0]] = if trailing_ls == 0 {
            add[joiners[0]]
        } else {
            // 2 * ls
            if x == 1 {
                2 * ls - 2
            } else {
                ls + if trailing_ls == 1 { 0 } else { trailing_ls - 2 }
            }
        };
        // add[joiners[0]] = 2 * ls;
    }
    for (i, a) in add.into_iter().enumerate() {
        let mut last = i;
        for _ in 0..a {
            edges.push((last, x));
            last = x;
            x += 1;
        }
    }
    assert_eq!(x, edges.len() + 1);
    out_line!(x);
    edges = edges.inc_by_one();
    output().print_per_line(&edges);
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
