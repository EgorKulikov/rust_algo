//{"name":"D. (XO)R-ождественское дерево","group":"Codeforces - Codeforces Global Round 18","url":"https://codeforces.com/contest/1615/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n6 5\n1 2 -1\n1 3 1\n4 2 7\n6 3 0\n2 5 -1\n2 3 1\n2 5 0\n5 6 1\n6 1 1\n4 5 1\n5 3\n1 2 -1\n1 3 -1\n1 4 1\n4 5 -1\n2 4 0\n3 4 1\n2 3 1\n3 3\n1 2 -1\n1 3 -1\n1 2 0\n1 3 1\n2 3 0\n2 1\n1 2 1\n1 2 0\n","output":"YES\n1 2 0\n1 3 1\n2 4 7\n3 6 0\n2 5 0\nYES\n1 2 1\n1 3 0\n1 4 1\n4 5 1\nNO\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DXOROzhdestvenskoeDerevo"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();
    let mut edges: Vec<(usize, usize, i32)> = input.read_vec(n - 1);
    let elves: Vec<(usize, usize, i32)> = input.read_vec(m);

    let mut tp = vec![0; n];
    let mut id = (0..n).collect_vec();
    let mut parts = Vec::with_capacity(n);
    for i in 0..n {
        parts.push(vec![i]);
    }
    for (mut x, mut y, mut v) in edges.iter().cloned().chain(elves.into_iter()) {
        if v == -1 {
            continue;
        }
        x -= 1;
        y -= 1;
        v = (v.count_ones() % 2) as i32;
        if id[x] == id[y] {
            if tp[x] ^ tp[y] != v {
                out_line!("NO");
                return;
            }
        } else {
            let change = tp[x] ^ tp[y] ^ v;
            if parts[id[x]].len() < parts[id[y]].len() {
                swap(&mut x, &mut y);
            }
            let mut idy = Vec::new();
            swap(&mut idy, &mut parts[id[y]]);
            for i in idy {
                id[i] = id[x];
                parts[id[x]].push(i);
                tp[i] ^= change;
            }
        }
    }
    for (x, y, v) in edges.iter_mut() {
        if *v == -1 {
            let x = *x - 1;
            let y = *y - 1;
            *v = tp[x] ^ tp[y];
        }
    }
    out_line!("YES");
    output().print_per_line(&edges);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
