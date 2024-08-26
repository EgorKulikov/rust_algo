//{"name":"E - Train Delay","group":"AtCoder - Hitachi Vantara Programming Contest 2024（AtCoder Beginner Contest 368）","url":"https://atcoder.jp/contests/abc368/tasks/abc368_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3 6 15\n1 2 10 20\n1 2 20 30\n2 3 25 40\n2 3 35 50\n3 1 15 30\n3 1 45 60\n","output":"0 10 0 0 5\n"},{"input":"10 9 100\n1 10 0 1\n10 2 1 100\n10 3 1 100\n10 4 1 100\n10 5 1 100\n10 6 1 100\n10 7 1 100\n10 8 1 100\n10 9 1 100\n","output":"100 100 100 100 100 100 100 100\n"},{"input":"4 4 10\n1 2 0 1\n1 2 0 10\n2 3 100 200\n2 4 100 200\n","output":"0 0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETrainDelay"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::collections::{BTreeSet, BinaryHeap};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let x1 = input.read_long();
    let trains = input.read_vec::<(usize, usize, i64, i64)>(m).dec();

    let mut departures = vec![BTreeSet::new(); n];
    for (i, &(a, _, s, _)) in trains.iter().enumerate() {
        departures[a].insert((s, i));
    }
    let mut x = vec![0; m];
    x[0] = x1;
    let mut heap = BinaryHeap::new();
    heap.push((x1, 0));
    let mut processed = BitSet::new(m);
    while let Some((_, id)) = heap.pop() {
        if processed[id] {
            continue;
        }
        processed.set(id);
        let (_, b, _, t) = trains[id];
        for &(_, i) in departures[b].range((t, 0)..(t + x[id], 0)) {
            let cand = t + x[id] - trains[i].2;
            if x[i].maxim(cand) {
                heap.push((x[i], i));
            } else {
                break;
            }
        }
    }
    out.print_line(&x[1..]);
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
