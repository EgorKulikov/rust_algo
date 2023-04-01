//{"name":"L. Chemical Lab","group":"Codeforces - Constructor Open Cup 2023","url":"https://constructor2023.contest.codeforces.com/group/sVRDLercWX/contest/431163/problem/L","interactive":false,"timeLimit":5000,"tests":[{"input":"3 3\n1 2 3\n4 5 6\n","output":"4\n"},{"input":"2 3\n3 4\n1 6 8\n","output":"-1\n"},{"input":"5 10\n2 3 5 6 8\n3 15 2 15 14 5 15 11 8 9\n","output":"29\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LChemicalLab"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n);
    let r = input.read_size_vec(m);

    let max = *r.iter().max().unwrap();
    let mut states = vec![Vec::new()];
    let mut edges = vec![Vec::new()];
    let mut parent = vec![0];
    let mut sum = Vec::new();
    let mut at = 0;
    let mut vert = vec![Vec::new(); max + 1];
    while at < states.len() {
        let cur = states[at].clone();
        let last = cur.last().copied().unwrap_or(a[a.len() - 1]);
        let s = cur.iter().sum::<usize>();
        vert[s].push(at);
        sum.push(s);
        for &i in &a {
            if i <= last && s + i <= max {
                let mut next = cur.clone();
                next.push(i);
                edges[at].push(states.len());
                states.push(next);
                edges.push(Vec::new());
                parent.push(at);
            }
        }
        at += 1;
    }
    for &i in &r {
        if vert[i].is_empty() {
            out_line!(-1);
            return;
        }
    }
    const INF: usize = usize::MAX - 1;
    let mut dist = vec![INF; states.len()];
    dist[0] = 0;
    let mut was = 0;
    for i in r {
        for j in 0..states.len() {
            if sum[j] != was {
                dist[j] = INF;
            }
        }
        for j in (1..states.len()).rev() {
            let k = parent[j];
            dist[k] = dist[k].min(dist[j] + 1);
        }
        for j in 0..states.len() {
            for &k in &edges[j] {
                dist[k] = dist[k].min(dist[j] + 1);
            }
        }
        was = i;
    }
    let mut ans = INF;
    for i in vert[was].iter() {
        ans = ans.min(dist[*i]);
    }
    out_line!(ans);
}

#[test]
fn test() {
    let a = (1..=15).collect_vec();
    let mut states = vec![Vec::new()];
    let mut at = 0;
    while at < states.len() {
        let cur = states[at].clone();
        let last = cur.last().copied().unwrap_or(a[a.len() - 1]);
        let s = cur.iter().sum::<usize>();
        for &i in &a {
            if i <= last && s + i <= 45 {
                let mut next = cur.clone();
                next.push(i);
                states.push(next);
            }
        }
        at += 1;
    }
    println!("{}", states.len());
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
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
