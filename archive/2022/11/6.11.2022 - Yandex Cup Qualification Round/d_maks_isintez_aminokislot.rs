//{"name":"D. Макс и синтез аминокислот","group":"Yandex - Yandex Cup 2022: Алгоритм, спринт (квалификация)","url":"https://contest.yandex.ru/yacup/contest/42199/problems/D/","interactive":false,"timeLimit":5000,"tests":[{"input":"3 5\nGCAGT\nACAGA\nGCAGA\n","output":"6\n"},{"input":"8 12\nCGGCCGCAACTC\nGGATGTTTGCTG\nTTGAAACGTACG\nGCTGCGGGGGTC\nAGGTACTCACAA\nACATATTTCTGG\nGGAGTGGAGGTC\nAACACGTAGCCC\n","output":"72\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMaksISintezAminokislot"}}}

use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::minimal_spanning_tree::MinimalSpanningTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let l = input.read_usize();
    let mut a = input.read_vec::<Str>(n);

    a.push(vec![b'A'; l].into());
    let mut graph = Graph::new(n + 1);
    for i in 0..=n {
        for j in 0..i {
            let mut dist = 0;
            for (a, b) in a[i].iter().zip(a[j].iter()) {
                if a != b {
                    if (a, b) == (b'A', b'G')
                        || (a, b) == (b'G', b'A')
                        || (a, b) == (b'C', b'T')
                        || (a, b) == (b'T', b'C')
                    {
                        dist += 2;
                    } else {
                        dist += 1;
                    }
                }
            }
            graph.add_edge(i, BiWeightedEdge::new(j, dist));
        }
    }
    let tree = graph.minimal_spanning_tree();
    let mut ans = 0;
    for i in 0..=n {
        for e in &tree[i] {
            ans += e.weight();
        }
    }
    ans /= 2;
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
