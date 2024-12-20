//{"name":"D. Kevin and Competition Memories","group":"Codeforces - Codeforces Global Round 28","url":"https://codeforces.com/contest/2048/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 4\n4 3 7 5\n2 5 4 6\n5 5\n5 0 4 8 6\n1 3 9 2 7\n6 7\n1 1 4 5 1 4\n1 9 1 9 8 1 0\n7 6\n1 9 1 9 8 1 0\n1 1 4 5 1 4\n","output":"7 4 2 3\n6 2 1 1 2\n7 3 2 1 1 1 1\n15 9 5 4 4 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DKevinAndCompetitionMemories"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::treap::multi_treap_set::MultiTreapSet;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(m).sorted();

    let mut ans = Vec::with_capacity(m);
    let mut set = MultiTreapSet::new();
    for i in 1..n {
        set.insert(a[i]);
    }
    let max_a = a.copy_max();
    let solvable = b.copy_filter(|x| *x <= a[0] || *x > max_a).collect_vec();
    let not_solvable = b.copy_filter(|x| *x > a[0] && *x <= max_a).collect_vec();
    for i in 1..=m {
        let mut cur = solvable.len() / i;
        let rem = m / i - cur;
        let mut j = not_solvable.len() + solvable.len() % i;
        if rem > 0 {
            for _ in 0..rem {
                j -= i;
                cur += 1;
                cur += set.more_or_eq(&not_solvable[j]);
            }
            // j -= i - solvable.len() % i;
            // cur += 1;
            // cur += set.more_or_eq(&not_solvable[j]);
        }
        ans.push(cur);
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
