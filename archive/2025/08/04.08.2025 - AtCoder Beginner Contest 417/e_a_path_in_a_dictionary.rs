//{"name":"E - A Path in A Dictionary","group":"AtCoder - AtCoder Beginner Contest 417","url":"https://atcoder.jp/contests/abc417/tasks/abc417_e","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n6 10 3 5\n1 2\n1 3\n1 5\n1 6\n2 4\n2 5\n2 6\n3 4\n3 5\n5 6\n3 2 3 2\n1 3\n2 3\n","output":"3 1 2 5\n3 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let x = input.read_size() - 1;
    let y = input.read_size() - 1;
    let edges = input.read_size_pair_vec(m).dec();

    let mut dsu = DSU::new(n);
    let mut used = BitSet::new(n);
    let mut ans = vec![x];
    let mut last = x;
    used.set(x);
    while last != y {
        dsu.clear();
        let mut next = vec![];
        for (u, v) in edges.copy_iter() {
            if !used[u] && !used[v] {
                dsu.union(u, v);
            }
            if u == last {
                next.push(v);
            } else if v == last {
                next.push(u);
            }
        }
        next.sort();
        for v in next {
            if dsu.find(y) == dsu.find(v) {
                last = v;
                ans.push(v);
                used.set(v);
                break;
            }
        }
    }
    out.print_line(ans.inc());
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
