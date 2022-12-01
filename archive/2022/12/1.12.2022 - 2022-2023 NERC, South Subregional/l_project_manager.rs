//{"name":"L. Project Manager","group":"Codeforces - 2022-2023 ICPC, NERC, Южный четвертьфинал (онлайн-трансляция, правила ICPC, командное соревнование)","url":"https://codeforces.com/contest/1765/problem/L","interactive":false,"timeLimit":4000,"tests":[{"input":"3 5 4\n2 Saturday Sunday\n2 Tuesday Thursday\n4 Monday Wednesday Friday Saturday\n4 7 13 14 15\n5 1 1 3 3 2\n3 2 3 2\n5 3 3 3 1 1\n8 3 3 3 3 3 3 3 3\n","output":"25 9 27 27\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LProjectManager"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let mut days = HashMap::new();
    days.insert("Monday", 0);
    days.insert("Tuesday", 1);
    days.insert("Wednesday", 2);
    days.insert("Thursday", 3);
    days.insert("Friday", 4);
    days.insert("Saturday", 5);
    days.insert("Sunday", 6);

    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let mut works = Vec::with_capacity(n);
    for _ in 0..n {
        let d = input.read_size();
        let mut cur = 0;
        for _ in 0..d {
            let day = input.read_string();
            cur.set_bit(days[day.as_str()]);
        }
        works.push(cur);
    }
    let h = input
        .read_size_vec(m)
        .dec_by_one()
        .into_iter()
        .collect::<HashSet<_>>();
    let mut next = (0..7).collect_vec();
    for i in 0..7 {
        while h.contains(&next[i]) {
            next[i] += 7;
        }
    }
    let mut available: Vec<Vec<usize>> = Vec::new();
    let mut projects = Vec::with_capacity(k);
    for _ in 0..k {
        let d = input.read_size();
        let mut cur = Vec::new();
        for _ in 0..d {
            let day = input.read_size() - 1;
            cur.push(day);
        }
        projects.push(cur);
    }

    let mut works_on = vec![BTreeSet::new(); n];
    let mut stage = vec![0; k];
    for i in 0..k {
        if works_on[projects[i][0]].is_empty() {
            let mut nxt = None;
            for j in 0..7 {
                if works[projects[i][0]].is_set(j) {
                    nxt.minim(next[j]);
                }
            }
            let nxt = nxt.unwrap();
            available.resize(available.len().max(nxt + 1), Vec::new());
            available[nxt].push(projects[i][0]);
        }
        works_on[projects[i][0]].insert(i);
    }
    let mut ans = vec![0; k];
    let mut solved = 0;
    for i in 0.. {
        if h.contains(&i) {
            continue;
        }
        next[i % 7] = i + 7;
        while h.contains(&next[i % 7]) {
            next[i % 7] += 7;
        }
        let mut new_work = Vec::new();
        let mut vec = Vec::new();
        swap(&mut available[i], &mut vec);
        for j in vec {
            let p = *works_on[j].iter().next().unwrap();
            works_on[j].remove(&p);
            stage[p] += 1;
            if stage[p] == projects[p].len() {
                ans[p] = i + 1;
                solved += 1;
            } else {
                new_work.push(p);
            }
            if !works_on[j].is_empty() {
                let mut nxt = None;
                for l in 0..7 {
                    if works[j].is_set(l) {
                        nxt.minim(next[l]);
                    }
                }
                let nxt = nxt.unwrap();
                let len = available.len();
                available.resize(len.max(nxt + 1), Vec::new());
                available[nxt].push(j);
            }
        }
        for p in new_work {
            let j = projects[p][stage[p]];
            if works_on[j].is_empty() {
                let mut nxt = None;
                for l in 0..7 {
                    if works[j].is_set(l) {
                        nxt.minim(next[l]);
                    }
                }
                let nxt = nxt.unwrap();
                available.resize(available.len().max(nxt + 1), Vec::new());
                available[nxt].push(j);
            }
            works_on[j].insert(p);
        }
        if solved == k {
            break;
        }
    }
    out_line!(ans);
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
