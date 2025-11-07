//{"name":"C. Dungeon","group":"Codeforces - Codeforces Global Round 30 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2164/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 2\n2 2 2\n2 3\n3 2\n2 3\n2 3\n2 3 4\n0 0 0\n3 5\n1 7 7\n6 6 2 2 2\n2 0 0 7 2\n4 4\n1 5 3 5\n7 4 6 5\n0 0 1 6\n2 2\n1 1000000000\n1000000000 1\n1000000000 0\n","output":"2\n2\n5\n3\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(m);
    let c = input.read_int_vec(m);

    let mut swords = BTreeSet::new();
    for i in 0..n {
        swords.insert((a[i], i));
    }
    let order = (0..m).collect::<Vec<_>>().sorted_by_key(|&i| b[i]);
    let mut ans = 0;
    for i in order.copy_iter() {
        if c[i] > 0 {
            if let Some(&(power, id)) = swords.ceil(&(b[i], 0)) {
                swords.remove(&(power, id));
                swords.insert((power.max(c[i]), id));
                ans += 1;
            }
        }
    }
    for i in order {
        if c[i] == 0 {
            if let Some(&(power, id)) = swords.ceil(&(b[i], 0)) {
                swords.remove(&(power, id));
                ans += 1;
            }
        }
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
