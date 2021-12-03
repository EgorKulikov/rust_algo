use crate::io::input::Input;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub trait ParallelJob: Sync + Send + Default + Clone {
    fn read_input(&mut self, input: &mut Input);
    fn solve(&mut self);
    fn write_output(&mut self, test_case: usize);
}

pub fn parallel_run<J: ParallelJob>(input: &mut Input) {
    let t = input.read();
    let mut jobs = vec![J::default(); t];
    for job in jobs.iter_mut() {
        job.read_input(input);
    }
    jobs.par_iter_mut().enumerate().for_each(|(test, job)| {
        job.solve();
        eprintln!("Test {} done", test);
    });
    for (i, mut job) in jobs.into_iter().enumerate() {
        job.write_output(i + 1);
    }
}
