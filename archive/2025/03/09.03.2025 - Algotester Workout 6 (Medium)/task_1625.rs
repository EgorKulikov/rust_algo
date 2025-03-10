//{"name":"Краса масиву","group":"Algotester","url":"https://algotester.com/en/ContestProblem/DisplayWithFile/136187","interactive":false,"timeLimit":6000,"tests":[{"input":"5\n1 2 2 3 2\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut cur = a[0];
    let mut qty = 0i64;
    let mut compressed = Vec::new();
    for i in 0..n {
        if a[i] != cur {
            compressed.push((cur, qty));
            qty = 0;
            cur = a[i];
        }
        qty += 1;
    }
    compressed.push((cur, qty));

    let base = compressed.copy_fold(0, |acc, (_, y)| acc + y * (y + 1) / 2);
    let mut add = 0;
    for _ in 0..2 {
        let mut best = DefaultHashMap::new(0);
        for (id, q) in compressed.copy_iter() {
            add.maxim(best[id] * q);
            best[id].maxim(q);
        }
        compressed.reverse();
    }
    let mut values = DefaultHashMap::new(Vec::new());
    for ((id1, q1), (id2, q2)) in compressed.consecutive_iter_copy() {
        values[(id1, id2)].push((q1, q2));
    }
    for mut vals in values.into_values() {
        vals.sort();
        let mut n_vals = Vec::new();
        for (a, b) in vals {
            while let Some(&(c, d)) = n_vals.last() {
                add.maxim(c * a + b * d);
                if d <= b {
                    n_vals.pop();
                } else {
                    break;
                }
            }
            n_vals.push((a, b));
        }
        for i in 0..n_vals.len() {
            let (a, b) = n_vals[i];
            for j in 0..i {
                let (c, d) = n_vals[j];
                add.maxim(a * c + b * d);
            }
        }
    }
    out.print_line(base + add);
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
