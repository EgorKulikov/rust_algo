use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::Graph;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(n).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut degree = graph.degrees();
    let mut queue = Vec::new();
    for i in 0..n {
        if degree[i] == 1 {
            queue.push(i);
        }
    }
    let mut ans = 0;
    while let Some(vert) = queue.pop() {
        assert_eq!(degree[vert], 1);
        degree[vert] = 0;
        let mut next = n;
        for e in graph.adj(vert) {
            if degree[e.to()] != 0 {
                next = e.to();
                break;
            }
        }
        assert_ne!(next, n);
        if a[vert] >= a[next] || a[vert] == 0 {
            out.print_line(-1);
            return;
        }
        ans.maxim(a[vert]);
        a[next] -= a[vert];
        degree[next] -= 1;
        if degree[next] == 1 {
            queue.push(next);
        }
    }
    let mut c = Vec::new();
    for i in 0..n {
        if degree[i] != 0 {
            assert_eq!(degree[i], 2);
            let mut cur = i;
            let mut prev = n;
            loop {
                c.push(a[cur]);
                let mut next = n;
                for e in graph.adj(cur) {
                    if degree[e.to()] == 2 && e.to() != prev {
                        next = e.to();
                        break;
                    }
                }
                assert_ne!(next, n);
                if next == i {
                    break;
                }
                prev = cur;
                cur = next;
            }
            break;
        }
    }
    assert!(!c.is_empty());
    if c.len() % 2 == 1 {
        let mut sum = 0;
        for i in c.indices() {
            if i % 2 == 0 {
                sum += c[i];
            } else {
                sum -= c[i];
            }
        }
        if sum % 2 != 0 {
            out.print_line(-1);
            return;
        }
        let mut cur = sum / 2;
        for i in c.indices() {
            if cur <= 0 {
                out.print_line(-1);
                return;
            }
            ans.maxim(cur);
            cur = c[i] - cur;
        }
        assert_eq!(cur, sum / 2);
    } else {
        let mut val = 0;
        let mut min_add = 0;
        let mut max_add = 0;
        let mut min_sub = c[0];
        let mut max_sub = c[0];
        for i in c.indices() {
            if i % 2 == 0 {
                min_add.minim(val);
                max_add.maxim(val);
            } else {
                min_sub.minim(val);
                max_sub.maxim(val);
            }
            val = c[i] - val;
        }
        if val != 0 {
            out.print_line(-1);
            return;
        }
        let min_x = 1 - min_add;
        let max_x = min_sub - 1;
        if min_x > max_x {
            out.print_line(-1);
            return;
        }
        let mut x = (max_sub - max_add) / 2;
        x.maxim(min_x);
        x.minim(max_x);
        ans.maxim(x + max_add);
        ans.maxim(max_sub - x);
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
