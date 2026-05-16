//{"name":"U503147 西湖有雅座","group":"Luogu","url":"https://www.luogu.com.cn/problem/U503147?contestId=218668","interactive":false,"timeLimit":500,"tests":[{"input":"3 2 2\n0 1\n1 1\n1 0\n0 0\n0 1\n0 1\n","output":"2\n"},{"input":"3 2 2\n0 1\n1 0\n0 0\n0 1\n1 0\n0 0\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"U503147"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::gen::VecGen;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let h = input.read_size();
    let w = input.read_size();
    let parts = Vec::gen(n, |_, _| input.read_int_vec(h * w));

    let mut graph = Graph::new(n);
    let mut s = Vec::with_capacity(n);
    for i in 0..n {
        s.push(parts[i].copy_sum());
        for j in 0..i {
            let f = parts[i]
                .copy_zip(&parts[j])
                .filter(|&(a, b)| a == 1 && b == 1)
                .count() as i32;
            if f < s[i].min(s[j]).upper_div(2) {
                graph.add_edge(BiEdge::new(i, j));
            }
        }
    }
    let mut color = vec![0; n];
    let mut ans = 0;
    for i in 0..n {
        if color[i] != 0 {
            continue;
        }
        let mut good = true;
        let mut white = 0;
        let mut black = 0;
        let mut dfs = RecursiveFunction2::new(|dfs, vert, c| {
            if color[vert] != 0 {
                if color[vert] != c {
                    good = false;
                }
                return;
            }
            color[vert] = c;
            if c == 1 {
                white += 1;
            } else {
                black += 1;
            }
            for e in &graph[vert] {
                dfs.call(e.to(), -c);
            }
        });
        dfs.call(i, 1);
        if !good {
            out.print_line(-1);
            return;
        }
        ans += white.max(black);
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
