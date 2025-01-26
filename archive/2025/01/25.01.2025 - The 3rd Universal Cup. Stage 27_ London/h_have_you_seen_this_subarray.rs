//{"name":"H. Have You Seen This Subarray?","group":"Universal Cup - The 3rd Universal Cup. Stage 27: London","url":"https://contest.ucup.ac/contest/1901/problem/8618","interactive":false,"timeLimit":1000,"tests":[{"input":"6 3 5\n1 5\n3 4\n1 6\n2 4 1\n3 3 1 5\n3 3 4 5\n4 5 2 4 3\n2 6 2\n","output":"1\n3\n0\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let sw = input.read_size_pair_vec(m).dec();
    let qs = input.read_vec::<Vec<usize>>(q);

    let mut ans = vec![0; q];
    let mut ps = DefaultHashMap::new(FxHashSet::default());
    for i in 0..q {
        let b = &qs[i];
        let k = b.len();
        if k == 1 {
            ans[i] = 0;
        }
        for (x, y) in b.consecutive_iter_copy() {
            ps[(x, y)].insert(i);
        }
    }
    let mut c = vec![0; q];
    let mut done = 0;
    let mut count = |a: usize, b: usize, d: i32, done: usize| {
        let p = (a, b);
        let pl = ps[p].clone();
        for i in pl {
            c[i] += d;
            let bi = &qs[i];
            let k = bi.len();
            if c[i] as usize == k - 1 {
                ans[i] = done;
                for j in 0..k - 1 {
                    let p2 = (bi[j], bi[j + 1]);
                    ps[p2].remove(&i);
                }
            }
        }
    };
    let mut a = (1..=n).collect::<Vec<_>>();
    for i in 1..n {
        count(a[i - 1], a[i], 1, done);
    }
    let mut update = |i: usize, d: i32, skip_left: bool, a: &[usize], done: usize| {
        if i > 0 && !skip_left {
            count(a[i - 1], a[i], d, done);
        }
        if i < n - 1 {
            count(a[i], a[i + 1], d, done);
        }
    };
    for (i, j) in sw {
        done += 1;
        update(i, -1, false, &a, done);
        update(j, -1, j == i + 1, &a, done);
        a.swap(i, j);
        update(i, 1, false, &a, done);
        update(j, 1, j == i + 1, &a, done);
    }
    out.print_per_line(&ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
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
//END MAIN
