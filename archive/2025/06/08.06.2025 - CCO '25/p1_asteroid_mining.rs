//{"name":"P1 - Asteroid Mining","group":"DMOJ - CCO '25","url":"https://dmoj.ca/problem/cco25p1","interactive":false,"timeLimit":2000,"tests":[{"input":"6 10\n1 1\n5 2\n200 6\n9 2\n6 2\n100 1\n","output":"310\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BinaryHeap;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mass = input.read_long();
    let vm = input.read_long_pair_vec(n).sorted_by_key(|x| x.1);

    let mut chunks = BinaryHeap::new();
    let mut chunk_size = 1;
    let mut ans = 0;
    for (v, m) in vm {
        if m == chunk_size {
            chunks.push(v);
            continue;
        }
        let extra = (mass % m) / chunk_size;
        for _ in 0..extra.min(chunks.len() as i64) {
            ans += chunks.pop().unwrap();
        }
        let cap = m / chunk_size;
        let mut cur = 0;
        let mut rem = cap;
        let mut next_chunks = BinaryHeap::new();
        while let Some(v) = chunks.pop() {
            cur += v;
            rem -= 1;
            if rem == 0 {
                next_chunks.push(cur);
                cur = 0;
                rem = cap;
            }
        }
        if cur > 0 {
            next_chunks.push(cur);
        }
        chunks = next_chunks;
        chunks.push(v);
        chunk_size = m;
    }
    let extra = mass / chunk_size;
    for _ in 0..extra.min(chunks.len() as i64) {
        ans += chunks.pop().unwrap();
    }
    out.print_line(ans);
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
