use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use std::collections::VecDeque;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m);
    drop(input);

    let mut g = vec![Vec::new(); n];
    for (u, v) in edges {
        g[u].push(v);
        g[v].push(u);
    }
    if g[0].is_empty() {
        out.print_line(0);
        return;
    }
    for i in 0..n {
        g[i].sort();
    }
    let mut d = vec![n; n];
    let mut p = vec![n; n];
    let mut last = 0;
    let mut visited = BitSet::new(n);
    let mut queue = VecDeque::new();
    queue.push_back(0);
    visited.set(0);
    while let Some(v) = queue.pop_front() {
        last = v;
        for i in g[v].copy_iter() {
            if !visited[i] {
                visited.set(i);
                queue.push_back(i);
                d[i] = d[v] + 1;
                p[i] = v;
            }
        }
    }

    let mut path = vec![last];
    while path[Back(0)] != 0 {
        path.push(p[path[Back(0)]]);
    }
    type Mod = ModInt7;
    let mut ans = Mod::zero();

    enum Vertex {
        Special(usize),
        Compound(Vec<Mod>),
    }
    fn add(queue: &mut VecDeque<Vertex>, v: Vec<Mod>) {
        if queue.is_empty() || !matches!(queue[queue.len() - 1], Vertex::Compound(_)) {
            queue.push_back(Vertex::Compound(v));
        } else {
            let at = queue.len() - 1;
            match &mut queue[at] {
                Vertex::Compound(vv) => {
                    for i in v.indices() {
                        vv[i] += v[i];
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    let mut queue = VecDeque::new();
    queue.push_back(Vertex::Special(0));
    while let Some(vert) = queue.pop_front() {
        match vert {
            Vertex::Special(v) => {
                path.pop();
                ans += 1;
                if path.len() == 1 {
                    break;
                }
                for j in g[v].copy_iter() {
                    if j == path[Back(0)] {
                        queue.push_back(Vertex::Special(j));
                    } else {
                        let mut v = vec![Mod::zero(); n];
                        v[j] = Mod::one();
                        add(&mut queue, v);
                    }
                }
            }
            Vertex::Compound(v) => {
                let mut vv = vec![Mod::zero(); n];
                for i in 0..n {
                    ans += v[i];
                    for j in g[i].copy_iter() {
                        vv[j] += v[i];
                    }
                }
                add(&mut queue, vv);
            }
        }
    }
    out.print_line(ans);
}

pub static TASK_TYPE: TaskType = TaskType::Classic;
pub static TEST_TYPE: TestType = TestType::MultiNumber;

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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
