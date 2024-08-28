//{"name":"coderun_4924","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"coderun_4924"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::StrReader;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_int();
    let s = input.read_str_vec(n);

    let mut dsu = DSU::new(20);
    let mut present = BitSet::new(20);
    for s in &s {
        let a = (s[0] - b'A') as usize;
        let b = (s[1] - b'A') as usize;
        dsu.join(a, b);
        present.set(a);
        present.set(b);
    }
    for i in 0..20 {
        for j in 0..i {
            if present[i] && present[j] && dsu.get(i) != dsu.get(j) {
                out.print_line(0);
                return;
            }
        }
    }

    type Mod = ModIntF;
    let odd = Mod::new((k + 1) / 2);
    let even = Mod::new(k / 2);
    let mut ways = vec![Mod::zero(); 1 << 20];
    ways[0] = Mod::one();
    let mut next = vec![Mod::zero(); 1 << 20];
    let mut mult = Mod::one();
    for s in &s {
        let a = (s[0] - b'A') as usize;
        let b = (s[1] - b'A') as usize;

        if a == b {
            mult *= Mod::new(k);
            continue;
        }
        next.fill(Mod::zero());
        for i in usize::iter_all(20) {
            next[i] += ways[i] * even;
            next[i ^ (1 << a) ^ (1 << b)] += ways[i] * odd;
        }
        swap(&mut ways, &mut next);
    }
    let mut ans = Mod::zero();
    for i in usize::iter_all(20) {
        if i.count_ones() <= 2 {
            ans += ways[i];
        }
    }
    out.print_line(ans * mult);
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
