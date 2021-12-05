//Yandex - Stage 8: Grand Prix of Poland
//{"type":"stdin","fileName":null,"pattern":null}
//{"type":"stdout","fileName":null,"pattern":null}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, Output, OUTPUT};
use algo_lib::misc::recursive_function::{Callable4, RecursiveFunction4};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let t = input.read();

    for _ in 0usize..t {
        let n = input.read();
        let mut k: usize = input.read();
        let mut a = input.read_vec::<usize>(n);

        /*let mut qty = HashMap::new();
        for i in a.iter().cloned() {
            let now = 1u32 + qty.get(&i).unwrap_or(&0);
            qty.insert(i, now);
        }
        let mut res = HashMap::new();
        for (i, v) in qty.iter() {
            let i = *i;
            let v = *v;
            let other = i ^ k;
            let add = match qty.get(&other) {
                None => v,
                Some(w) => {
                    if i < other {
                        v.max(*w)
                    } else {
                        0
                    }
                }
            };
            let key = i & (!k);
            res.insert(key, *res.get(&key).unwrap_or(&0) + add);
        }*/
        a.sort();
        k += 1;
        if k == usize::bit(20) {
            out_line!(n);
            return;
        }
        let mut f = RecursiveFunction4::new(
            |f,
             mut bit: usize,
             (f1, t1): (usize, usize),
             (f2, t2): (usize, usize),
             split: bool|
             -> usize {
                if bit == 0 {
                    return (t1 - f1).max(t2 - f2);
                }
                if f1 == t1 && f2 == t2 {
                    return 0;
                }
                bit -= 1;
                let ff = |f: usize, t: usize| {
                    if f == t || a[f].is_set(bit) {
                        ((f, f), (f, t))
                    } else if !a[t - 1].is_set(bit) {
                        ((f, t), (t, t))
                    } else {
                        let mid = ((a[f] >> bit) + 1) << bit;
                        let mid = a.as_slice().lower_bound(&mid);
                        debug_assert!(f < mid && mid < t);
                        ((f, mid), (mid, t))
                    }
                };
                let left = ff(f1, t1);
                let right = ff(f2, t2);
                if k.is_set(bit) {
                    let mut ans = 0;
                    let v = left.0 .1 - left.0 .0 + right.0 .1 - right.0 .0;
                    ans.maxim(v);
                    ans.maxim(left.1 .1 - left.1 .0 + right.1 .1 - right.1 .0);
                    if split {
                        ans.maxim(
                            f.call(bit, left.0, right.1, true) + f.call(bit, left.1, right.0, true),
                        );
                    } else {
                        ans.maxim(f.call(bit, left.0, left.1, true));
                    }
                    ans
                } else {
                    let mut ans = 0;
                    if split {
                        ans.maxim(t1 - f1);
                        ans.maxim(t2 - f2);
                    }
                    ans.maxim(
                        f.call(bit, left.0, right.0, split)
                            .max(f.call(bit, left.1, right.1, split)),
                    );
                    ans
                }
            },
        );
        out_line!(
            f.call(20, (0, n), (n, n), false) /*.max(*res.values().max().unwrap() as usize)*/
        );
    }
}

//START SKIP
//END SKIP

fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START SKIP
fn check(expected: &mut &[u8], actual: &mut &[u8]) -> Result<(), String> {
    let mut expected = Input::new(expected);
    let mut actual = Input::new(actual);
    let mut token_num = 0usize;
    loop {
        let expected_token = expected.next_token();
        let actual_token = actual.next_token();
        if expected_token != actual_token {
            if expected_token.is_none() {
                return Err(format!("Expected has only {} tokens", token_num));
            } else if actual_token.is_none() {
                return Err(format!("Actual has only {} tokens", token_num));
            } else {
                return Err(format!(
                    "Token #{} differs, expected {}, actual {}",
                    token_num,
                    String::from_utf8(expected_token.unwrap()).unwrap(),
                    String::from_utf8(actual_token.unwrap()).unwrap()
                ));
            }
        }
        token_num += 1;
        if actual_token.is_none() {
            break;
        }
    }
    Ok(())
}

static mut OUT: Vec<u8> = Vec::new();

struct WriteDelegate {}

impl std::io::Write for WriteDelegate {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unsafe {
            OUT.append(&mut Vec::from(buf));
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn run_tests() -> bool {
    let blue = "\x1B[34m";
    let red = "\x1B[31m";
    let green = "\x1B[32m";
    let yellow = "\x1B[33m";
    let def = "\x1B[0m";
    let time_limit = std::time::Duration::from_millis(3000);
    let mut paths = std::fs::read_dir("./tests/i_interesting_numbers/")
        .unwrap()
        .map(|res| res.unwrap())
        .collect::<Vec<_>>();
    paths.sort_by(|a, b| a.path().cmp(&b.path()));
    let mut test_failed = 0usize;
    let mut test_total = 0usize;
    for path in paths {
        let sub_path = path;
        if sub_path.file_type().unwrap().is_file() {
            let path = sub_path.path();
            match path.extension() {
                None => {}
                Some(extension) => {
                    if extension.to_str() == Some("in") {
                        println!("=====================================================");
                        test_total += 1;
                        let name = path.file_name().unwrap().to_str().unwrap();
                        let name = &name[..name.len() - 3];
                        println!("{}Test {}{}", blue, name, def);
                        println!("{}Input:{}", blue, def);
                        println!("{}", std::fs::read_to_string(&path).unwrap());
                        let expected = match std::fs::read_to_string(
                            path.parent().unwrap().join(format!("{}.out", name)),
                        ) {
                            Ok(res) => Some(res),
                            Err(_) => None,
                        };
                        println!("{}Expected:{}", blue, def);
                        match &expected {
                            None => {
                                println!("{}Not provided{}", yellow, def);
                            }
                            Some(expected) => {
                                println!("{}", expected);
                            }
                        }
                        println!("{}Output:{}", blue, def);
                        match std::panic::catch_unwind(|| {
                            unsafe {
                                OUT.clear();
                            }
                            let mut file = std::fs::File::open(&path).unwrap();
                            let input = Input::new(&mut file);
                            let started = std::time::Instant::now();
                            unsafe {
                                OUTPUT = Some(Output::new(Box::new(WriteDelegate {})));
                            }
                            let is_exhausted = run(input);
                            let res = started.elapsed();
                            let output;
                            unsafe {
                                output = OUT.clone();
                            }
                            println!("{}", String::from_utf8_lossy(&output));
                            (output, res, is_exhausted)
                        }) {
                            Ok((output, duration, is_exhausted)) => {
                                println!(
                                    "{}Time elapsed: {:.3}s{}",
                                    blue,
                                    (duration.as_millis() as f64) / 1000.,
                                    def,
                                );
                                if !is_exhausted {
                                    println!("{}Input not exhausted{}", red, def);
                                }
                                if let Some(expected) = expected {
                                    let mut expected_bytes = expected.as_bytes().clone();
                                    match check(&mut expected_bytes, &mut &output[..]) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            println!(
                                                "{}Verdict: {}Wrong Answer ({}){}",
                                                blue, red, err, def
                                            );
                                            test_failed += 1;
                                            continue;
                                        }
                                    }
                                }
                                if duration > time_limit {
                                    test_failed += 1;
                                    println!("{}Verdict: {}Time Limit{}", blue, red, def);
                                } else {
                                    println!("{}Verdict: {}OK{}", blue, green, def)
                                }
                            }
                            Err(err) => {
                                test_failed += 1;
                                println!(
                                    "{}Verdict: {}RuntimeError ({:#?}){}",
                                    blue, red, err, def
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    if test_failed == 0 {
        println!(
            "{}All {}{}{} tests passed{}",
            blue, green, test_total, blue, def
        );
    } else {
        println!(
            "{}{}/{}{} tests failed{}",
            red, test_failed, test_total, blue, def
        );
    }
    test_failed == 0
}
//END SKIP
#[test]
fn i_interesting_numbers() {
    assert!(run_tests());
}
