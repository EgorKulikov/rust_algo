//{"name":"day_23","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_23"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::id::Id;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::treap::payload::Pushable;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::scan;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut data = Vec::new();
    let mut id = Id::new();
    while !input.is_empty() {
        scan!(input, "@-@", c1: Str, c2: Str);
        id.get(c1.clone());
        id.get(c2.clone());
        data.push((c1, c2));
    }

    let mut graph = Graph::new(id.len());
    for (c1, c2) in data {
        let c1 = id.get(c1);
        let c2 = id.get(c2);
        graph.add_edge(BiEdge::new(c1, c2));
    }

    // part 1
    {
        let mut ts = Vec::new();
        for (k, v) in id.iter() {
            if k[0] == b't' {
                ts.push(*v);
            }
        }
        let mut ans = 0;
        for i in 0..id.len() {
            for e1 in &graph[i] {
                if e1.to() >= i {
                    continue;
                }
                for e2 in &graph[i] {
                    if e1.to() == e2.to() {
                        break;
                    }
                    if e2.to() >= i {
                        continue;
                    }
                    if !ts.contains(&e1.to()) && !ts.contains(&e2.to()) && !ts.contains(&i) {
                        continue;
                    }
                    for e3 in &graph[e1.to()] {
                        if e3.to() == e2.to() {
                            ans += 1;
                            break;
                        }
                    }
                }
            }
        }
        out.print_line(ans);
    }

    // part 2
    {
        let arr = id.by_id();
        let mut sets = vec![BitSet::new(id.len()); id.len()];
        for i in 0..id.len() {
            for e in &graph[i] {
                sets[i].set(e.to());
            }
        }
        let mut ans = 0;
        let mut best = Vec::new();
        let mut rec = RecursiveFunction2::new(|f, step: usize, v: Vec<usize>| {
            if step == id.len() {
                if v.len() > ans {
                    ans = v.len();
                    best = v;
                }
                return;
            }
            let mut good = true;
            for v in v.copy_iter() {
                if !sets[step][v] {
                    good = false;
                    break;
                }
            }
            if good {
                let mut next = v.clone();
                next.push(step);
                f.call(step + 1, next);
            }
            f.call(step + 1, v);
        });
        rec.call(0, Vec::new());
        let best = best
            .iter_map(|x| arr[x].clone())
            .collect::<Vec<_>>()
            .sorted();
        out.set_separator(b',');
        out.print_line(best);
    }
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
