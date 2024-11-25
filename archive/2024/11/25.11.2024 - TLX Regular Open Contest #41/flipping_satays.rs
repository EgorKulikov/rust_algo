//{"name":"Flipping Satays","group":"TLX - TLX Regular Open Contest #41","url":"https://tlx.toki.id/contests/troc-41/problems/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3 6 3 2 11\n1 1 2\n3 2 3\n5 3 3\n7 2 3\n8 2 2\n9 3 3\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FlippingSatays"}}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BTreeSet;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let k = input.read_size();
    let b = input.read_size();
    let f = input.read_size();
    let flips = input.read_vec::<(usize, usize, usize)>(q);

    let mut adds = vec![Vec::new(); n];
    let mut removes = vec![Vec::new(); n];
    for (t, l, r) in flips {
        adds[l - 1].push(t);
        if r < n {
            removes[r].push(t);
        }
    }
    let mut set = BTreeSet::new();
    set.insert(0);
    set.insert(f);
    let mut value = f / k;
    let mut ans = 0;
    for i in 0..n {
        for t in adds[i].copy_iter() {
            let prev = set.prev(&t).copied().unwrap();
            let next = set.next(&t).copied().unwrap();
            value -= (next - prev) / k;
            value += (t - prev) / k;
            value += (next - t) / k;
            set.insert(t);
        }
        for t in removes[i].copy_iter() {
            set.remove(&t);
            let prev = set.prev(&t).copied().unwrap();
            let next = set.next(&t).copied().unwrap();
            value += (next - prev) / k;
            value -= (t - prev) / k;
            value -= (next - t) / k;
        }
        ans += value.min(b);
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
