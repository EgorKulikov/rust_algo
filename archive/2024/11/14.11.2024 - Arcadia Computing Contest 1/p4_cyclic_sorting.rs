//{"name":"P4 - Cyclic Sorting","group":"DMOJ - Arcadia Computing Contest 1","url":"https://dmoj.ca/problem/ahscc1p4","interactive":false,"timeLimit":3000,"tests":[{"input":"5 3\n1 2 4 3 5\n3 3\n1 6\n3 1\n","output":"0\n1\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P4CyclicSorting"}}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut a = input.read_size_vec(n);

    let mut bad = FxHashSet::default();
    for i in 0..n {
        if a[i] > a[(i + 1) % n] {
            bad.insert(i);
        }
    }
    for _ in 0..q {
        let pos = input.read_size() - 1;
        let x = input.read_size();

        bad.remove(&pos);
        bad.remove(&((pos + n - 1) % n));
        a[pos] = x;
        for i in [pos, (pos + n - 1) % n] {
            if a[i] > a[(i + 1) % n] {
                bad.insert(i);
            }
        }
        if bad.is_empty() {
            out.print_line(0);
        } else if bad.len() == 1 {
            let pos = *bad.iter().next().unwrap();
            out.print_line((pos + 1).min(n - 1 - pos));
        } else {
            out.print_line(-1);
        }
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
