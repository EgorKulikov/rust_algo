use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let k = input.read_size();
    let q = input.read_size();

    'outer:
    for _ in 0..q {
        let l = input.read_size();
        let r = input.read_size() + 1;
        let n = input.read_size();
        let t = input.read_str();

        if n > r - l {
            out.print_line(0);
            continue;
        }

        let a = Vec::with_gen(n, |i| (t[i] - b'0') as usize);
        let mut step = 1;
        let mut at = 1;
        let mut expected = 1;
        let mut varians = Vec::new();
        let mut k_k = 1usize;
        for _ in 0..k {
            k_k = k_k.saturating_mul(k);
        }
        if n == 1 {
            let mut x = 1;
            while x < r {
                varians.push((0, a[0], x));
                x = x.saturating_mul(k);
            }
        } else {
            for _ in 0.. {
                let mut special = Vec::new();
                let mut qty = 0;
                assert!(at < n);
                for i in (at..n).step_by(step) {
                    qty += 1;
                    if (a[i] + k - a[i - 1]) % k != expected {
                        special.push(i);
                    }
                }
                for i in 1..special.len() {
                    if (special[i] - special[0]) % (k * step) != 0 {
                        out.print_line(0);
                        continue 'outer;
                    }
                }
                if special.is_empty() {
                    if qty >= 2 * k {
                        out.print_line(0);
                        continue 'outer;
                    }
                    if qty >= k {
                        for i in qty - k..k {
                            let pos = at + i * step;
                            let mut x = step.saturating_mul(k_k);
                            while x < r {
                                varians.push((pos, a[pos], x));
                                x = x.saturating_mul(k_k);
                            }
                        }
                    } else {
                        for i in 0..qty {
                            let pos = at + i * step;
                            let mut x = step.saturating_mul(k_k);
                            while x < r {
                                varians.push((pos, a[pos], x));
                                x = x.saturating_mul(k_k);
                            }
                        }
                        if qty == 1 {
                            varians.push((at, a[at], step));
                        } else {
                            let mut val = (a[at + (qty - 1) * step] + 1) % k;
                            for i in qty..k {
                                val = (val + 1) % k;
                                let pos = at + i * step;
                                let mut x = step.saturating_mul(k);
                                let mut val = val;
                                while x < r + pos {
                                    if pos >= n || a[pos] == val {
                                        varians.push((pos, val, x));
                                    }
                                    x = x.saturating_mul(k);
                                    val = (val + 1) % k;
                                }
                            }
                        }
                    }
                    break;
                }
                at = special[0];
                step = step.saturating_mul(k);
                at %= step;
                if at == 0 {
                    at = step;
                }
                expected = (expected + 1) % k;
            }
        }
        let mut ans = 0;
        for (pos, val, step) in varians {
            let mut from = l + pos;
            let to = from + r - l - n + 1;
            from = from.upper_div(step) * step;
            while from < to {
                if (from / step) % k == 0 && (from / step / k) % k == 0 {
                    let big_step = step.saturating_mul(k).saturating_mul(k);
                    let end = to / big_step * big_step;
                    ans += (end - from) / big_step * (k - 1);
                    from = end.saturating_add(step);
                    continue;
                }
                if (from / step) % k == 0 {
                    from = from.saturating_add(step);
                    continue;
                }
                let mut cur = 0;
                let mut x = from;
                while x > 0 {
                    cur += x % k;
                    x /= k;
                }
                cur %= k;
                if cur == val {
                    ans += 1;
                }
                from = from.saturating_add(step);
            }
        }
        out.print_line(ans);
    }
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
