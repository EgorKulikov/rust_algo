//{"name":"Dwarf Tower","group":"Eolymp - Basecamp - Educational Round #6","url":"https://eolymp.com/en/compete/ikoi44ho994uj2rcj49h0l45v4/problem/5","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n5 0 1 2 5\n5 2 3\n4 2 3\n1 4 5\n","output":"2\n"},{"input":"3 1\n2 2 1\n1 2 3\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::indexed_heap::IndexedHeap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut c = input.read_size_vec(n);
    let axy = input.read_vec::<(usize, usize, usize)>(m);

    let mut need = vec![2; m];
    let mut list = vec![Vec::new(); n];
    for i in 0..m {
        let (_, x, y) = axy[i];
        list[x - 1].push(i);
        list[y - 1].push(i);
    }
    let mut heap = IndexedHeap::new(n);
    for i in 0..n {
        heap.add_or_adjust(i, c[i]);
    }
    let mut done = BitSet::new(n);
    for _ in 0..n {
        let (cur, val) = heap.pop().unwrap();
        done.set(cur);
        c[cur] = val;
        if cur == 0 {
            out.print_line(val);
            return;
        }
        for v in list[cur].drain(..) {
            need[v] -= 1;
            if need[v] == 0 {
                let (a, x, y) = axy[v];
                if !done[a - 1] {
                    heap.add_or_relax(a - 1, c[x - 1] + c[y - 1]);
                }
            }
        }
    }
    unreachable!()
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
