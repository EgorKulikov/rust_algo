//{"name":"C - Range Sums 2","group":"AtCoder - AtCoder Regular Contest 192 (Div. 2)","url":"https://atcoder.jp/contests/arc192/tasks/arc192_c","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut request = |i: usize, j: usize| -> i64 {
        output!(out, "? {} {}", i, j);
        out.flush();
        input.read_long()
    };
    let mut ans = vec![1, 2];
    let mut vals = vec![0, request(1, 2)];
    let mut map = FxHashMap::default();
    for i in 3..=n {
        let s1i = request(ans[0], i);
        let sni = request(ans[Back(0)], i);
        if s1i < vals[Back(0)] && sni < vals[Back(0)] {
            map.insert(i, s1i + sni - vals[Back(0)]);
            let pos = vals.lower_bound(&s1i);
            ans.insert(pos, i);
            vals.insert(pos, s1i);
        } else if s1i > sni {
            map.insert(ans[Back(0)], vals[Back(0)] + sni - s1i);
            ans.push(i);
            vals.push(s1i);
        } else {
            map.insert(ans[0], vals[Back(0)] + s1i - sni);
            ans.insert(0, i);
            let delta = sni - vals[Back(0)];
            for i in vals.indices() {
                vals[i] += delta;
            }
            vals.insert(0, 0);
            vals[1] = s1i;
        }
    }
    let s12 = request(ans[0], ans[1]);
    let p1 = s12 - map[&ans[1]];
    map.insert(ans[0], p1);
    let s2n = request(ans[Back(1)], ans[Back(0)]);
    let pn = s2n - map[&ans[Back(1)]];
    map.insert(ans[Back(0)], pn);
    let mut a = Vec::with_gen(n, |i| map[&ans[i]]);
    if ans.copy_find(1).unwrap() > ans.copy_find(2).unwrap() {
        ans.reverse();
        a.reverse();
    }
    let ans = ans.dec();
    let p = ans.inv();
    out.print_line(('!', p.inc(), a));
    out.flush();
}

pub static TEST_TYPE: TestType = TestType::Single;
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
