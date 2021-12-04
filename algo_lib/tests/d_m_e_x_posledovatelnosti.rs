//Codeforces - Educational Codeforces Round 118 (рейтинговый для Div. 2)
//{"type":"stdin","fileName":null}
//{"type":"stdout","fileName":null}

use algo_lib::io::input::Input;
use algo_lib::io::output::{output, Output, OUTPUT};
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::from_u8::FromU8;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::{out, out_line};

fn solve_impl(n: usize, a: Vec<usize>) -> ModIntF {
    type Mod = ModIntF;
    let mut exact = vec![Mod::zero(); n + 3];
    let mut seq = vec![Mod::zero(); n + 3];
    let mut ans = Mod::zero();
    for i in a {
        ans += exact[i];
        exact[i] *= Mod::from_u8(2);
        ans += exact[i + 2];
        exact[i + 2] *= Mod::from_u8(2);
        let mut cseq = seq[i];
        if i > 0 {
            cseq += seq[i - 1];
        } else {
            cseq += Mod::one();
        }
        ans += cseq;
        seq[i] += cseq;
        if i > 1 {
            ans += seq[i - 2];
            exact[i] += seq[i - 2];
        } else if i == 1 {
            ans += Mod::one();
            exact[i] += Mod::one();
        }
    }
    ans
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = input.read_vec::<usize>(n);

    out_line!(solve_impl(n, a));
}

//START SKIP
#[test]
fn stress() {
    let mex = |a: &[usize]| -> usize {
        let set = a.iter().cloned().collect::<std::collections::HashSet<_>>();
        let mut i = 0usize;
        loop {
            if !set.contains(&i) {
                break i;
            }
            i += 1;
        }
    };
    let mut do_stress = RecursiveFunction2::new(|f, n, a: Vec<usize>| {
        if a.len() == n {
            let mut res = ModIntF::zero();
            for i in 1u32..(1 << n) {
                let mut b = Vec::new();
                for j in 0..n {
                    if ((i >> j) & 1) == 1 {
                        b.push(a[j]);
                    }
                }
                let mut good = true;
                for j in 1..=b.len() {
                    if ((b[j - 1] as isize) - (mex(&b[0..j]) as isize)).abs() > 1 {
                        good = false;
                        break;
                    }
                }
                if good {
                    res += ModIntF::one();
                }
            }
            let act = solve_impl(n, a.clone());
            if res != act {
                panic!("");
            }
        } else {
            for i in 0..=n {
                let mut b = a.clone();
                b.push(i);
                f.call(n, b);
            }
        }
    });
    for n in 1..=5 {
        do_stress.call(n, Vec::new());
    }
}

//END SKIP

fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
                    "Token #{} differs, expectes {}, actual {}",
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
    let time_limit = std::time::Duration::from_millis(2000);
    let mut paths = std::fs::read_dir("./tests/d_m_e_x_posledovatelnosti/")
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
fn d_m_e_x_posledovatelnosti() {
    assert!(run_tests());
}
