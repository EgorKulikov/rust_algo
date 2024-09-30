//{"name":"B - 01 Graph Construction","group":"AtCoder - AtCoder Grand Contest 068","url":"https://atcoder.jp/contests/agc068/tasks/agc068_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 2 1\n","output":"4\n0011\n1100\n"},{"input":"5\n1 2 3 4 5\n","output":"5\n01010\n01010\n"},{"input":"6\n1 1 1 1 1 1\n","output":"6\n011111\n111110\n"},{"input":"10\n1 2 3 2 4 3 4 4 5 6\n","output":"21\n000101010111100011011\n011010000010101111110\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B01GraphConstruction"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut s = Str::new();
    let mut t = Str::new();
    for _ in 0..n {
        s.push(b'0');
        t.push(b'1');
    }
    let mut id = (0..n).collect_vec();
    for i in 0..n {
        let mut same = i;
        for j in i + 1..n {
            if a[j] == a[i] {
                same = j;
                break;
            }
        }
        let mut next_id = Vec::new();
        for j in id.indices() {
            if id[j] == same {
                s.push(b'1');
                t.push(b'0');
            } else if id[j] == i {
                next_id.push(same);
                s.push(b'1');
                t.push(b'1');
            } else {
                next_id.push(id[j]);
                s.push(b'1');
                t.push(b'1');
            }
        }
        id = next_id;
    }
    out.print_line(s.len());
    out.print_line(s);
    out.print_line(t);
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
