//{"name":"World","group":"Eolymp - Basecamp - Weekend Practice #22","url":"https://eolymp.com/en/compete/rast2dle4l13n0vnc70f076fao/problem/4","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n6 10 15 14 21\n2\n1 5\n2 4\n","output":"1 3 5\n-1\n"},{"input":"8\n4 9 6 5 10 15 7 11\n5\n1 8\n1 5\n3 6\n1 4\n4 6\n","output":"4 5 6\n1 3 5\n4 5 6\n-1\n4 5 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::primes::sieve::divisor_table;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);
    let q = input.read_size();
    let lr = input.read_size_pair_vec(q).dec();

    let d = divisor_table(200_001);
    let mut pos = vec![Vec::new(); 200_000];
    let mut primes = vec![Vec::new(); n];
    let mut map = DefaultHashMap::new(Vec::new());
    for i in 0..n {
        let mut cur = a[i];
        while cur > 1 {
            let p = d[cur];
            primes[i].push(p);
            pos[p].push(i);
            while cur % p == 0 {
                cur /= p;
            }
        }
        for a in primes[i].indices() {
            for b in 0..a {
                map[(primes[i][a], primes[i][b])].push(i);
                map[(primes[i][b], primes[i][a])].push(i);
            }
        }
    }

    let mut base = Vec::new();
    for i in 0..200_000 {
        if pos[i].len() > 2 {
            for j in 2..pos[i].len() {
                base.push((pos[i][j - 2], pos[i][j - 1], pos[i][j]));
            }
        }
    }
    let mut check = |l: usize, r: usize| {
        for i in primes[l].indices() {
            for j in primes[r].indices() {
                if primes[l][i] != primes[r][j] {
                    continue;
                }
                for a in primes[l].indices() {
                    if a == i {
                        continue;
                    }
                    for b in primes[r].indices() {
                        if b == j {
                            continue;
                        }
                        let key = (primes[l][a], primes[r][b]);
                        let pos = map[key].upper_bound(&l);
                        if pos < map[key].len() && map[key][pos] < r {
                            base.push((l, map[key][pos], r));
                            break;
                        }
                    }
                }
            }
        }
    };
    if n <= 1000 {
        for i in 0..n {
            for j in i + 1..n {
                check(i, j);
            }
        }
    }
    for (l, r) in lr.copy_iter() {
        check(l, r);
    }

    base.sort_by_key(|&(a, _, b)| (a, b));
    let mut ans = Vec::new();
    'outer: for (i, j, k) in base {
        while let Some(&(i0, _, k0)) = ans.last() {
            if i0 == i && k0 <= k {
                continue 'outer;
            }
            if k0 >= k {
                ans.pop();
            } else {
                break;
            }
        }
        ans.push((i, j, k));
    }
    for (l, r) in lr {
        let pos = ans.lower_bound(&(l, l, l));
        if pos < ans.len() && ans[pos].2 <= r {
            out.print_line((ans[pos].0 + 1, ans[pos].1 + 1, ans[pos].2 + 1));
        } else {
            out.print_line(-1);
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
