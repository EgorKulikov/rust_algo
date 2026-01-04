use crate::print::print_interacting;
use crate::{process_error, Outcome, Tester};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;
use std::io::{Read, Write};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

pub struct SolutionRunner {
    handle: Option<JoinHandle<()>>,
    print_limit: usize,
    solution: fn(Input, Output) -> bool,
}

impl SolutionRunner {
    pub fn run(&mut self) -> (Input, Output<'static>) {
        self.join();
        let (snd1, rcv1) = std::sync::mpsc::channel();
        let (snd2, rcv2) = std::sync::mpsc::channel();
        let pl = self.print_limit;
        let sol = self.solution;
        self.handle = Some(thread::spawn(move || {
            let read_delegate = ReadDelegate::new(rcv2);
            let write_delegate = WriteDelegate::new(snd1, "> ", pl);
            sol(
                Input::delegate(read_delegate),
                Output::delegate(write_delegate),
            );
        }));
        let read_delegate = ReadDelegate::new(rcv1);
        let write_delegate = WriteDelegate::new(snd2, "< ", self.print_limit);
        (
            Input::delegate(read_delegate),
            Output::delegate(write_delegate),
        )
    }

    pub fn join(&mut self) {
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }

    pub fn is_finished(&self) -> bool {
        self.handle.as_ref().is_none_or(|h| h.is_finished())
    }
}

impl Drop for SolutionRunner {
    fn drop(&mut self) {
        self.join();
    }
}

pub(crate) fn run_single_test_interactive(
    tester: &Tester,
    interactor: fn(Input, Option<Input>, SolutionRunner) -> Result<(), String>,
    input: &[u8],
    expected: Option<&[u8]>,
    print_details: bool,
) -> Outcome {
    print_interacting(print_details);

    let start = Instant::now();
    let solution = tester.solution;
    let print_limit = if print_details { tester.print_limit } else { 0 };
    match std::panic::catch_unwind(|| {
        let runner = SolutionRunner {
            handle: None,
            print_limit,
            solution,
        };
        interactor(Input::slice(input), expected.map(Input::slice), runner)
    }) {
        Ok(res) => match res {
            Ok(()) => {
                let duration = start.elapsed();
                if duration.as_millis() as u64 > tester.time_limit {
                    Outcome::TimeLimit {
                        duration,
                        input_exhausted: true,
                    }
                } else {
                    Outcome::OK {
                        duration,
                        input_exhausted: true,
                    }
                }
            }
            Err(err) => Outcome::WrongAnswer {
                checker_output: err,
                input_exhausted: false,
            },
        },
        Err(err) => process_error(err),
    }
}

pub fn std_interactor(
    _sol_input: Input,
    _expected: Option<Input>,
    mut runner: SolutionRunner,
) -> Result<(), String> {
    let (mut _sol, mut out) = runner.run();
    let mut input = Input::stdin();
    while !input.is_exhausted() {
        let line = input.read_line();
        if runner.is_finished() {
            break;
        }
        if line.as_slice() == b"@@@" {
            (_sol, out) = runner.run();
        }
        out.print_line(line);
        out.flush();
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

impl Read for ReadDelegate {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.cur_at == self.cur_buf.len() {
            self.cur_buf = match self.rcv.recv() {
                Ok(v) => v,
                Err(_) => return Ok(0),
            };
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
    fn new(snd: Sender<Vec<u8>>, prefix: &'static str, print_limit: usize) -> Self {
        Self {
            snd,
            prefix,
            need_show: true,
            remaining_show: print_limit,
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
