use crate::io::input::Input;
use crate::io::output::Output;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use rayon::ThreadPoolBuilder;
use std::sync::atomic::AtomicUsize;

pub trait ParallelJob: Sync + Send + Default + Clone {
    fn read_input(&mut self, input: &mut Input);
    fn solve(&mut self);
    fn write_output(&mut self, output: &mut Output, test_case: usize);
}

pub fn run_parallel<J: ParallelJob>(input: &mut Input, output: &mut Output, do_parallel: bool) {
    let t = input.read();
    let mut jobs = vec![J::default(); t];
    for job in jobs.iter_mut() {
        job.read_input(input);
    }
    ThreadPoolBuilder::new()
        .stack_size(1000000000)
        .build_global()
        .unwrap();
    let rem = AtomicUsize::new(t);
    let do_job = |(test, job): (usize, &mut J)| {
        job.solve();
        eprintln!(
            "Test {} done, {} remaining",
            test,
            rem.fetch_sub(1, std::sync::atomic::Ordering::Relaxed) - 1
        );
    };
    if do_parallel {
        jobs.par_iter_mut().enumerate().for_each(do_job);
    } else {
        jobs.iter_mut().enumerate().for_each(do_job);
    }
    for (i, mut job) in jobs.into_iter().enumerate() {
        job.write_output(output, i + 1);
    }
}
