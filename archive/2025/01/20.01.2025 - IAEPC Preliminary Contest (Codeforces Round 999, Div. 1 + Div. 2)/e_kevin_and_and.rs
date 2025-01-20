//{"name":"E. Kevin and And","group":"Codeforces - IAEPC Preliminary Contest (Codeforces Round 999, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2061/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 3 2\n7\n5 6 3\n2 3 2\n5 6\n5 6 3\n10 2 5\n3 1 4 1 5 9 2 6 5 3\n7 8\n5 1 0\n1073741823 1073741823 1073741823 1073741823 1073741823\n1073741823\n1 1 0\n0\n0\n","output":"1\n3\n11\n5368709115\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BinaryHeap;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(m);

    let b_and = Vec::with_gen(1 << m, |i| {
        let mut res = i64::all_bits(30);
        for j in 0..m {
            if i.is_set(j) {
                res &= b[j];
            }
        }
        res
    });
    let mut res = Arr2d::new(n, m + 1, i64::all_bits(30));
    for i in 0..n {
        for j in b_and.indices() {
            res[(i, j.count_ones() as usize)].minim(b_and[j] & a[i]);
        }
    }
    let mut level = vec![0; n];
    let mut ans = a.copy_sum();
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((res[(i, 0)] - res[(i, 1)], i));
    }
    for _ in 0..k {
        let (val, id) = heap.pop().unwrap();
        ans -= val;
        level[id] += 1;
        if level[id] != m {
            heap.push((res[(id, level[id])] - res[(id, level[id] + 1)], id));
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
