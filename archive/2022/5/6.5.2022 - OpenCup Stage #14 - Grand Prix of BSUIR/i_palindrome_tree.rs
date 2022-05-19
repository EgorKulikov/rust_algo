//{"name":"I. Palindrome tree","group":"Yandex - Grand Prix of BSUIR","url":"https://official.contest.yandex.com/opencupXXII/contest/37753/problems/I/","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\naba\n1 2\n1 3\n","output":"3\n1 2 3\n"},{"input":"3 2\naba\n1 2\n2 3\n","output":"2\n1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IPalindromeTree"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::hash::{HashBase, SimpleHash, StringHash};
use algo_lib::string::string::Str;
use algo_lib::string::string_algorithms::StringAlgorithms;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let s: Str = input.read();
    let edges = input.read_usize_pair_vec(n - 1).dec_by_one();

    HashBase::init();
    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let lca = graph.lca();
    let mut palindromes: Vec<Vec<(u16, u16)>> = vec![Vec::new(); n];
    let mut enabled = BitSet::new(n);
    let mut ans = Vec::new();
    ans.push(1);
    enabled.set(0, true);
    for i in 1..n {
        if enabled[i] {
            ans.push(i + 1);
            continue;
        }
        let mut path_down = Vec::new();
        let mut cur = i;
        while !enabled[cur] {
            path_down.push(cur);
            cur = lca.parent(cur).unwrap();
        }
        let contact_point = cur;
        path_down.reverse();
        let path = path_down;
        let mut tail = Str::new();
        let mut good = true;
        for &j in &path {
            if j < i {
                good = false;
                break;
            }
            tail += s[j];
        }
        if !good {
            continue;
        }
        let mut check =
            RecursiveFunction3::new(|f, vert: usize, prev: usize, pos: usize| -> bool {
                if lca.path_length(vert, contact_point) + pos >= k {
                    return false;
                }
                if pos == tail.len() {
                    return true;
                }
                for e in &graph[vert] {
                    if e.to() != prev
                        && enabled[e.to()]
                        && s[e.to()] == tail[pos]
                        && !f.call(e.to(), vert, pos + 1)
                    {
                        return false;
                    }
                }
                true
            });
        for &(j, k) in &palindromes[contact_point] {
            if !check.call(j.into_usize(), k.into_usize(), 0) {
                good = false;
                break;
            }
        }
        if !good {
            continue;
        }
        let hash = SimpleHash::new(tail.as_slice());
        let mut rev_tail = tail.clone();
        rev_tail.reverse();
        let rev_hash = SimpleHash::new(rev_tail.as_slice());
        if !check.call(contact_point, contact_point, 0) {
            continue;
        }
        let op = tail.odd_palindromes();
        for i in op {
            if 2 * i - 1 > k {
                good = false;
                break;
            }
        }
        if !good {
            continue;
        }
        let ep = tail.even_palindromes();
        for i in ep {
            if 2 * i > k {
                good = false;
                break;
            }
        }
        if !good {
            continue;
        }
        for i in 0..tail.len() {
            if tail[i] == s[contact_point]
                && hash.hash(..i) == rev_hash.hash(tail.len() - i..)
                && !check.call(contact_point, contact_point, i + 1)
            {
                good = false;
                break;
            }
        }
        if !good {
            continue;
        }
        let mut last = contact_point;
        for &j in &path {
            let mut new_palindromes = Vec::new();
            for &(k, p) in &palindromes[last] {
                let k = k.into_usize();
                let p = p.into_usize();
                for e in &graph[k] {
                    if enabled[e.to()] && e.to() != p && s[e.to()] == s[j] {
                        new_palindromes.push((e.to(), k));
                        // palindromes[j].push((e.to(), k));
                        // palindromes[e.to()].push(((j, last)));
                    }
                }
            }
            if s[j] == s[last] {
                new_palindromes.push((last, j));
                // palindromes[j].push((last, j));
                // palindromes[last].push((j, last));
            }
            for e in &graph[last] {
                if enabled[e.to()] && s[e.to()] == s[j] {
                    new_palindromes.push((e.to(), last));
                    // palindromes[j].push((e.to(), last));
                    // palindromes[e.to()].push(((j, last)));
                }
            }
            for (b, c) in new_palindromes {
                palindromes[j].push((b.into_u16(), c.into_u16()));
                palindromes[b].push((j.into_u16(), last.into_u16()));
            }
            enabled.set(j, true);
            last = j;
        }
        ans.push(i + 1);
    }
    out_line!(ans.len());
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
