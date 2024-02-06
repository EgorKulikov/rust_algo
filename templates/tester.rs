#![allow(unused_variables)]
#![allow(dead_code)]

use crate::run;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;
use std::io::{stdin, stdout, Write};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

const PRINT_LIMIT: usize = 1000;

fn interact(
    mut _sol_input: Input,
    mut sol_output: Output,
    mut _input: Input,
) -> Result<(), String> {
    let mut stdin = stdin();
    let mut input = Input::new(&mut stdin);
    while !input.is_exhausted() {
        let line = input.read_line();
        if line == "###".into() {
            break;
        }
        sol_output.print_line(line);
    }
    Ok(())
}

fn generate_test(out: &mut Output) {}

fn stupid(input: &mut Input, out: &mut Output) -> bool {
    false
}

pub fn check(input: &mut &[u8], expected: &mut &[u8], actual: &mut &[u8]) -> Result<(), String> {
    let mut _input = Input::new(input);
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

struct ReadDelegate {
    rcv: Receiver<Vec<u8>>,
    cur_buf: Vec<u8>,
    cur_at: usize,
}

impl ReadDelegate {
    fn new(rcv: Receiver<Vec<u8>>) -> Self {
        Self {
            rcv,
            cur_buf: Vec::new(),
            cur_at: 0,
        }
    }
}

impl std::io::Read for ReadDelegate {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.cur_at == self.cur_buf.len() {
            self.cur_buf = self.rcv.recv().unwrap();
            self.cur_at = 0;
        }
        let to_read = std::cmp::min(buf.len(), self.cur_buf.len() - self.cur_at);
        buf[..to_read].copy_from_slice(&self.cur_buf[self.cur_at..self.cur_at + to_read]);
        self.cur_at += to_read;
        Ok(to_read)
    }
}

struct WriteDelegate {
    snd: Sender<Vec<u8>>,
    prefix: &'static str,
    need_show: bool,
    remaining_show: usize,
}

impl WriteDelegate {
    fn new(snd: Sender<Vec<u8>>, prefix: &'static str) -> Self {
        Self {
            snd,
            prefix,
            need_show: true,
            remaining_show: PRINT_LIMIT,
        }
    }
}

impl Write for WriteDelegate {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.remaining_show != 0 {
            for c in buf {
                if self.need_show {
                    print!("{}", self.prefix);
                    self.need_show = false;
                }
                if *c == b'\n' {
                    self.need_show = true;
                }
                print!("{}", *c as char);
                self.remaining_show -= 1;
                if self.remaining_show == 0 {
                    println!("...");
                    break;
                }
            }
        }
        self.snd.send(buf.to_vec()).unwrap();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn do_interact(input: Input) -> Result<(), String> {
    let (snd1, rcv1) = std::sync::mpsc::channel();
    let (snd2, rcv2) = std::sync::mpsc::channel();

    let handle = thread::spawn(move || {
        let mut read_delegate = ReadDelegate::new(rcv2);
        let mut write_delegate = WriteDelegate::new(snd1, "> ");
        run(
            Input::new_with_size(&mut read_delegate, 1),
            Output::new_with_auto_flush(&mut write_delegate),
        );
    });

    let mut read_delegate = ReadDelegate::new(rcv1);
    let mut write_delegate = WriteDelegate::new(snd2, "< ");
    let result = interact(
        Input::new_with_size(&mut read_delegate, 1),
        Output::new_with_auto_flush(&mut write_delegate),
        input,
    );
    result
}

fn print_string_with_limit(s: &str) {
    if s.len() <= PRINT_LIMIT {
        println!("{}", s);
    } else {
        println!("{}...", s.split_at(PRINT_LIMIT).0);
    }
}

pub(crate) fn run_tests() -> bool {
    let blue = "\x1B[34m";
    let red = "\x1B[31m";
    let green = "\x1B[32m";
    let yellow = "\x1B[33m";
    let def = "\x1B[0m";
    let time_limit = std::time::Duration::from_millis($TIME_LIMIT);
    let mut paths = std::fs::read_dir("./$TASK/tests/")
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
                        let input = std::fs::read_to_string(&path).unwrap();
                        print_string_with_limit(&input);
                        if $INTERACTIVE {
                            println!("{}Interacting:{}", blue, def);
                            match do_interact(Input::new(&mut input.as_bytes())) {
                                Ok(_) => {
                                    println!("{}Verdict: {}OK{}", blue, green, def);
                                }
                                Err(err) => {
                                    println!("{}Error: {}{}{}", red, err, blue, def);
                                    test_failed += 1;
                                }
                            }
                            continue;
                        }
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
                                print_string_with_limit(&expected);
                            }
                        }
                        println!("{}Output:{}", blue, def);
                        match std::panic::catch_unwind(|| {
                            let mut file = std::fs::File::open(&path).unwrap();
                            let started = std::time::Instant::now();
                            let mut output = Vec::new();
                            let is_exhausted =
                                crate::run(Input::new(&mut file), Output::new(&mut output));
                            let res = started.elapsed();
                            let out_str = String::from_utf8_lossy(&output);
                            print_string_with_limit(out_str.as_ref());
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
                                    match check(
                                        &mut input.as_bytes(),
                                        &mut expected_bytes,
                                        &mut &output[..],
                                    ) {
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
                                match err.downcast::<&str>() {
                                    Ok(as_string) => println!(
                                        "{}Verdict: {}RuntimeError ({:?}){}",
                                        blue, red, as_string, def
                                    ),
                                    Err(err) => println!(
                                        "{}Verdict: {}RuntimeError ({:?}){}",
                                        blue, red, err, def
                                    ),
                                }
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

pub(crate) fn stress_test() {
    fn generate_test_int() -> Vec<u8> {
        let mut res = Vec::new();
        let mut output = Output::new(&mut res);
        generate_test(&mut output);
        output.flush();
        res
    }

    fn stupid_int(mut input: &[u8]) -> Option<Vec<u8>> {
        let mut input = Input::new(&mut input);
        let mut res = Vec::new();
        let mut output = Output::new(&mut res);
        if !stupid(&mut input, &mut output) {
            return None;
        }
        output.flush();
        Some(res)
    }

    fn actual(inp: &[u8]) -> Vec<u8> {
        match std::panic::catch_unwind(|| {
            let mut inp = inp;
            let input = Input::new(&mut inp);
            let mut res = Vec::new();
            let output = Output::new(&mut res);
            crate::run(input, output);
            res
        }) {
            Err(_) => {
                println!();
                println!(
                    "Crashed on test:\n{}",
                    String::from_utf8(inp.to_vec()).unwrap()
                );
                panic!();
            }
            Ok(res) => res,
        }
    }

    println!("");
    println!("\x1B[34mStress testing\x1B[0m");
    loop {
        let input = generate_test_int();
        let expected = stupid_int(&input);
        let actual = actual(&input);
        print!("#");
        stdout().flush().unwrap();
        if let Some(expected) = expected {
            let res = check(
                &mut input.as_slice(),
                &mut expected.as_slice(),
                &mut actual.as_slice(),
            );
            match res {
                Ok(_) => {
                    print!(".");
                    stdout().flush().unwrap();
                }
                Err(err) => {
                    println!();
                    println!("Failed on test:\n{}", String::from_utf8(input).unwrap());
                    println!("Expected:\n{}", String::from_utf8(expected).unwrap());
                    println!("Actual:\n{}", String::from_utf8(actual).unwrap());
                    println!("Error: {}", err);
                    panic!();
                }
            }
        } else {
            print!(".");
        }
    }
}
