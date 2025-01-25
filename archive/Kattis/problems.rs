//{"name":"99 Problems","group":"Kattis","url":"https://open.kattis.com/problems/99problems2?tab=submissions","interactive":false,"timeLimit":1000,"tests":[{"input":"3 4\n10 10 11\n1 10\n1 10\n1 9\n1 5\n","output":"11\n-1\n10\n10\n"},{"input":"3 4\n10 10 11\n2 10\n2 10\n2 10\n2 15\n","output":"10\n10\n-1\n11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Problems"}}}

use algo_lib::collections::treap::multi_treap_set::MultiTreapSet;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let d = input.read_size_vec(n).sorted();

    let mut d_qty = Vec::new();
    let mut start = 0;
    for i in 1..n {
        if d[i] != d[i - 1] {
            d_qty.push((d[i - 1], i - start));
            start = i;
        }
    }
    d_qty.push((d[n - 1], n - start));
    let mut set = unsafe { MultiTreapSet::gen(d_qty.len(), |i| d_qty[i]) };

    for _ in 0..q {
        let t = input.read_size();
        let d = input.read_size();
        match t {
            1 => {
                let next = set.next(&d).copied();
                out.print_line(next);
                if let Some(next) = next {
                    set.remove(&next);
                }
            }
            2 => {
                let prev = set.floor(&d).copied();
                out.print_line(prev);
                if let Some(prev) = prev {
                    set.remove(&prev);
                }
            }
            _ => unreachable!(),
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
