//{"name":"Duck, Duck, Geese","group":"Google Coding Competitions - Round 3 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/00000000008779b4/0000000000b45244","interactive":false,"timeLimit":20000,"tests":[{"input":"3\n3 2\n1 1\n1 1\n1 1 2\n5 2\n1 1\n1 2\n1 2 1 2 2\n3 3\n1 2\n1 2\n2 2\n1 1 3\n","output":"Case #1: 2\nCase #2: 9\nCase #3: 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DuckDuckGeese"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

use algo_lib::out_line;

fn solve(input: &mut Input, test_case: usize) {
    let n = input.read_usize();
    let c = input.read_usize();
    let limits = input.read_usize_pair_vec(c);
    let p = input.read_usize_vec(n).dec_by_one();

    let mut pos = vec![Vec::new(); c];
    for (j, &i) in p.iter().enumerate() {
        pos[i].push(j);
    }
    let mut at = vec![0; c];

    let mut had = BitSet::new(c);
    let mut ans = 0;
    for i in 0..n {
        had.fill(false);
        let mut min = i + 1;
        let mut max = n + i - 2;
        for j in i.. {
            if j > max {
                break;
            }
            let cur = p[j % n];
            if had[cur] {
                if j >= min {
                    ans += 1;
                }
                continue;
            }
            had.set(cur, true);
            let (f, t) = limits[cur];
            if f > pos[cur].len() {
                break;
            }
            if f != 0 {
                let mut p = pos[cur][(at[cur] + f - 1) % pos[cur].len()];
                if p < j {
                    p += n;
                }
                min.maxim(p);
            }
            if t == 0 {
                break;
            }
            if t < pos[cur].len() {
                let mut p = pos[cur][(at[cur] + t) % pos[cur].len()];
                if p < j {
                    p += n;
                }
                p -= 1;
                max.minim(p);
            }
            if j >= min {
                ans += 1;
            }
        }
        at[p[i]] += 1;
    }

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
