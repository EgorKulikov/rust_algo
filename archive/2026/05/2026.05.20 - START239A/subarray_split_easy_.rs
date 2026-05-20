//{"name":"Subarray Split (Easy)","group":"CodeChef - START239A","url":"https://www.codechef.com/START239A/problems/SUBADDLXM0","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n3 5\n1 2 3\n4 3\n0 0 0 0\n5 9\n2 8 4 1 2\n7 6\n3 2 3 2 2 0 3\n8 9\n1 4 6 3 8 2 2 7\n","output":"1 2 3\n0 0 0 0\n2 0 5 2 3\n0 0 1 0 2 1 5\n1 0 0 0 0 0 4 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::multi_set::MultiTreeSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n);

    let mut ans = None;
    for p in 1..=n {
        let mut available = MultiTreeSet::new();
        for i in 0..p {
            available.insert(i % k);
        }
        let mut extra = MultiTreeSet::new();
        let mut cur = Vec::with_capacity(n);
        let mut lst = None;
        for i in 0..n {
            if available.is_empty() {
                let best = (a[i] + lst.unwrap()) % k;
                if !extra.is_empty() {
                    let cand = extra
                        .ceil(&(k - a[i]))
                        .map(|x| *x.0)
                        .unwrap_or(*extra.first().unwrap());
                    let other = (a[i] + cand) % k;
                    if other < best {
                        extra.remove(&cand);
                        lst = Some(cand);
                        cur.push(other);
                        continue;
                    }
                }
                cur.push(best);
                continue;
            }
            let target = k - a[i];
            let best = available
                .ceil(&target)
                .map(|x| *x.0)
                .unwrap_or(*available.first().unwrap());
            let cand = (a[i] + best) % k;
            let mut insert_extra = false;
            if n - i > available.len() {
                if let Some(last) = lst {
                    let other = (a[i] + last) % k;
                    if other < cand {
                        if !extra.is_empty() {
                            let other2 = extra
                                .ceil(&(k - a[i]))
                                .map(|x| *x.0)
                                .unwrap_or(*extra.first().unwrap());
                            let val = (a[i] + other2) % k;
                            if val < other {
                                extra.remove(&other2);
                                lst = Some(other2);
                                cur.push(val);
                                continue;
                            }
                        }
                        cur.push(other);
                        continue;
                    }
                    if other == cand {
                        insert_extra = true;
                    }
                }
            }
            if !extra.is_empty() && n - i > available.len() {
                let other = extra
                    .ceil(&(k - a[i]))
                    .map(|x| *x.0)
                    .unwrap_or(*extra.first().unwrap());
                let val = (a[i] + other) % k;
                if val < cand {
                    extra.remove(&other);
                    lst = Some(other);
                    cur.push(val);
                    continue;
                }
            }
            if insert_extra {
                extra.insert(best);
            }
            cur.push(cand);
            available.remove(&best);
            lst = Some(best);
        }
        ans.minim(cur);
    }
    out.print_line(ans);
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
