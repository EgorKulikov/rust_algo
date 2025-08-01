//{"name":"E. Запросы на индуцированных подграфах","group":"Codeforces - Codeforces Round 1040 (Div. 1)","url":"https://codeforces.com/contest/2129/problem/E","interactive":false,"timeLimit":5000,"tests":[{"input":"2\n4 5\n1 3\n1 4\n2 3\n2 4\n3 4\n3\n1 2 2\n1 3 1\n2 4 3\n2 1\n2 1\n3\n1 1 1\n2 2 1\n1 2 2\n","output":"0\n3\n7\n0\n0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::misc::time_tracker::TimeTracker;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_utils::UpperDiv;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut tt = TimeTracker::new();
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();
    let q = input.read_size();
    let queries: Vec<(usize, usize, usize)> = input.read_vec(q).dec();

    let mut qq = vec![Vec::new(); n];
    for (i, (_, r, _)) in queries.copy_enumerate() {
        qq[r].push(i);
    }
    for i in 0..n {
        qq[i].sort_by_key(|&j| Reverse(queries[j].0));
    }
    let graph = Graph::with_biedges(n, &edges);
    const BUBEN: usize = 1500;
    let mut pos = vec![0];
    let mut cur = 0;
    for i in 0..n - 1 {
        cur += graph[i].len() + 1;
        if cur >= BUBEN {
            pos.push(i + 1);
            cur = 0;
        }
    }
    eprintln!("pos: {}", pos.len());
    let nn = (1 << n.highest_bit()) * 2;
    const SHIFT: usize = 10;
    let mut ex = vec![0; nn];
    let mut ex2 = vec![0; nn.upper_div(1 << SHIFT)];
    let mut val = vec![0; n];
    let mut ans = vec![0; q];
    tt.milestone("init");
    for i in pos {
        val.fill(0);
        for j in i..n {
            for e in &graph[j] {
                let to = e.to();
                if to >= i && to < j {
                    ex[val[to]] -= 1;
                    ex2[val[to] >> SHIFT] -= 1;
                    val[to] ^= j + 1;
                    ex[val[to]] += 1;
                    ex2[val[to] >> SHIFT] += 1;
                    val[j] ^= to + 1;
                }
            }
            ex[val[j]] += 1;
            ex2[val[j] >> SHIFT] += 1;
            while let Some(&kk) = qq[j].last() {
                let (l, r, k) = queries[kk];
                assert_eq!(r, j);
                if l > i {
                    break;
                }
                qq[j].pop();
                for x in (l..i).rev() {
                    for e in &graph[x] {
                        let to = e.to();
                        if to > x && to <= j {
                            ex[val[to]] -= 1;
                            ex2[val[to] >> SHIFT] -= 1;
                            val[to] ^= x + 1;
                            ex[val[to]] += 1;
                            ex2[val[to] >> SHIFT] += 1;
                            val[x] ^= to + 1;
                        }
                    }
                    ex[val[x]] += 1;
                    ex2[val[x] >> SHIFT] += 1;
                }
                let mut rem = (k - 1) as i32;
                for i in ex2.indices() {
                    if ex2[i] > rem {
                        for j in (i << SHIFT)..nn.min((i + 1) << SHIFT) {
                            if ex[j] <= rem {
                                rem -= ex[j];
                            } else {
                                ans[kk] = j;
                                break;
                            }
                        }
                        break;
                    } else {
                        rem -= ex2[i];
                    }
                }
                for x in l..i {
                    for e in &graph[x] {
                        let to = e.to();
                        if to > x && to <= j {
                            ex[val[to]] -= 1;
                            ex2[val[to] >> SHIFT] -= 1;
                            val[to] ^= x + 1;
                            ex[val[to]] += 1;
                            ex2[val[to] >> SHIFT] += 1;
                        }
                    }
                    ex[val[x]] -= 1;
                    ex2[val[x] >> SHIFT] -= 1;
                    val[x] = 0;
                }
            }
        }
        ex.fill(0);
        ex2.fill(0);
    }
    tt.milestone("long");
    val.fill(0);
    for i in 0..n {
        for kk in qq[i].copy_iter() {
            let (l, r, k) = queries[kk];
            for j in l..=r {
                for e in &graph[j] {
                    let to = e.to();
                    if to > j && to <= r {
                        val[to] ^= j + 1;
                        val[j] ^= to + 1;
                    }
                }
            }
            val[l..=r].sort();
            ans[kk] = val[l + k - 1];
            val[l..=r].fill(0);
        }
    }
    tt.milestone("short");
    out.print_per_line(&ans);
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
