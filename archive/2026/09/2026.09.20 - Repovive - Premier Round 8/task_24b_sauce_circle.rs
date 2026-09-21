use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let ab = input.read_size_pair_vec(n).dec();

    let mut graph = vec![DefaultHashMap::new(Vec::new()); m];
    for (i, (a, b)) in ab.copy_enumerate() {
        graph[a][b].push(i);
        graph[b][a].push(i);
    }
    let mut singles = vec![FxHashSet::default(); m];
    let mut degrees = graph.iter().map(|x| x.len()).collect::<Vec<_>>();
    for i in 0..m {
        if graph[i].len() == 1 && singles[i].is_empty() {
            let to = *graph[i].iter().next().unwrap().0;
            singles[to].insert(i);
            degrees[to] -= 1;
            degrees[i] = 0;
        }
    }
    if degrees.copy_max() > 2 {
        out.print_line(-1);
        return;
    }
    let mut ans = Vec::new();
    let mut processed = BitSet::new(m);
    for i in 0..m {
        if degrees[i] == 1 && !processed[i] {
            let mut cur = i;
            let mut prev = m;
            loop {
                processed.set(cur);
                let mut next = m;
                for &to in singles[cur].iter() {
                    for v in graph[cur][to].copy_iter() {
                        ans.push(v);
                    }
                }
                for (&to, ids) in graph[cur].iter() {
                    if !singles[cur].contains(&to) && to != prev {
                        next = to;
                        for v in ids.copy_iter() {
                            ans.push(v);
                        }
                    }
                }
                if next == m {
                    break;
                }
                prev = cur;
                cur = next;
            }
        }
        if degrees[i] == 0 && !singles[i].is_empty() {
            for &to in singles[i].iter() {
                for v in graph[i][to].copy_iter() {
                    ans.push(v);
                }
            }
        }
    }
    if ans.len() > 0 && ans.len() < n {
        out.print_line(-1);
        return;
    }
    if ans.is_empty() {
        for i in 0..m {
            if degrees[i] == 2 {
                let mut cur = i;
                let mut prev = m;
                loop {
                    processed.set(cur);
                    let mut next = m;
                    for &to in singles[cur].iter() {
                        for v in graph[cur][to].copy_iter() {
                            ans.push(v);
                        }
                    }
                    for (&to, ids) in graph[cur].iter() {
                        if !singles[cur].contains(&to) && to != prev {
                            next = to;
                            for v in ids.copy_iter() {
                                ans.push(v);
                            }
                            break;
                        }
                    }
                    if next == i {
                        break;
                    }
                    prev = cur;
                    cur = next;
                }
                break;
            }
        }
    }
    if ans.len() < n {
        out.print_line(-1);
        return;
    }
    out.print_line(ans.inc());
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
