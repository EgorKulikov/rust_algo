//{"name":"Permutation Shop","group":"CodeChef - START198A","url":"https://www.codechef.com/START198A/problems/PERMSHOP","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3\n-1 -1 -1\n3\n1 3 -1\n3\n1 2 3\n4\n-1 3 -1 -1\n6\n-1 -1 2 4 -1 -1\n","output":"2\n1\n0\n4\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::next_permutation::NextPermutation;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_utils::factorial;

type PreCalc = Vec<Vec<usize>>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_int_vec(n);

    type Mod = ModIntF;
    let mut ans = Mod::zero();
    for q in data.iter() {
        if q.len() > n {
            continue;
        }
        let mut set = FxHashSet::default();
        for x in p.copy_iter() {
            if x != -1 && x as usize > q.len() {
                set.insert(x);
            }
        }
        let free = factorial::<Mod>(n - q.len() - set.len());
        let mut mem = Memoization2d::new(n + 1, q.len() + 1, |mem, pp, pq| -> Mod {
            if pp == n {
                if pq == q.len() {
                    free
                } else {
                    Mod::zero()
                }
            } else if p[pp] != -1 {
                let cur = p[pp] as usize;
                if cur > q.len() {
                    mem.call(pp + 1, pq)
                } else if pq == q.len() || cur != q[pq] {
                    Mod::zero()
                } else {
                    mem.call(pp + 1, pq + 1)
                }
            } else {
                let mut res = mem.call(pp + 1, pq);
                if pq != q.len() {
                    res += mem.call(pp + 1, pq + 1);
                }
                res
            }
        });
        ans += mem.call(0, 0);
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = Vec::new();
    for n in [3, 6] {
        let mut p = (1..=n).collect::<Vec<_>>();
        loop {
            let mut left = Vec::new();
            let mut rem = n;
            for i in 0..n {
                if rem >= p[i] {
                    left.push(p[i]);
                    rem -= p[i];
                }
            }
            let mut right = Vec::new();
            let mut rem = n;
            for i in (0..n).rev() {
                if rem >= p[i] {
                    right.push(p[i]);
                    rem -= p[i];
                }
            }
            right.reverse();
            if left == right {
                pre_calc.push(p.clone());
            }
            if !p.next_permutation() {
                break;
            }
        }
    }

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
