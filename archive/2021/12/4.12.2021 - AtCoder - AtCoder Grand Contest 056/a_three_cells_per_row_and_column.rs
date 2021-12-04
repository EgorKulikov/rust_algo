//AtCoder - AtCoder Grand Contest 056
//{"type":"stdin","fileName":null,"pattern":null}
//{"type":"stdout","fileName":null,"pattern":null}

use algo_lib::collections::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, Output, OUTPUT};
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut n: usize = input.read();

    let mut table = Arr2d::new(n, n, '.');

    while n >= 12 {
        for i in (n - 6..n).step_by(2) {
            for j in 0usize..3 {
                table[(i, n - 6 + j)] = '#';
                table[(i + 1, n - 3 + j)] = '#';
            }
        }
        n -= 6;
    }
    let mut cols = vec![0usize; n];
    let mut dsu = DSU::new(n * n);
    let mut f = RecursiveFunction::new(|f, row| {
        if row == n {
            dsu.clear();
            for i in 0..n {
                for j in 0..n {
                    if table[(i, j)] != '#' {
                        continue;
                    }
                    if i + 1 < n && table[(i + 1, j)] == '#' {
                        dsu.join(i * n + j, (i + 1) * n + j);
                    }
                    if j + 1 < n && table[(i, j + 1)] == '#' {
                        dsu.join(i * n + j, i * n + j + 1);
                    }
                }
            }
            let res = dsu.count() == n * n - 2 * n;
            res
        } else {
            for i in 0..n {
                if cols[i] == 3 {
                    continue;
                }
                for j in 0..i {
                    if cols[j] == 3 {
                        continue;
                    }
                    for k in 0..j {
                        if cols[k] == 3 {
                            continue;
                        }
                        table[(row, i)] = '#';
                        table[(row, j)] = '#';
                        table[(row, k)] = '#';
                        cols[i] += 1;
                        cols[j] += 1;
                        cols[k] += 1;
                        if f.call(row + 1) {
                            return true;
                        }
                        table[(row, i)] = '.';
                        table[(row, j)] = '.';
                        table[(row, k)] = '.';
                        cols[i] -= 1;
                        cols[j] -= 1;
                        cols[k] -= 1;
                    }
                }
            }
            false
        }
    });
    assert!(f.call(0));
    output().print_table(&table);
}

#[test]
fn ways() {
    for n in 6usize..12 {
        let mut table = Arr2d::new(n, n, '.');
        let mut cols = vec![0usize; n];
        let mut dsu = DSU::new(n * n);
        let mut f = RecursiveFunction::new(|f, row| {
            if row == n {
                dsu.clear();
                for i in 0..n {
                    for j in 0..n {
                        if table[(i, j)] != '#' {
                            continue;
                        }
                        if i + 1 < n && table[(i + 1, j)] == '#' {
                            dsu.join(i * n + j, (i + 1) * n + j);
                        }
                        if j + 1 < n && table[(i, j + 1)] == '#' {
                            dsu.join(i + n + j, i * n + j + 1);
                        }
                    }
                }
                let res = dsu.count() == n * n - 2 * n;
                if res {
                    let mut output = Output::new(Box::new(std::io::stderr()));
                    output.print_table(&table);
                }
                res
            } else {
                for i in 0..n {
                    if cols[i] == 3 {
                        continue;
                    }
                    for j in 0..i {
                        if cols[j] == 3 {
                            continue;
                        }
                        for k in 0..j {
                            if cols[k] == 3 {
                                continue;
                            }
                            table[(row, i)] = '#';
                            table[(row, j)] = '#';
                            table[(row, k)] = '#';
                            cols[i] += 1;
                            cols[j] += 1;
                            cols[k] += 1;
                            if f.call(row + 1) {
                                return true;
                            }
                            table[(row, i)] = '.';
                            table[(row, j)] = '.';
                            table[(row, k)] = '.';
                            cols[i] -= 1;
                            cols[j] -= 1;
                            cols[k] -= 1;
                        }
                    }
                }
                false
            }
        });
        assert!(f.call(0));
    }
}

//START SKIP
//BEGIN MAIN
fn main() {
    run_tests();
}
//END MAIN
//END SKIP

fn run(mut input: Input) -> bool {
    loop {
        input.skip_whitespace();
        if input.peek().is_none() {
            break;
        }
        solve(&mut input, 1);
        output().flush();
        // input.skip_whitespace();
        // !input.peek().is_some()
    }
    true
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
                    expected_token.unwrap(),
                    actual_token.unwrap()
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
    let mut paths = std::fs::read_dir("./a_three_cells_per_row_and_column/tests/")
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
