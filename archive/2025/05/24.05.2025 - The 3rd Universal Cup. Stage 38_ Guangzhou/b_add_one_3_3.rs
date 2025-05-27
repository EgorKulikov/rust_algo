//{"name":"B. Add One 3 / 加一 3","group":"Universal Cup - The 3rd Universal Cup. Stage 38: Guangzhou","url":"https://contest.ucup.ac/contest/2036/problem/11106","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n6 6\n-1 2 -2 0 1 -1\n1 1\n1 2\n1 3\n1 4\n1 5\n1 6\n1 1\n0\n1 1\n","output":"-1\n2\n-1\n-1\n4\n5\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
// use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n);

    let ps = a.partial_sums();
    let mut min = 0;
    let mut is_unique_min = true;
    let mut is_unique = false;
    let mut target = 0;
    let mut is_left = BitSet::new(n + 1);
    is_left.set(0);
    // let mut last_pos = 0;
    // let mut pos = vec![0];
    let mut from = vec![0; n + 1];
    let mut cur_from = 0;
    let mut min_at = 0;
    let mut erase = 0;
    for i in 0..n {
        // if a[i] >= 0 {
        //     last_pos = i + 1;
        // }
        // pos.push(last_pos);
        if ps[i + 1] < min {
            min = ps[i + 1];
            is_left.set(i + 1);
            is_unique_min = true;
            min_at = i + 1;
        } else if ps[i + 1] == min {
            is_unique_min = false;
            min_at = i + 1;
        }
        let cur = ps[i + 1] - min;
        if target.maxim(cur) {
            is_unique = is_unique_min;
            cur_from = min_at;
            erase = i + 1;
        } else if target == cur {
            is_unique = false;
            cur_from = min_at;
        }
        from[i + 1] = cur_from;
    }
    from[..erase].fill(0);
    let mut is_right = BitSet::new(n + 1);
    is_right.set(n);
    min = 0;
    for i in (0..n).rev() {
        let cur = ps[n] - ps[i];
        if cur < min {
            min = cur;
            is_right.set(i);
        }
    }

    // let mut best_left = BitSet::new(n + 1);
    // let mut best_right = BitSet::new(n + 1);
    // for (l, r) in best.copy_iter() {
    //     best_left.set(l);
    //     best_right.set(r);
    // }
    // let best_set = best.copy_iter().collect::<FxHashSet<_>>();
    // let mut good = Vec::new();
    // let single = Vec::new();
    // for i in 1..n - 1 {
    //     if !best_right[i - 1] && !best_left[i + 1] && a[i] < 0 {
    //         good.push(i);
    //     } else if (!best_right[i - 1] || !best_left[i + 1]) && a[i] < 0 {
    //
    //     }
    // }

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();
        if !is_left[l] || !is_right[r] {
            out.print_line(-1);
            continue;
        }
        let ans = target - (ps[r] - ps[l]);
        if ans == 0 && is_unique {
            out.print_line(0);
        } else {
            if l + 1 == r {
                out.print_line(ans + 1);
            } else {
                let need = (1 - a[l]).max(0) + (1 - a[r - 1]).max(0);
                let target = if from[r - 1] > l {
                    target + 2
                } else {
                    target + 1
                };
                let target = target.max(2);
                out.print_line((target - (ps[r] - ps[l])).max(need));
            }
        }
    }
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
