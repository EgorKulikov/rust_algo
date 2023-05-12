//{"name":"Xor over tree","group":"HackerEarth - April Circuits '23","url":"https://www.hackerearth.com/challenges/competitive/april-circuits-23/approximate/xor-over-tree-29dfb939/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4 1 9\n","output":"1 2\n2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"XorOverTree"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::random::random;
use algo_lib::misc::time_tracker::TimeTracker;
use std::time::Duration;

fn solve(input: &mut Input, _test_case: usize) {
    let tt = TimeTracker::new();

    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| -a[i]);
    let mut ans = Vec::with_capacity(n - 1);
    if order[0] != 0 {
        ans.push((1, order[0] + 1));
    }
    let mut left = order[0];
    let mut right = order[0];
    for &i in order.iter().skip(1) {
        if i == 0 {
            continue;
        }
        ans.push((left + 1, i + 1));
        left = right;
        right = i;
    }

    fn score(a: &[i64], ans: &[(usize, usize)]) -> i64 {
        let mut graph = Graph::new(a.len());
        for &(u, v) in ans {
            graph.add_edge(u - 1, BiEdge::new(v - 1));
        }
        let lca = graph.lca();
        let mut res = 0;
        for i in 0..a.len() {
            for j in i + 1..a.len() {
                let l = lca.lca(i, j);
                let s = (lca.level(i) + lca.level(j) - 2 * lca.level(l)) as i64;
                res += (s ^ a[l]) + s * a[l];
            }
        }
        res
    }
    let mut sc = score(&a, &ans);
    let mut deg = vec![0; n];

    let r = random();

    while tt.elapsed() < Duration::from_millis(2900) {
        let mut update = false;
        for &(u, v) in &ans {
            deg[u - 1] += 1;
            deg[v - 1] += 1;
        }
        for x in 1..=n {
            for y in 1..x {
                if r.next(100) != 0 {
                    continue;
                }
                let mut cand = ans.clone();
                for (u, v) in &mut cand {
                    if *u == x {
                        *u = y;
                    } else if *u == y {
                        *u = x;
                    }
                    if *v == x {
                        *v = y;
                    } else if *v == y {
                        *v = x;
                    }
                }
                let s = score(&a, &cand);
                if s > sc {
                    sc = s;
                    ans = cand;
                    update = true;
                    break;
                }
            }
        }

        /*for i in 0..n - 1 {
            let (x, y) = ans[i];
            if deg[x - 1] == 1 {
                for j in 0..n {
                    let mut cand = ans.clone();
                    if j != x - 1 && j != y - 1 {
                        cand[i].1 = j + 1;
                        let s = score(&a, &cand);
                        if s > sc {
                            sc = s;
                            ans = cand;
                            update = true;
                            break;
                        }
                    }
                }
                if update {
                    break;
                }
            }
            if deg[y - 1] == 1 {
                for j in 0..n {
                    let mut cand = ans.clone();
                    if j != x - 1 && j != y - 1 {
                        cand[i].0 = j + 1;
                        let s = score(&a, &cand);
                        if s > sc {
                            sc = s;
                            ans = cand;
                            update = true;
                            break;
                        }
                    }
                }
                if update {
                    break;
                }
            }
        }*/
        if !update {
            break;
        }
    }

    output().print_per_line(&ans);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
