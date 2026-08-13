use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut n = input.read_size();
    let mut s = input.read_str();

    let mut pos = vec![0];
    for (i, (a, b)) in s.consecutive_iter_copy().enumerate() {
        if a != b {
            pos.push(i + 1);
        }
    }
    pos.push(n);
    let mut q_zeroes = 0;
    for i in 0..pos.len() - 1 {
        if s[pos[i]] == b'0' {
            q_zeroes += 1;
        }
    }
    if q_zeroes <= 1 {
        out.print_line(Str::from(vec![b'1'; n]));
        return;
    }
    if q_zeroes == 2 && s[n - 1] == b'0' {
        s[..pos[2]].fill(b'1');
        out.print_line(s);
        return;
    }

    while s[Back(0)] == b'0' {
        s.pop();
        n -= 1;
    }

    let mut left = 0;
    let mut right = n;
    while left < right {
        let mid = (left + right) / 2;

        let mut v = vec![None; 3];
        v[0] = Some(0);
        for i in 0..n {
            for j in 1..3 {
                if let Some(val) = v[j - 1] {
                    v[j].minim(val);
                }
            }
            let x = (s[i] - b'0') as usize;
            for j in (x..3).step_by(2) {
                if let Some(val) = v[j] {
                    if val == mid {
                        v[j] = None;
                    } else {
                        v[j] = Some(val + 1);
                    }
                }
            }
            for j in ((x ^ 1)..3).step_by(2) {
                if v[j].is_some() {
                    v[j] = Some(0);
                }
            }
        }
        if v.copy_any(|x| x.is_some()) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    let zeroes = left;
    let mut left = 1;
    let mut right = n;
    while left < right {
        let mid = (left + right + 1) / 2;
        let mut v = vec![None; 3];
        v[0] = Some((0, Reverse(0)));
        for i in 0..n {
            for j in 1..3 {
                if let Some(val) = v[j - 1] {
                    v[j].minim(val);
                }
            }
            let x = (s[i] - b'0') as usize;
            for j in (x..3).step_by(2) {
                if let Some((v1, Reverse(v2))) = v[j] {
                    if v2 > 0 {
                        if v2 < mid && v1 == zeroes {
                            v[j] = None;
                        } else {
                            v[j] = Some((1, Reverse(0)));
                        }
                    } else if v1 == zeroes {
                        v[j] = None;
                    } else {
                        v[j] = Some((v1 + 1, Reverse(0)));
                    }
                }
            }
            for j in ((x ^ 1)..3).step_by(2) {
                if let Some(x) = v[j].as_mut() {
                    x.1.0 += 1;
                }
            }
        }
        if v.copy_any(|x| {
            if let Some((v1, Reverse(v2))) = x {
                if v1 < zeroes || v2 >= mid {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    out.print(Str::from(vec![b'0'; zeroes]));
    out.print_line(Str::from(vec![b'1'; left]));

    /*let mut val = vec![(Reverse(0), 0); pos.len()];
    let mut set = BTreeSet::new();
    for i in 1..pos.len() - 1 {
        if s[pos[i - 1]] == b'0' {
            val[i - 1] = (Reverse(pos[i] - pos[i - 1]), pos[i + 1] - pos[i]);
            set.insert((val[i - 1], Reverse(i - 1)));
        }
    }
    let id = set.pop_first().unwrap().1.0;
    let mut left = id;
    let mut right = id;
    let mut ans = val[id];
    let mut left_set = BTreeSet::new();
    let mut right_set = BTreeSet::new();
    for i in (id % 2..pos.len() - 2).step_by(2) {
        left_set.insert((val[i], i));
    }
    for i in (id % 2..pos.len() - 2).step_by(2).rev() {
        left_set.remove(&(val[i], i));
        if i + 2 < pos.len() - 2 {
            let mut cur = (val[i].0, pos[i + 4] - pos[i + 1]);
            if let Some(&(left, _)) = left_set.first() {
                cur.minim(left);
            }
            if let Some(&(right, _)) = right_set.first() {
                cur.minim(right);
            }
            ans.maxim(cur);
            right_set.insert((val[i + 2], i + 2));
        }
    }
    let mut seen = (Reverse(0), 0);
    if id >= 2 {
        set.remove(&(val[id - 2], Reverse(id - 2)));
        val[id - 2].1 = pos[id + 2] - pos[id - 1];
        set.insert((val[id - 2], Reverse(id - 2)));
    }
    while let Some((_, Reverse(id))) = set.pop_first() {
        if left <= id && id <= right {
            continue;
        }
        ans.maxim(seen.min(val[id]));
        if id < left {
            for i in (id + 1..left).step_by(2) {
                let mut cur = (Reverse(pos[i + 1] - pos[i]), pos[i + 2] - pos[i + 1]);
                if i + 1 == right {
                    cur.1 = pos[i + 3] - pos[i + 1];
                }
                seen.minim(cur);
            }
            left = id;
        } else {
            if left < right {
                let i = right - 1;
                let cur = (Reverse(pos[i + 1] - pos[i]), pos[i + 2] - pos[i + 1]);
                seen.minim(cur);
            }
            for i in (right + 1..id).step_by(2) {
                let mut cur = (Reverse(pos[i + 1] - pos[i]), pos[i + 2] - pos[i + 1]);
                if i + 1 == id {
                    cur.1 = pos[i + 3] - pos[i + 1];
                }
                seen.minim(cur);
            }
            right = id;
        }
        if left >= 2 {
            let id = left;
            set.remove(&(val[id - 2], Reverse(id - 2)));
            val[id - 2].1 = pos[id + 2] - pos[id - 1];
            set.insert((val[id - 2], Reverse(id - 2)));
        }
    }
    ans.maxim(seen);
    out.print(Str::from(vec![b'0'; ans.0.0]));
    out.print_line(Str::from(vec![b'1'; ans.1]));*/
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
