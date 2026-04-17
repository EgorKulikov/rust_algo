//{"name":"Swapping Operations","group":"CodeChef - START234A","url":"https://www.codechef.com/START234A/problems/SWAPOPS","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3\n1?1\n5\n1?0?1\n7\n1?0101?\n9\n?????????\n8\n11010101\n6\n??011?\n","output":"0 1\n1 1\n1 2\n0 3\n2 2\n0 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::sliding_window::SlidingWindow;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let ones = s.copy_count(b'1');
    let qs = s.copy_count(b'?');
    let mut left = 0;
    let mut right = n;
    /*#[derive(Clone)]
    struct Node {
        val: usize,
        delta: usize,
    }
    impl Default for Node {
        fn default() -> Self {
            Self {
                val: usize::MAX,
                delta: 0,
            }
        }
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val.min(right_val.val);
        }
        fn accumulate(&mut self, value: &Self) {
            self.val -= value.delta;
            self.delta += value.delta;
        }
        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }
    let mut st = SegmentTree::<Node>::new(n);*/
    let mut v = vec![0; n];
    while left < right {
        let mid = (left + right + 1) / 2;
        let mut good = false;
        'outer: for len in ones.max(mid)..=ones + qs {
            let mut cur = 0;
            for i in 0..len {
                if s[i] == b'1' {
                    cur += 1;
                }
            }
            if cur > len - mid {
                continue;
            }
            // st.point_update(
            //     0,
            //     Node {
            //         val: len - mid - cur,
            //         delta: 0,
            //     },
            // );
            v[0] = len - mid - cur;
            for i in len..n {
                if s[i] == b'1' {
                    cur += 1;
                }
                if s[i - len] == b'1' {
                    cur -= 1;
                }
                if cur > len - mid {
                    continue 'outer;
                }
                v[i - len + 1] = len - mid - cur;
                // st.point_update(
                //     i - len + 1,
                //     Node {
                //         val: len - mid - cur,
                //         delta: 0,
                //     },
                // );
            }
            let mut sw = SlidingWindow::new(len, usize::min);
            let mut found = 0;
            for i in 0..n {
                if i <= n - len {
                    sw.push(v[i] + found);
                } else {
                    sw.push(usize::MAX);
                }
                if s[i] == b'?' {
                    if sw.get() > found {
                        found += 1;
                    }
                    // if st.query(i.saturating_sub(len - 1)..=i.min(n - len)).val > 0 {
                    //     found += 1;
                    //     st.update(
                    //         i.saturating_sub(len - 1)..=i.min(n - len),
                    //         &Node { val: 0, delta: 1 },
                    //     );
                    // }
                }
            }
            if found >= len - ones {
                good = true;
                break;
            }
        }
        if good {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    let mut min = None;
    for i in 0..n {
        let mut mid_ones = 0;
        let mut mid_qs = 0;
        for j in i..n {
            if s[j] == b'1' {
                mid_ones += 1;
            }
            if s[j] == b'?' {
                mid_qs += 1;
            }
            if j - i + 1 >= ones {
                let cur = (ones - mid_ones).max(j - i + 1 - mid_ones - mid_qs);
                min.minim(cur);
            }
        }
    }
    if ones == 0 {
        min.minim(0);
    }
    out.print_line((min, left));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
