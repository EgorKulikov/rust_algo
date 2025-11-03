//{"name":"D. Locked Out","group":"Codeforces - Pinely Round 5 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2161/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1\n1\n5\n1 2 3 4 5\n5\n5 4 3 2 1\n5\n5 5 5 4 4\n7\n1 7 1 2 5 7 1\n6\n1 2 5 6 5 5\n","output":"0\n2\n0\n0\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::by_index;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let pos = by_index(&a);
    let mut b = a.clone();
    b.sort();
    b.dedup();
    let mut ans = 0;
    let mut cur = Vec::new();
    let mut last = n + 1;
    for x in b {
        if last + 1 != x {
            if !cur.is_empty() {
                ans += cur.copy_map(|(_, v)| v).min().unwrap();
            }
            cur.clear();
            for i in pos[x].indices() {
                cur.push((pos[x][i], i));
            }
            cur.push((n, pos[x].len()));
            last = x;
            continue;
        }
        let mut at = cur.len();
        let mut next = Vec::new();
        let mut best = n;
        for i in pos[x].indices().rev() {
            best -= 1;
            while at > 0 && cur[at - 1].0 > pos[x][i] {
                at -= 1;
                best.minim(cur[at].1 + pos[x].len() - 1);
            }
            next.push((pos[x][i], best));
        }
        let best = cur.copy_map(|(_, v)| v).min().unwrap();
        next.reverse();
        cur = next;
        cur.push((n, best + pos[x].len()));
        last = x;
    }
    ans += cur.copy_map(|(_, v)| v).min().unwrap();
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
