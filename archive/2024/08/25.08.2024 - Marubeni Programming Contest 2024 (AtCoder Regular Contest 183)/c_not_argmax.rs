//{"name":"C - Not Argmax","group":"AtCoder - Marubeni Programming Contest 2024 (AtCoder Regular Contest 183)","url":"https://atcoder.jp/contests/arc183/tasks/arc183_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n1 3 2\n1 2 1\n","output":"1\n"},{"input":"5 1\n1 1 1\n","output":"0\n"},{"input":"10 5\n3 8 4\n3 10 4\n1 7 2\n1 8 3\n3 8 7\n","output":"1598400\n"},{"input":"15 17\n2 11 9\n2 15 13\n1 14 2\n5 11 5\n3 15 11\n1 6 2\n4 15 12\n3 11 6\n9 13 10\n2 14 6\n10 15 11\n1 8 6\n6 14 8\n2 10 2\n6 12 6\n3 14 12\n2 6 2\n","output":"921467228\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNotArgmax"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use std::collections::HashSet;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let cond = input.read_vec::<(usize, usize, usize)>(m);

    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(n + 1);
    let mut bit_set_pools = vec![Vec::new(); n + 1];
    let mut remove_pools = vec![Vec::new(); n + 1];
    let mut add_pools = vec![Vec::new(); n + 1];
    let mut set = (0..m).collect::<HashSet<_>>();
    let mut mem = Memoization2d::new(n + 2, n + 2, |mem, f, t| {
        if f > t {
            return Mod::one();
        }
        let mut bad = bit_set_pools[t - f + 1]
            .pop()
            .unwrap_or_else(|| BitSet::new(t - f + 1));
        let mut add = add_pools[t - f + 1]
            .pop()
            .unwrap_or_else(|| vec![Vec::new(); t - f + 1]);
        let mut remove = remove_pools[t - f + 1]
            .pop()
            .unwrap_or_else(|| vec![Vec::new(); t - f + 1]);
        for &i in &set {
            let (l, r, x) = cond[i];
            bad.set(x - f);
            remove[l - f].push(i);
            add[r - f].push(i);
        }
        let mut right_set = HashSet::new();
        let mut ans = Mod::zero();
        for i in f..=t {
            for j in remove[i - f].drain(..) {
                set.remove(&j);
            }
            if !bad[i - f] {
                let right_ans = mem.call(i + 1, t);
                swap(&mut right_set, &mut set);
                let left_ans = mem.call(f, i - 1);
                swap(&mut right_set, &mut set);
                ans += left_ans * right_ans * c.c(t - f, i - f);
            } else {
                bad.unset(i - f);
            }
            for j in add[i - f].drain(..) {
                right_set.insert(j);
            }
        }
        swap(&mut right_set, &mut set);
        bit_set_pools[t - f + 1].push(bad);
        add_pools[t - f + 1].push(add);
        remove_pools[t - f + 1].push(remove);
        ans
    });
    out.print_line(mem.call(1, n));
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
