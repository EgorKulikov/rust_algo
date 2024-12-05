//{"name":"B. Multi-Slot Machine","group":"Yandex - Yandex Cup 2024 — Algorithm — Final","url":"https://contest.yandex.com/contest/72057/problems/B/","interactive":false,"timeLimit":3000,"tests":[{"input":"4 3 3 5\n2 1 3 1\n3\n4 2 3\n1\n2\n2\n1 3\n1 2\n3 1\n2 1\n2 2\n3 2\n","output":"N\nY\nN\nY\nN\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMultiSlotMachine"}}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::random;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n).dec();
    let drums = input.read_vec::<Vec<usize>>(m).dec();

    const LEN: usize = 50;
    let val = Arr2d::generate(n, LEN, |_, _| random().next(k));
    let shift = Arr2d::generate(m, LEN, |i, j| {
        let mut sum = 0;
        for d in drums[i].copy_iter() {
            sum += val[(d, j)];
        }
        sum % k
    });
    let mut state = vec![0; LEN];
    for i in 0..n {
        for j in 0..LEN {
            state[j] = (state[j] + val[(i, j)] * a[i]) % k;
        }
    }
    let mut same = vec![0; LEN];
    for i in 0..n {
        for j in 0..LEN {
            same[j] = (same[j] + val[(i, j)]) % k;
        }
    }
    let mut good = FxHashSet::default();
    for i in 0..k {
        let mut cur = vec![0; LEN];
        for j in 0..LEN {
            cur[j] = (same[j] * i) % k;
        }
        good.insert(cur);
    }
    for _ in 0..q {
        let l = input.read_size() - 1;
        let w = input.read_size();
        for j in 0..LEN {
            state[j] = (state[j] + w * shift[(l, j)]) % k;
        }
        if good.contains(&state) {
            out.print_line("Y");
        } else {
            out.print_line("N");
        }
    }
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
