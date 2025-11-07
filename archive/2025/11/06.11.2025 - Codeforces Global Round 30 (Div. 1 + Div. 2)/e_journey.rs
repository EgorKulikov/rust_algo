//{"name":"E. Journey","group":"Codeforces - Codeforces Global Round 30 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2164/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n5 6\n2 4 15\n2 5 4\n1 3 6\n2 3 9\n1 2 10\n3 4 7\n4 3\n1 2 3\n1 3 2\n1 4 1\n2 3\n1 2 1\n2 1 3\n1 1 4\n6 6\n2 3 10\n1 3 10\n5 6 10\n6 6 1\n4 5 10\n3 4 10\n5 5\n1 2 4\n5 1 5\n4 3 6\n2 4 10\n1 4 7\n","output":"58\n8\n8\n71\n43\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeMap;
use std::mem::take;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, i64)>(m).dec();

    let mut ans = 0;
    let mut degree = vec![0; n];
    for i in (0..m).rev() {
        let (u, v, w) = edges[i];
        ans += w;
        degree[u] += 1;
        degree[v] += 1;
    }
    let mut e = vec![BTreeMap::new(); n];
    let mut free = vec![0; n];
    for i in 0..n {
        if degree[i] % 2 == 1 {
            free[i] = 1;
        }
    }
    let mut dsu = DSU::new(n);
    for i in 0..m {
        let (u, v, w) = edges[i];
        let uu = dsu.find(u);
        let vv = dsu.find(v);
        let f = free[uu] + free[vv];
        if dsu.union(u, v) {
            if e[uu].len() < e[vv].len() {
                e.swap(uu, vv);
            }
            let set = take(&mut e[vv]);
            for (key, val) in set {
                *e[uu].entry(key).or_insert(0) += val;
            }
            free[dsu.find(u)] = f;
        }
        let u = dsu.find(u);
        if free[u] >= 2 {
            free[u] -= 2;
            *e[uu].entry(w).or_insert(0) += 1;
        }
        while let Some((key, val)) = e[u].pop_last() {
            if key <= w {
                e[u].insert(key, val);
                break;
            }
            *e[u].entry(w).or_insert(0) += val;
        }
    }
    for (k, v) in e[dsu.find(0)].iter() {
        ans += k * *v;
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
        TaskType::Classic | TaskType::RunTwice => input.is_empty(),
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
