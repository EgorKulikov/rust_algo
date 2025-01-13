//{"name":"M. Make It Divisible","group":"Universal Cup - The 3rd Universal Cup. Stage 25: Hangzhou","url":"https://contest.ucup.ac/contest/1893/problem/9738","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n5 10\n7 79 1 7 1\n2 1000000000\n1 2\n1 100\n1000000000\n","output":"3 8\n0 0\n100 5050\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MMakeItDivisible"}}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::order::Order;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::primes::factorize::Factorize;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_u64();
    let b = input.read_u64_vec(n);

    let order = b.order();
    let mut present = BTreeSet::new();
    let mut pairs = Vec::new();
    for i in order {
        if let Some(&j) = present.prev(&i) {
            if b[i] != b[j] {
                pairs.push((b[i] - b[j], b[j]));
            }
        }
        if let Some(&j) = present.next(&i) {
            if b[i] != b[j] {
                pairs.push((b[i] - b[j], b[j]));
            }
        }
        present.insert(i);
    }

    if pairs.is_empty() {
        out.print_line((k, k * (k + 1) / 2));
        return;
    }
    let d = pairs[0].0.divisors();
    let mut qty = 0;
    let mut sum = 0;
    for d in d {
        if d <= pairs[0].1 {
            continue;
        }
        let cand = d - pairs[0].1;
        if cand > k {
            continue;
        }
        let mut ok = true;
        for (diff, start) in pairs.copy_iter() {
            if diff % (start + cand) != 0 {
                ok = false;
                break;
            }
        }
        if ok {
            qty += 1;
            sum += cand;
        }
    }
    out.print_line((qty, sum));
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
