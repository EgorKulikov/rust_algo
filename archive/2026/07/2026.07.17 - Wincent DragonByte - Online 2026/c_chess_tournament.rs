use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::debug;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::{Random, RandomTrait};
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::slicelike::Slicelike;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let d = input.read_size();
    drop(input);

    fn assert_points(t: &Arr2d<u8>) {
        let mut last = i32::MAX;
        for i in t.rows() {
            let mut cur = 0;
            for j in t.cols() {
                if t[(i, j)] == b'W' {
                    cur += 1;
                } else if t[(i, j)] == b'L' {
                    cur -= 1;
                }
            }
            if last <= cur {
                debug!(&t);
            }
            assert!(last > cur);
            last = cur;
        }
    }
    if n == 1 {
        out.print_line(true);
        out.print_line("X");
        return;
    }
    let decisive = n * (n - 1) / 2 - d;
    let half = n / 2;
    if decisive < half * (half + 1) / 2 {
        out.print_line(false);
        return;
    }
    out.print_line(true);
    let mut ans = Arr2d::new(n, n, b'D');
    for i in 0..n {
        ans[(i, i)] = b'X';
    }
    let mut len = half;
    while (len + 1) * (len + 2) / 2 <= decisive {
        len += 1;
    }
    let mut excess = decisive;
    // let shift = (excess / half).min(half);
    // excess += half * (half + 1) / 2;
    for i in 0..len {
        let from = n - len + i;
        let to = n;
        for j in from..to {
            ans[(i, j)] = b'W';
            ans[(j, i)] = b'L';
            excess -= 1;
        }
    }
    assert_points(&ans);
    let mut r = Random::new_with_seed(239);
    while excess > 0 {
        let pts = Vec::with_gen(n, |i| {
            let mut cur = 0;
            for j in 0..n {
                if ans[(i, j)] == b'W' {
                    cur += 1;
                } else if ans[(i, j)] == b'L' {
                    cur -= 1;
                }
            }
            cur
        });
        let mut vars = Vec::new();
        for i in 0..n {
            for j in i + 1..n {
                if (i == 0 || pts[i - 1] > pts[i] + 1)
                    && (j == n - 1 || pts[j + 1] < pts[j] - 1)
                    && ans[(i, j)] == b'D'
                {
                    vars.push((i, j));
                }
            }
        }
        if vars.is_empty() && excess >= 2 {
            let mut vars = Vec::new();
            for i in 0..n {
                for j in i + 1..n {
                    for k in j + 1..n {
                        if (i == 0 || pts[i - 1] > pts[i] + 1)
                            && (k == n - 1 || pts[k + 1] < pts[k] - 1)
                            && ans[(i, j)] == b'D'
                            && ans[(j, k)] == b'D'
                        {
                            vars.push((i, j, k));
                        }
                    }
                }
            }
            assert!(!vars.is_empty());
            excess -= 2;
            let (i, j, k) = vars[r.gen_range(0..vars.len())];
            ans[(i, j)] = b'W';
            ans[(j, i)] = b'L';
            ans[(j, k)] = b'W';
            ans[(k, j)] = b'L';
            continue;
        }
        if vars.is_empty() {
            let mut vars = Vec::new();
            for i in 0..n {
                for j in i + 1..n {
                    for k in j + 1..n {
                        if ans[(i, j)] == b'D' && ans[(j, k)] == b'D' && ans[(i, k)] == b'W' {
                            vars.push((i, j, k));
                        }
                    }
                }
            }
            assert!(!vars.is_empty());
            excess -= 1;
            let (i, j, k) = vars[r.gen_range(0..vars.len())];
            ans[(i, j)] = b'W';
            ans[(j, i)] = b'L';
            ans[(j, k)] = b'W';
            ans[(k, j)] = b'L';
            ans[(i, k)] = b'D';
            ans[(k, i)] = b'D';
            continue;
        }
        let (i, j) = vars[r.gen_range(0..vars.len())];
        ans[(i, j)] = b'W';
        ans[(j, i)] = b'L';
        excess -= 1;
    }
    /*    for i in 0..shift {
        for j in 0..=i {
            if excess == 0 {
                break;
            }
            let f = j;
            let t = n - i - 1 + j;
            assert_eq!(ans[(f, t)], b'D');
            ans[(f, t)] = b'W';
            ans[(t, f)] = b'L';
            excess -= 1;
            assert_points(&ans);
        }
    }*/
    /*if excess % 2 == 1 {
        ans[(0, n - 1)] = b'D';
        ans[(n - 1, 0)] = b'D';
        excess += 1;
        // assert_points(&ans);
    }
    let x = n - len - 1;
    let mut lim = half;
    for i in 0..x {
        let mut at = 0;
        while excess > 0 {
            if ans[(at, half)] == b'D' {
                lim.minim(at);
                ans[(at, half)] = b'W';
                ans[(half, at)] = b'L';
                excess -= 1;
                ans[(half, n - 1 - at)] = b'W';
                ans[(n - 1 - at, half)] = b'L';
                excess -= 1;
                at += 1;
                assert_points(&ans);
            } else {
                if at >= lim {
                    break;
                }
                let next = n - len + at - 1;
                if next >= half {
                    break;
                }
                ans[(at, next)] = b'W';
                ans[(next, at)] = b'L';
                ans[(n - 1 - next, n - 1 - at)] = b'W';
                ans[(n - 1 - at, n - 1 - next)] = b'L';
                excess -= 2;
                assert_points(&ans);
                at = next;
            }
        }
        len += 1;
    }*/
    assert_eq!(excess, 0);
    out.print_table(&ans);
}

pub static TASK_TYPE: TaskType = TaskType::Classic;
pub static TEST_TYPE: TestType = TestType::MultiNumber;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let pre_calc = ();
    let is_exhausted = run_parallel(input, &mut output, false, pre_calc, solve);
    eprint!("\x1B[0m");
    output.flush();
    is_exhausted
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
