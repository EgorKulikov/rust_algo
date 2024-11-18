//{"name":"T436524 「GFOI Round 2」Aob & Blice","group":"Luogu","url":"https://www.luogu.com.cn/problem/T436524?contestId=210735","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n0 2 0\n","output":"2\n"},{"input":"7\n0 3 2 0 5 7 0\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T436524GFOIRound2AobBlice"}}}

use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::One;
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut p = input.read_size_vec(n);
    // let p = input.read_size_vec(n).dec();

    type Mod = ModIntF;
    /*let mut ans = Mod::one();
    let mut segs = FxHashMap::default();
    for i in 0..n {
        if p[i] != 0 && p[i] != i + 1 {
            let id = i + p[i] - 1;
            let mut next = segs.get(&id).copied().unwrap_or((id + 1) / 2);
            while next <= i.max(p[i] - 1) {
                if p[next] != 0 && p[next] != next + 1 && next + p[next] - 1 != id {
                    out.print_line(0);
                    return;
                }
                let other = id + 1 - next;
                if p[other] != 0 && p[other] != other + 1 && other + p[other] - 1 != id {
                    out.print_line(0);
                    return;
                }
                if p[next] == 0 && p[other] == 0 {
                    ans *= Mod::new(2);
                    p[next] = other + 1;
                    p[other] = next + 1;
                    continue;
                }
                if p[next] == p[other] && next != other {
                    out.print_line(0);
                    return;
                }
                if p[next] == 0 {
                    p[next] = id - (p[other] - 1) + 1;
                }
                if p[other] == 0 {
                    p[other] = id - (p[next] - 1) + 1;
                }
                next += 1;
            }
            segs.insert(id, next);
        }
    }*/
    for i in 0..n {
        if p[i] != 0 && p[i] != i + 1 {
            let o = p[i] - 1;
            if p[o] == 0 {
                p[o] = i + 1;
            } else if p[o] != i + 1 {
                out.print_line(0);
                return;
            }
        }
    }
    let zero = p.iter().count_eq(&&0);
    let mut mem = Memoization1d::new(zero + 1, |mem, rem| {
        if rem == 0 {
            Mod::one()
        } else if rem == 1 {
            Mod::one()
        } else {
            mem.call(rem - 1) + Mod::from_index(rem - 1) * mem.call(rem - 2)
        }
    });

    out.print_line(mem.call(zero));
    // out.print_line(ans);

    /*let mut next = vec![0; n + 1];
    next[n] = n;
    for i in (0..n).rev() {
        if p[i] == 0 {
            next[i] = next[i + 1];
        } else {
            next[i] = i;
        }
    }
    let mut mem = Memoization1d::new(n + 1, |mem, pos| -> (Mod, Mod, Mod) {
        if pos == n {
            return (Mod::one(), Mod::one(), Mod::zero());
        }
        // let next = next[pos];
        let (_, odd, even) = mem.call(pos + 1);
        let mut res = odd + even;
        let res_even = if next[pos] == pos {
            Mod::zero()
        } else {
            odd * Mod::new(2)
        };
        let mut res_odd = if next[pos] == pos { Mod::zero() } else { even };

        res_odd += res;
        (res, res_odd, res_even)
    });*/
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
