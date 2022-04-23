//{"name":"F1. Перемешивание массива","group":"Codeforces - Codeforces Global Round 20","url":"https://codeforces.com/contest/1672/problem/F1","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2\n2 1\n4\n1 2 3 3\n","output":"1 2\n3 3 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F1PeremeshivanieMassiva"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n).dec_by_one();

    let mut pos = vec![Vec::new(); n];
    for (i, a) in a.into_iter().enumerate() {
        pos[a].push(i);
    }

    struct V(Vec<usize>);
    impl PartialEq for V {
        fn eq(&self, other: &Self) -> bool {
            self.0.len() == other.0.len()
        }
    }
    impl PartialOrd for V {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.len().partial_cmp(&other.0.len())
        }
    }
    impl Eq for V {}
    impl Ord for V {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.len().cmp(&other.0.len())
        }
    }

    let mut heap = BinaryHeap::new();
    for (i, p) in pos.into_iter().enumerate() {
        if !p.is_empty() {
            heap.push((V(p), i + 1));
        }
    }
    let mut ans = vec![0; n];
    while let Some((V(v), i)) = heap.pop() {
        let mut cur = vec![(v, i)];
        while let Some((V(v), i)) = heap.pop() {
            if v.len() == cur[0].0.len() || cur.len() == 1 {
                cur.push((v, i));
            } else {
                heap.push((V(v), i));
                break;
            }
        }
        for i in 0..cur.len() - 1 {
            ans[*cur[i].0.last().unwrap()] = cur[i + 1].1;
        }
        ans[*cur[cur.len() - 1].0.last().unwrap()] = cur[0].1;
        for (mut v, i) in cur {
            v.pop();
            if !v.is_empty() {
                heap.push((V(v), i));
            }
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
    let test_type = TestType::MultiNumber;
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
