use crate::io::input::Input;
use crate::io::output::Output;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{available_parallelism, scope, yield_now, Builder};

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
    let free_slots = AtomicUsize::new(available_parallelism().unwrap().get() - 1);
    let input_locked = AtomicBool::new(false);
    scope(|s| {
        let mut handles = Vec::with_capacity(t);
        for i in 1..=t {
            eprintln!("Test {} started", i);
            let tr = &tests_remaining;
            let inp = input.clone();
            let pre_calc = &pre_calc;
            free_slots.fetch_sub(1, Ordering::Relaxed);
            input_locked.store(true, Ordering::Relaxed);
            let il = &input_locked;
            let fs = &free_slots;
            let builder = Builder::new().stack_size(1 << 30);
            let handle = builder.spawn_scoped(s, move || {
                let lock = inp.lock().unwrap();
                il.store(false, Ordering::Relaxed);
                let mut res = Vec::new();
                let mut output = Output::buf(&mut res);
                run(lock, &mut output, i, pre_calc);
                eprintln!(
                    "Test {} done, {} tests remaining",
                    i,
                    tr.fetch_sub(1, Ordering::Relaxed) - 1
                );
                output.flush();
                fs.fetch_add(1, Ordering::Relaxed);
                drop(output);
                res
            });
            if do_parallel {
                while input_locked.load(Ordering::Relaxed) {
                    yield_now();
                }
                input.lock().unwrap();
                while free_slots.load(Ordering::Relaxed) == 0 {
                    yield_now();
                }
                handles.push(handle.unwrap());
            } else {
                let res = handle.unwrap().join().unwrap();
                output.write_all(&res).unwrap();
            }
        }
        for handle in handles {
            let res = handle.join().unwrap();
            output.write_all(&res).unwrap();
        }
    });
    input.lock().unwrap().check_empty()
}
