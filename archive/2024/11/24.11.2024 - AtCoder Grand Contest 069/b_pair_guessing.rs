//{"name":"B - Pair Guessing","group":"AtCoder - AtCoder Grand Contest 069","url":"https://atcoder.jp/contests/agc069/tasks/agc069_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n01\n11\n2\n11\n11\n10\n0101011110\n1100100001\n1101100000\n0111101010\n1000011001\n1110101010\n1110110100\n1110000110\n0000001011\n1001111100\n","output":"Yes\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPairGuessing"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_char_table(n, n);

    if n == 1 {
        out.print_line(true);
        return;
    }
    let need = if n == 2 { 1 } else { n - 2 };
    let mut dsu = DSU::new(2 * n);
    for i in 0..n {
        for j in 0..n {
            if s[(i, j)] == b'0' {
                dsu.join(i, n + j);
            }
        }
    }
    let mut has = 0;
    for i in 0..2 * n {
        if dsu.get(i) == i {
            has += dsu.size(i) - 1;
        }
    }
    out.print_line(has >= need);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
