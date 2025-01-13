//{"name":"E. Elevator II","group":"Universal Cup - The 3rd Universal Cup. Stage 25: Hangzhou","url":"https://contest.ucup.ac/contest/1893/problem/9730","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4 2\n3 6\n1 3\n2 7\n5 6\n2 5\n2 4\n6 8\n","output":"11\n2 3 1 4\n5\n2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EElevatorII"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::order::Order;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let f = input.read_long();
    let people = input.read_long_pair_vec(n);

    let mut ans = Vec::new();
    let mut cost: i64 = people.copy_map(|(a, b)| b - a).sum();
    let order = people.order();
    let mut used = BitSet::new(n);
    let mut cur = f;
    for i in order {
        let (a, b) = people[i];
        if a > cur {
            cost += a - cur;
        }
        if b > cur {
            used.set(i);
            cur = b;
            ans.push(i);
        }
    }
    let mut other = Vec::new();
    for i in 0..n {
        if !used[i] {
            other.push(i);
        }
    }
    other.sort_unstable_by_key(|&i| -people[i].1);
    ans.extend_from_slice(&other);
    out.print_line(cost);
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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
