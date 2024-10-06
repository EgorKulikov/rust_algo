use crate::io::input::Input;
use crate::io::output::Output;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, TryLockError};
use std::thread::{scope, yield_now};

pub fn run_parallel<P>(
    mut input: Input,
    output: &mut Output,
    do_parallel: bool,
    pre_calc: P,
    run: impl Fn(MutexGuard<Input>, &mut Output, usize, &P) + Send + Sync + 'static + Copy,
) -> bool
where
    for<'a> &'a P: Send,
{
    let t = input.read_size();
    let tests_remaining = AtomicUsize::new(t);
    let input = Arc::new(Mutex::new(input));
    scope(|s| {
        let mut handles = Vec::with_capacity(t);
        for i in 1..=t {
            eprintln!("Test {} started", i);
            let tr = &tests_remaining;
            let inp = input.clone();
            let pre_calc = &pre_calc;
            let handle = s.spawn(move || {
                let mut res = Vec::new();
                let mut output = Output::new(&mut res);
                run(inp.lock().unwrap(), &mut output, i, pre_calc);
                eprintln!(
                    "Test {} done, {} tests remaining",
                    i,
                    tr.fetch_sub(1, Ordering::Relaxed) - 1
                );
                output.flush();
                res
            });
            if do_parallel {
                while let Err(err) = input.try_lock() {
                    match err {
                        TryLockError::Poisoned(poison) => {
                            panic!("Poisoned lock: {:?}", poison);
                        }
                        TryLockError::WouldBlock => {
                            yield_now();
                        }
                    }
                }
                handles.push(handle);
            } else {
                let res = handle.join().unwrap();
                output.write_all(&res).unwrap();
            }
        }
        for handle in handles {
            let res = handle.join().unwrap();
            output.write_all(&res).unwrap();
        }
    });
    let res = input.lock().unwrap().is_empty();
    res
}
