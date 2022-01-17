//{"name":"S3 - Kaguya Wants to Analyze the Academy Anthem","group":"DMOJ - Mock CCC '22 2","url":"https://dmoj.ca/problem/nccc10s3","interactive":false,"timeLimit":5000,"tests":[{"input":"abacabadabacaba\n4\na 7\ne 3\nbac 2\nabada 1\n","output":"13\n-1\n10\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"S3KaguyaWantsToAnalyzeTheAcademyAnthem"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::string::hash::{hash, HashBase, SimpleHash, StringHash};
use algo_lib::string::string::Str;

fn solve(input: &mut Input) {
    let s: Str = input.read();
    let q = input.read_usize();
    let queries: Vec<(Str, usize)> = input.read_vec(q);

    let res = solve_impl(s, queries);
    output().print_per_line(&res);
}

fn solve_impl(s: Str, queries: Vec<(Str, usize)>) -> Vec<Option<usize>> {
    let mut res = Vec::new();
    HashBase::init();
    let h = SimpleHash::new(s.as_slice());
    let mut ans: Vec<DefaultMap<i64, Vec<usize>>> = vec![DefaultMap::new(); s.len() + 1];
    for (sample, index) in queries {
        let index = index - 1;

        if sample.len() > s.len() {
            res.push(None);
            continue;
        }
        if ans[sample.len()].is_empty() {
            for i in 0..=s.len() - sample.len() {
                ans[sample.len()][h.hash(i..i + sample.len())].push(i + 1);
            }
        }
        let c_hash = hash(sample.as_slice());
        if ans[sample.len()][c_hash].len() <= index {
            res.push(None);
        } else {
            res.push(Some(ans[sample.len()][c_hash][index]));
        }
    }
    res
}

#[test]
fn test() {
    let mut s = Str::new();
    for _ in 0..200000 {
        s += b'a';
    }
    let mut queries: Vec<(Str, usize)> = Vec::new();
    let mut sum = 0;
    let mut len = 1;
    while sum + len <= 200000 {
        queries.push((Str::from(&s[..len]).clone(), 1));
        sum += len;
        len += 1;
    }
    solve_impl(s.clone(), queries);
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
