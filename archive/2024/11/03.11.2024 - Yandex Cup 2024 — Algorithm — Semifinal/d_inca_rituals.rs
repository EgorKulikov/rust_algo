//{"name":"D. Inca rituals","group":"Yandex - Yandex Cup 2024 — Algorithm — Semifinal","url":"https://contest.yandex.com/contest/70295/problems/D/","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n4 5\n","output":"16\n"},{"input":"2\n10 20\n2024 2024\n","output":"351\n4114\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DIncaRituals"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::integer_sqrt::IntegerSqrt;
use algo_lib::numbers::interpolation::Interpolation;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let t = input.read_size();

    type Mod = ModInt7;
    let mut pw = Vec::with_capacity(60);
    for i in 0..60 {
        let mut values = vec![Mod::zero()];
        let mut last = Mod::zero();
        for j in 1..=i + 2 {
            last += Mod::from_index(j).power(i + 1);
            values.push(last);
        }
        pw.push(Interpolation::new(values));
    }

    for _ in 0..t {
        let l = input.read_long();
        let r = input.read_long();

        let mut ans = Mod::zero();
        for i in 0..60 {
            let l1 = l.upper_root(i + 1).max(2);
            let r1 = (r + 1).upper_root(i + 1).max(2);
            let c = if i == 0 { Mod::new(2) } else { Mod::one() };
            let delta = pw[i].calculate(Mod::new_from_wide(r1 - 1))
                - pw[i].calculate(Mod::new_from_wide(l1 - 1));
            ans += (Mod::new_from_wide(r + 1) * Mod::new_from_wide(r1 - l1) - delta) * c;
            ans += Mod::new_from_wide(l1 - 2) * Mod::new_from_wide(r + 1 - l) * c;
        }
        out.print_line(ans);
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
