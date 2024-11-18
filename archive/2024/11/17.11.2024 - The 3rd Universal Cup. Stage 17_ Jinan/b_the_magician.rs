//{"name":"B. The Magician","group":"Universal Cup - The 3rd Universal Cup. Stage 17: Jinan","url":"https://contest.ucup.ac/contest/1843/problem/9549","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5\n2H 3H 4H 5H 6D\n1 1 1 1 0 0\n5\n2S 3S 4D 5C 6D\n0 0 1 0 1 1\n5\n2S 3S 4D 5C 6D\n0 0 1 0 1 0\n13\nAS 2S 3S 4S 5H 6H 7H 8H 9H TH JH QH KH\n0 0 0 0 0 1\n","output":"1\n1\n0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTheMagician"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let cards = input.read_str_vec(n);
    let t = input.read::<[usize; 6]>();

    let mut qty = [0usize; 4];
    for s in cards {
        match s[1] {
            b'D' => qty[0] += 1,
            b'C' => qty[1] += 1,
            b'H' => qty[2] += 1,
            b'S' => qty[3] += 1,
            _ => unreachable!(),
        }
    }
    let mut ans = 0;
    for i in 0..4 {
        ans += qty[i] / 5;
        qty[i] %= 5;
    }
    let total = qty.iter().sum::<usize>();
    let mut add = 0;
    for i in 1usize..16 {
        if i.count_ones() as usize * 5 > total {
            continue;
        }
        let mut have = t[4] + t[5];
        for j in 0..4 {
            if i.is_set(j) {
                have += (qty[j] + 3 * t[j]).min(5);
            }
        }
        if have >= i.count_ones() as usize * 5 {
            add.maxim(i.count_ones() as usize);
        }
    }
    out.print_line(ans + add);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
