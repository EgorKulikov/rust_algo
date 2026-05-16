//{"name":"H. Crystalfly","group":"Yandex - Stage 9: Grand Prix of Nanjing","url":"https://official.contest.yandex.ru/opencupXXII/contest/33444/problems/H/","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n5\n1 10 100 1000 10000\n1 2 1 1 1\n1 2\n1 3\n2 4\n2 5\n5\n1 10 100 1000 10000\n1 3 1 1 1\n1 2\n1 3\n2 4\n2 5\n","output":"10101\n10111\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HCrystalfly"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a: Vec<i64> = input.read_vec(n);
    let t: Vec<u8> = input.read_vec(n);
    let edges: Vec<(usize, usize)> = input.read_vec(n - 1);

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u - 1, BiEdge::new(v - 1));
    }
    let mut dfs = RecursiveFunction2::new(|f, vert, prev| -> (i64, i64, i64) {
        let mut take = a[vert];
        let mut take_and_return = a[vert];
        let mut calls = Vec::new();
        for e in graph[vert].iter() {
            if e.to() == prev {
                continue;
            }
            calls.push((f.call(e.to(), vert), e.to()));
        }
        let mut max_delta_take = 0;
        let mut max_delta_take_and_return: i64 = 0;
        let mut second_delta_take_and_return: i64 = 0;
        let mut sum_not_take: i64 = 0;
        for ((c_take, c_not_take, c_take_and_return), _) in calls.iter() {
            take += c_not_take;
            sum_not_take += c_not_take;
            max_delta_take.maxim(c_take - c_not_take);
            take_and_return += c_not_take;
            let cur_delta_take_and_return = c_take_and_return - c_not_take;
            if cur_delta_take_and_return > max_delta_take_and_return {
                second_delta_take_and_return = max_delta_take_and_return;
                max_delta_take_and_return = cur_delta_take_and_return;
            } else {
                second_delta_take_and_return.maxim(cur_delta_take_and_return);
            }
        }
        take += max_delta_take;
        for ((c_take, c_not_take, c_take_and_return), i) in calls {
            if t[i] == 3 {
                if c_take_and_return - c_not_take == max_delta_take_and_return {
                    take.maxim(
                        a[vert] + c_take + sum_not_take - c_not_take + second_delta_take_and_return,
                    );
                } else {
                    take.maxim(
                        a[vert] + c_take + sum_not_take - c_not_take + max_delta_take_and_return,
                    );
                }
            }
        }
        (take, take - a[vert], take_and_return)
    });
    out_line!(dfs.call(0, 0).0);
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
