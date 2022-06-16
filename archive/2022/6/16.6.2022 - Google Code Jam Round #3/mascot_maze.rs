//{"name":"Mascot Maze","group":"Google Coding Competitions - Round 3 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/00000000008779b4/0000000000b44a4f","interactive":false,"timeLimit":20000,"tests":[{"input":"4\n3\n2 1 1\n3 3 2\n6\n3 1 4 1 2 3\n5 3 5 2 4 5\n20\n2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 1 1\n3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 20 2\n19\n2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 1 1\n3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 19 3\n","output":"Case #1: IMPOSSIBLE\nCase #2: TSHIRT\nCase #3: HCJKSHCJKSHCJKSHCJKS\nCase #4: CODEJAMROCKSTHEMOST\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MascotMaze"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use std::collections::BTreeSet;

use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, test_case: usize) {
    let n = input.read_usize();
    let left = input.read_usize_vec(n).dec_by_one();
    let right = input.read_usize_vec(n).dec_by_one();

    let mut inp = vec![Vec::new(); n];
    for i in 0..n {
        for j in vec![left[i], right[i]] {
            inp[j].push(i);
            inp[i].push(j);
            for k in vec![left[j], right[j]] {
                if i == k {
                    out_line!(format!("Case #{}:", test_case), "IMPOSSIBLE");
                    return;
                }
                inp[k].push(i);
                inp[i].push(k);
            }
        }
    }
    let mut qty = inp.iter().map(|v| v.len()).collect_vec();
    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert((qty[i], i));
    }
    let mut processed = BitSet::new(n);
    let mut order = Vec::with_capacity(n);
    for _ in 0..n {
        let &(_, cur) = set.iter().next().unwrap();
        set.remove(&(qty[cur], cur));
        processed.set(cur, true);
        order.push(cur);
        for &i in &inp[cur] {
            if !processed[i] {
                set.remove(&(qty[i], i));
                qty[i] -= 1;
                set.insert((qty[i], i));
            }
        }
    }
    let s: Str = "ACDEHIJKMORST".into();
    let mut ans = vec![0; n];
    for i in order.into_iter().rev() {
        for c in s.iter() {
            let mut good = true;
            for &j in &inp[i] {
                if ans[j] == c {
                    good = false;
                    break;
                }
            }
            if good {
                ans[i] = c;
                break;
            }
        }
    }
    let ans: Str = ans.into();
    out_line!(format!("Case #{}:", test_case), ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    true
    // input.skip_whitespace();
    // !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
