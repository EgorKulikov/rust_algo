//{"name":"E - Prefix Equality","group":"AtCoder - AtCoder Beginner Contest 250","url":"https://atcoder.jp/contests/abc250/tasks/abc250_e","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n1 2 3 4 5\n1 2 2 4 3\n7\n1 1\n2 2\n2 3\n3 3\n4 4\n4 5\n5 5\n","output":"Yes\nYes\nYes\nNo\nNo\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPrefixEquality"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, BoolOutput};
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);

    let mut a_size = Vec::with_capacity(n + 1);
    let mut b_size = Vec::with_capacity(n + 1);
    let mut good_size = BitSet::new(n + 1);
    let mut a_set = HashSet::new();
    let mut b_set = HashSet::new();
    let mut common_set = HashSet::new();

    a_size.push(0);
    b_size.push(0);
    let mut j = 0;
    for i in 0..n {
        if a_set.contains(&a[i]) || common_set.contains(&a[i]) {
            a_size.push(a_set.len() + common_set.len());
            continue;
        }
        if b_set.contains(&a[i]) {
            b_set.remove(&a[i]);
            common_set.insert(a[i]);
        } else {
            a_set.insert(a[i]);
        }
        a_size.push(a_set.len() + common_set.len());
        while j < n {
            if b_set.contains(&b[j]) || common_set.contains(&b[j]) {
                b_size.push(b_set.len() + common_set.len());
                j += 1;
                continue;
            }
            if a_set.contains(&b[j]) {
                a_set.remove(&b[j]);
                common_set.insert(b[j]);
            } else {
                b_set.insert(b[j]);
            }
            b_size.push(b_set.len() + common_set.len());
            j += 1;
            break;
        }
        if a_set.is_empty() && b_set.is_empty() {
            good_size.set(common_set.len(), true);
        }
    }
    while j < n {
        if b_set.contains(&b[j]) || common_set.contains(&b[j]) {
            b_size.push(b_set.len() + common_set.len());
            j += 1;
            continue;
        }
        if a_set.contains(&b[j]) {
            a_set.remove(&b[j]);
            common_set.insert(b[j]);
        } else {
            b_set.insert(b[j]);
        }
        b_size.push(b_set.len() + common_set.len());
        j += 1;
    }

    output().bool_output = BoolOutput::YesNo;
    let q = input.read_usize();
    for _ in 0..q {
        let x = input.read_usize();
        let y = input.read_usize();

        out_line!(a_size[x] == b_size[y] && good_size[a_size[x]]);
    }
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
