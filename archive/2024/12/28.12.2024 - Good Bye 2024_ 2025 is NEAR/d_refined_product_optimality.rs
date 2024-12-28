//{"name":"D. Refined Product Optimality","group":"Codeforces - Good Bye 2024: 2025 is NEAR","url":"https://codeforces.com/contest/2053/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n3 4\n1 1 2\n3 2 1\n1 3\n2 3\n1 1\n2 1\n6 8\n1 4 2 7 3 5\n7 6 5 6 3 3\n2 5\n1 6\n1 5\n1 5\n1 5\n2 3\n2 3\n1 6\n13 8\n7 7 6 6 5 5 5 2 2 3 4 5 1\n1 4 1 9 6 6 9 1 5 1 3 8 4\n2 2\n2 11\n2 4\n2 4\n1 7\n1 1\n2 12\n1 5\n5 3\n10000000 20000000 30000000 40000000 50000000\n10000000 20000000 30000000 40000000 50000000\n1 1\n2 2\n2 1\n","output":"2 3 3 6 6\n840 840 1008 1344 1680 2016 2016 2016 2352\n2116800 2646000 3528000 3528000 3528000 4233600 4838400 4838400 4838400\n205272023 205272023 205272023 264129429\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRefinedProductOptimality"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use std::ops::Mul;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut a = input.read_unsigned_vec(n);
    let mut b = input.read_unsigned_vec(n);

    let mut a_sorted = a.clone().sorted();
    let mut b_sorted = b.clone().sorted();
    type Mod = ModIntF;
    let mut ans = a_sorted
        .copy_zip(&b_sorted)
        .map(|(a, b)| Mod::new(a.min(b)))
        .reduce(Mod::mul)
        .unwrap();
    let mut res = Vec::with_capacity(q + 1);
    res.push(ans);
    for _ in 0..q {
        let o = input.read_int();
        let x = input.read_size() - 1;

        if o == 1 {
            let pos = a_sorted.upper_bound(&a[x]) - 1;
            ans /= Mod::new(a_sorted[pos].min(b_sorted[pos]));
            a_sorted[pos] += 1;
            ans *= Mod::new(a_sorted[pos].min(b_sorted[pos]));
            a[x] += 1;
        } else {
            let pos = b_sorted.upper_bound(&b[x]) - 1;
            ans /= Mod::new(a_sorted[pos].min(b_sorted[pos]));
            b_sorted[pos] += 1;
            ans *= Mod::new(a_sorted[pos].min(b_sorted[pos]));
            b[x] += 1;
        }
        res.push(ans);
    }
    out.print_line(res);
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
