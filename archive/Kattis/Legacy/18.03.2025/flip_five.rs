//{"name":"Flip Five","group":"Kattis","url":"https://open.kattis.com/problems/flipfive","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n*..\n**.\n*..\n***\n*..\n..*\n","output":"1\n3\n\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let t = input.read_vec::<u8>(9);

    let mut target = 0;
    for i in 0..9 {
        if t[i] == b'*' {
            target.set_bit(i);
        }
    }
    let mut moves = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            let mut current = 1 << (i * 3 + j);
            for (r, c) in D4::iter(i, j, 3, 3) {
                current.set_bit(r * 3 + c);
            }
            moves.push(current);
        }
    }
    let mut ans = None;
    for i in usize::iter_all(9) {
        let mut cur = 0;
        for j in 0..9 {
            if i.is_set(j) {
                cur ^= moves[j];
            }
        }
        if cur == target {
            ans.minim(i.count_ones());
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
