//Yandex - Stage 8: Grand Prix of Poland
//{"type":"stdin","fileName":null,"pattern":null}
//{"type":"stdout","fileName":null,"pattern":null}

use algo_lib::collections::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, Output, OUTPUT};
use algo_lib::misc::dirs::D4;
use algo_lib::{out, out_line};
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let k: u16 = input.read();
    let map = input.read_table::<char>(n, m);

    let mut left_up = Arr2d::new(n, m, 0u16);
    let mut right_up = Arr2d::new(n, m, 0u16);
    let mut left_down = Arr2d::new(n, m, 0u16);
    let mut right_down = Arr2d::new(n, m, 0u16);
    for i in 0..n {
        for j in 0..m {
            if map[(i, j)] == 'x' {
                let left = if i == 0 { k + 1 } else { left_up[(i - 1, j)] };
                let up = if j == 0 { k + 1 } else { left_up[(i, j - 1)] };
                left_up[(i, j)] = left.min(up) + 1;
            }
        }
    }
    for i in (0..n).rev() {
        for j in 0..m {
            if map[(i, j)] == 'x' {
                let left = if i == n - 1 {
                    k + 1
                } else {
                    left_down[(i + 1, j)]
                };
                let up = if j == 0 { k + 1 } else { left_down[(i, j - 1)] };
                left_down[(i, j)] = left.min(up) + 1;
            }
        }
    }
    for i in 0..n {
        for j in (0..m).rev() {
            if map[(i, j)] == 'x' {
                let left = if i == 0 { k + 1 } else { right_up[(i - 1, j)] };
                let up = if j == m - 1 {
                    k + 1
                } else {
                    right_up[(i, j + 1)]
                };
                right_up[(i, j)] = left.min(up) + 1;
            }
        }
    }

    for i in (0..n).rev() {
        for j in (0..m).rev() {
            if map[(i, j)] == 'x' {
                let left = if i == n - 1 {
                    k + 1
                } else {
                    right_down[(i + 1, j)]
                };
                let up = if j == m - 1 {
                    k + 1
                } else {
                    right_down[(i, j + 1)]
                };
                right_down[(i, j)] = left.min(up) + 1;
            }
        }
    }
    let mut bases = VecDeque::new();
    let mut remap = Arr2d::new(n, m, '.');
    for i in 0..n {
        for j in 0..m {
            if left_up[(i, j)] >= k + 1
                && right_up[(i, j)] >= k + 1
                && left_down[(i, j)] >= k + 1
                && right_down[(i, j)] >= k + 1
            {
                bases.push_back((i, j, k));
                remap[(i, j)] = 'x';
            }
        }
    }
    while let Some((r, c, k)) = bases.pop_front() {
        if k != 0 {
            for (nr, nc) in D4::iter(r, c, n, m) {
                if remap[(nr, nc)] != 'x' {
                    remap[(nr, nc)] = 'x';
                    bases.push_back((nr, nc, k - 1));
                }
            }
        }
    }
    if map == remap {
        out_line!("TAK");
    } else {
        out_line!("NIE");
    }
}

//START SKIP
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
    let time_limit = std::time::Duration::from_millis(4000);
    let mut paths = std::fs::read_dir("./tests/l_lemurs/")
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
fn l_lemurs() {
    assert!(run_tests());
}
