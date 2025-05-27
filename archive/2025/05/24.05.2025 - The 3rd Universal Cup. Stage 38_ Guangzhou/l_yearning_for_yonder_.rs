//{"name":"L. Yearning for Yonder / 对远方的向往","group":"Universal Cup - The 3rd Universal Cup. Stage 38: Guangzhou","url":"https://contest.ucup.ac/contest/2036/problem/11116","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3\n1 2 3\n2 3 4\n4\n1 2 3\n1 3 4\n2 4 2\n","output":"OK n = 3 , 3 queries\nOK n = 4 , 5 queries\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::misc::value_ref::ValueRef;
use algo_lib::value_ref;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    value_ref!(Cache: FxHashMap<(usize, usize), i64> = FxHashMap::default());
    let mut query = |x: usize, y: usize| -> i64 {
        if x == y {
            return 0;
        }
        if let Some(res) = Cache::with(|cache| cache.get(&(x, y)).copied()) {
            return res;
        }
        if let Some(res) = Cache::with(|cache| cache.get(&(y, x)).copied()) {
            return res;
        }
        out.print_line(('?', x, y));
        out.flush();
        let res = input.read_long();
        Cache::with_mut(|cache| {
            cache.insert((x, y), res);
        });
        res
    };
    let mut ans = Vec::new();
    let mut rec = RecursiveFunction2::new(|rec, set: Vec<usize>, x: usize| {
        if set.len() == 1 {
            return;
        }
        // eprintln!("{}", set.len());
        let mut y = x;
        let mut dist = 0;
        for i in set.copy_iter() {
            let d = query(x, i);
            if d > dist {
                dist = d;
                y = i;
            }
        }
        let mut parts = DefaultHashMap::new(Vec::new());
        let mut roots = Vec::new();
        let mut root = FxHashMap::default();
        let d = query(x, y);
        for z in set.copy_iter() {
            let zx = query(x, z);
            let zy = query(y, z);
            parts[zx - zy].push(z);
            if zx + zy == d {
                roots.push((zx, z));
                root.insert(zx - zy, z);
            }
        }
        roots.sort();
        for ((d1, x), (d2, y)) in roots.consecutive_iter_copy() {
            ans.push((x, y, d2 - d1));
        }
        for (d, v) in parts {
            let root = root[&d];
            for v in v.copy_iter() {
                let dist = query(x, v) - query(x, root);
                Cache::with_mut(|cache| {
                    cache.insert((root, v), dist);
                });
            }
            rec.call(v, root);
        }
    });
    rec.call((1..=n).collect::<Vec<_>>(), 1);
    out.print_line(('!', ans));
    out.flush();
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
