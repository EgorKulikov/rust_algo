//{"name":"D: Dividing Passcodes","group":"Meta Coding Competitions - Meta Hacker Cup 2025 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2025/round-2/problems/D","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n1 3 2\n129 135 5\n98 3669 11\n12345678 87654321 20\n","output":"Case #1: 1\nCase #2: 4\nCase #3: 2025\nCase #4: 66132213\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"dividing_passcodes_.*input[.]txt"},"output":{"type":"file","fileName":"dividing_passcodes_output.txt","pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::TaskType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::{Str, StrReader};
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let l = input.read_str();
    let r = input.read_str();
    let k = input.read_size();
    drop(input);

    type Mod = ModIntF;
    #[derive(Eq, PartialEq, Hash, Clone, Copy)]
    enum State {
        Bad,
        Value(usize),
    }
    impl State {
        fn new() -> Self {
            State::Value(0)
        }

        fn advance(self, d: usize, k: usize) -> Self {
            match self {
                State::Bad => State::Bad,
                State::Value(v) => {
                    let nv = v.with_bit(0) << (d % k);
                    let nv = (nv & usize::all_bits(k)) | (nv >> k);
                    if nv.is_set(0) {
                        State::Bad
                    } else {
                        State::Value(nv)
                    }
                }
            }
        }

        fn join(self, other: Self, k: usize) -> bool {
            match (self, other) {
                (State::Bad, _) | (_, State::Bad) => true,
                (State::Value(v1), State::Value(v2)) => {
                    for i in 1..k {
                        if v1.is_set(i) && v2.is_set(k - i) {
                            return true;
                        }
                    }
                    false
                }
            }
        }
    }

    let mut qty = Vec::with_capacity(r.len() + 1);
    let mut map = DefaultHashMap::new(Mod::zero());
    map[State::new()] += Mod::one();
    qty.push(map);
    for _ in 0..r.len() {
        let last = &qty[Back(0)];
        let mut map = DefaultHashMap::new(Mod::zero());
        for (&state, &count) in last.iter() {
            for d in 0..10 {
                let new_state = state.advance(d, k);
                map[new_state] += count;
            }
        }
        qty.push(map);
    }
    let go = |max: Str, include: bool| -> Mod {
        let mut res = Mod::zero();
        for i in 0..max.len() - 1 {
            for (&state, &count) in qty[i].iter() {
                for d in 1..10 {
                    let new_state = state.advance(d, k);
                    if new_state == State::Bad {
                        res += count;
                    }
                }
            }
        }
        let mut state = State::new();
        for i in max.indices() {
            let cd = max[i] as usize - b'0' as usize;
            for d in if i == 0 { 1 } else { 0 }..cd {
                let new_state = state.advance(d, k);
                for (&state, &count) in qty[max.len() - 1 - i].iter() {
                    if new_state.join(state, k) {
                        res += count;
                    }
                }
            }
            state = state.advance(cd, k);
        }
        if include && state == State::Bad {
            res += Mod::one();
        }
        res
    };
    let ans = go(r, true) - go(l, false);

    out.print_line((format!("Case #{}:", test_case), ans));
}

pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let pre_calc = ();
    let is_exhausted = run_parallel(input, &mut output, true, pre_calc, solve);
    eprint!("\x1B[0m");
    output.flush();
    is_exhausted
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let paths = std::fs::read_dir(".").unwrap();
    let mut result = None;
    let mut last_accessed = None;
    let re = regex::Regex::new("dividing_passcodes_.*input[.]txt").unwrap();
    for path in paths {
        let path = path.unwrap();
        let cur_accessed = path.metadata().unwrap().accessed().unwrap();
        let path = path.path();
        let cur_name = path.file_name().unwrap().to_str().unwrap();
        if re.is_match(cur_name) {
            if last_accessed.is_none() || cur_accessed > last_accessed.unwrap() {
                result = Some(cur_name.to_string());
                last_accessed = Some(cur_accessed);
            }
        }
    }
    let in_file = std::fs::File::open(result.unwrap()).unwrap();
    let input = algo_lib::io::input::Input::file(in_file);
    let out_file = std::fs::File::create("dividing_passcodes_output.txt").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);
    run(input, output);
}
