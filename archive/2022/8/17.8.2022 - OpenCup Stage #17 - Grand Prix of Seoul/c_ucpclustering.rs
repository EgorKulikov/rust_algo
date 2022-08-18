//{"name":"C. UCP-Clustering","group":"Yandex - Stage 17: Grand Prix of Seoul","url":"https://official.contest.yandex.com/opencupXXII/contest/39021/problems/C/","interactive":false,"timeLimit":5000,"tests":[{"input":"4\n0 0\n0 3\n3 0\n3 3\n","output":"0.000000 0.000000 3.000000 3.000000 1.000000000000\n0.000000 1.500000 3.000000 1.500000 2.000000000000\n0.000000 3.000000 3.000000 0.000000 1.000000000000\n1.500000 0.000000 1.500000 3.000000 2.000000000000\n"},{"input":"64\n18 39\n-45 -13\n28 -62\n-13 -3\n-3 9\n-4 -50\n-51 -31\n46 -37\n-41 -30\n-52 38\n5 48\n-62 -50\n45 -16\n-59 -9\n26 12\n-40 16\n41 17\n26 26\n-1 -8\n-22 -22\n15 -14\n62 41\n28 -1\n-14 -51\n22 -2\n-36 8\n-34 -33\n8 59\n10 44\n-38 0\n-57 1\n-33 -42\n43 -20\n-60 -6\n14 19\n-11 43\n46 55\n-36 -26\n13 -4\n9 -19\n26 -1\n22 56\n0 -53\n-19 -59\n-17 -3\n54 -64\n5 -41\n-49 -34\n-38 29\n-22 48\n-32 -1\n22 53\n63 -17\n-3 -28\n2 56\n46 -53\n7 62\n-63 56\n-29 5\n-20 -45\n26 -26\n54 -36\n-33 29\n62 2\n","output":"-36.000000 -7.500000 22.000000 -1.000000 3.894736842105\n-36.000000 -4.500000 22.000000 -1.500000 7.674134419552\n-35.000000 -17.500000 24.000000 5.500000 6.251798561151\n-35.000000 -11.000000 24.000000 0.500000 4.000000000000\n-35.000000 -7.500000 24.000000 -1.000000 2.666666666667\n-34.000000 -9.000000 26.000000 -1.000000 5.294871794872\n-30.500000 38.500000 11.000000 -24.000000 3.948051948052\n-25.500000 38.500000 7.000000 -24.000000 2.000000000000\n-21.000000 -24.000000 22.000000 40.000000 3.157894736842\n-20.000000 -26.000000 22.000000 39.000000 2.000000000000\n-20.000000 -22.000000 22.000000 41.000000 4.873239436620\n-19.000000 -26.000000 18.000000 38.000000 3.580000000000\n-15.500000 -18.000000 16.000000 46.000000 2.705882352941\n-15.000000 29.000000 11.000000 -29.000000 3.240000000000\n-13.500000 -21.000000 12.000000 42.000000 2.000000000000\n-13.000000 -20.000000 10.000000 43.000000 4.053231939163\n-12.000000 27.500000 7.000000 -30.500000 2.410256410256\n-11.000000 29.000000 5.000000 -30.000000 2.000000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CUCPClustering"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::topological_sort::TopologicalSort;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::rational::Rational;
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut points = input.read_long_pair_vec(n);

    points.sort_unstable();
    for (x, y) in &mut points {
        *x *= 2;
        *y *= 2;
    }
    let mut points_by_y = points.clone();
    points_by_y.sort_unstable_by_key(|&(_, y)| y);
    let mut id = HashMap::new();
    let mut state = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            id.insert(
                (points[i].0, points[i].1, points[j].0, points[j].1),
                state.len(),
            );
            state.push((points[i].0, points[i].1, points[j].0, points[j].1));
        }
    }
    let mut i = 0;
    let mut edge = Vec::new();
    fn median(v: Vec<i64>) -> i64 {
        // v.sort_unstable();
        if v.len() % 2 == 0 {
            (v[v.len() / 2 - 1] + v[v.len() / 2]) / 2
        } else {
            v[v.len() / 2]
        }
    }
    while i < state.len() {
        let (x1, y1, x2, y2) = state[i];
        let mut left_x = Vec::new();
        let mut left_y = Vec::new();
        let mut right_x = Vec::new();
        let mut right_y = Vec::new();
        for j in 0..n {
            let (x, y) = points[j];
            let dx1 = x1 - x;
            let dy1 = y1 - y;
            let dx2 = x2 - x;
            let dy2 = y2 - y;
            let d1 = dx1 * dx1 + dy1 * dy1;
            let d2 = dx2 * dx2 + dy2 * dy2;
            if d1 <= d2 {
                left_x.push(x);
            } else {
                right_x.push(x);
            }
        }

        for j in 0..n {
            let (x, y) = points_by_y[j];
            let dx1 = x1 - x;
            let dy1 = y1 - y;
            let dx2 = x2 - x;
            let dy2 = y2 - y;
            let d1 = dx1 * dx1 + dy1 * dy1;
            let d2 = dx2 * dx2 + dy2 * dy2;
            if d1 <= d2 {
                left_y.push(y);
            } else {
                right_y.push(y);
            }
        }

        let mut nx1 = median(left_x);
        let mut ny1 = median(left_y);
        let mut nx2 = median(right_x);
        let mut ny2 = median(right_y);
        if nx1 > nx2 || (nx1 == nx2 && ny1 > ny2) {
            std::mem::swap(&mut nx1, &mut nx2);
            std::mem::swap(&mut ny1, &mut ny2);
        }
        if (x1, y1, x2, y2) == (nx1, ny1, nx2, ny2) {
            edge.push(None)
        } else {
            match id.get(&(nx1, ny1, nx2, ny2)) {
                Some(id) => edge.push(Some(*id)),
                None => {
                    id.insert((nx1, ny1, nx2, ny2), state.len());
                    edge.push(Some(state.len()));
                    state.push((nx1, ny1, nx2, ny2));
                }
            }
        }
        i += 1;
    }
    let mut graph = Graph::new(state.len());
    for (i, &e) in edge.iter().enumerate() {
        if let Some(id) = e {
            graph.add_edge(i, Edge::new(id));
        }
    }
    let order = graph.topological_sort().unwrap();
    let mut ans = Vec::new();
    let mut ways = vec![0i64; state.len()];
    ways[0..n * (n - 1) / 2].fill(1);
    let mut sum = vec![0i64; state.len()];
    for i in order {
        match edge[i] {
            None => {
                let (x1, y1, x2, y2) = state[i];
                ans.push((x1, y1, x2, y2, Rational::new(sum[i], ways[i])));
            }
            Some(to) => {
                let add = ways[i];
                ways[to] += add;
                let add = sum[i] + ways[i];
                sum[to] += add;
            }
        }
    }
    ans.sort_unstable();
    let ans = ans
        .into_iter()
        .map(|(x, y, x2, y2, r)| {
            (
                x as f64 / 2.,
                y as f64 / 2.,
                x2 as f64 / 2.,
                y2 as f64 / 2.,
                (r.num() as f64) / (r.den() as f64) + 1.,
            )
        })
        .collect_vec();
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
}
//END MAIN
