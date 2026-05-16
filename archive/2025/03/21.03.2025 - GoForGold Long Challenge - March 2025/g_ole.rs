//{"name":"G. OLE","group":"Codeforces - GoForGold Long Challenge - March 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/596267/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n","output":"2\n2 1 1\n"},{"input":"0\n","output":"2\n1 1\n"},{"input":"7\n","output":"4\n2 1 1\n4 1 2 0 0\n3 3 1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::primes::prime::is_prime;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut prefix = Str::new();
    let mut num_primes = 0;
    for k in 0..=n / 2 {
        let mut graph = Graph::new(2 * k + 2);
        let source = 2 * k;
        let sink = source + 1;
        for i in 0..k {
            graph.add_edge(FlowEdge::new(source, i, 1));
            graph.add_edge(FlowEdge::new(k + i, sink, 1));
            for j in 0..k {
                if (n - i) % (j + 1) == 0 {
                    graph.add_edge(FlowEdge::new(i, k + j, 1));
                }
            }
        }
        if graph.max_flow(source, sink) == k as i64 {
            prefix.push(b'1');
        } else {
            prefix.push(b'0');
        }
        if is_prime(n - k) {
            num_primes += 1;
            if num_primes >= 2 {
                break;
            }
        }
    }
    if prefix.len() == n / 2 + 1 {
        let mut suf = prefix.clone();
        suf.reverse();
        if n % 2 == 0 {
            prefix.pop();
        }
        prefix += &suf;
        out.print_line(2);
        let mut ans = Vec::with_capacity(n + 1);
        for i in 0..=n {
            if prefix[i] == b'1' {
                ans.push(1);
            } else {
                ans.push(0);
            }
        }
        out.print_line((ans.len(), ans));
        return;
    }
    let zeroes = n + 1 - 2 * prefix.len();
    let mut ans = Vec::new();
    ans.push(vec![0, 0]);
    let mut cur = 2;
    while cur * 2 <= zeroes {
        ans.push(vec![ans.len() + 1, ans.len() + 1]);
        cur *= 2;
    }
    let mut cur = Vec::new();
    for c in prefix.copy_iter() {
        if c == b'1' {
            cur.push(1);
        } else {
            cur.push(0);
        }
    }
    if zeroes.is_set(0) {
        cur.push(0);
    }
    for i in 1..60 {
        if zeroes.is_set(i) {
            cur.push(i + 1);
        }
    }
    for c in prefix.copy_rev() {
        if c == b'1' {
            cur.push(1);
        } else {
            cur.push(0);
        }
    }
    ans.push(cur);
    out.print_line(ans.len() + 1);
    for row in ans {
        out.print_line((row.len(), row));
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

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
