//{"name":"T539331 排队","group":"Luogu","url":"https://www.luogu.com.cn/problem/T539331?contestId=218363","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2 1 4 3\n","output":"1 4 2 3\n"},{"input":"6\n6 5 4 3 2 1\n","output":"1 2 3 4 5 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T539331"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let inv = input.read_size_vec(n).dec();

    let p = inv.inv();
    let mut done = BitSet::new(n);
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        if done[p[i]] {
            continue;
        }
        ans.push(i);
        done.set(p[i]);
        let mut min = i;
        if p[i] == 0
            || done[p[i] - 1]
            || p[i] != n - 1 && !done[p[i] + 1] && inv[p[i] - 1] < inv[p[i] + 1]
        {
            for j in p[i] + 1..n {
                if !done[j] && inv[j] > min {
                    ans.push(inv[j]);
                    min = inv[j];
                    done.set(j);
                } else {
                    break;
                }
            }
        } else {
            for j in (0..p[i]).rev() {
                if !done[j] && inv[j] > min {
                    ans.push(inv[j]);
                    min = inv[j];
                    done.set(j);
                } else {
                    break;
                }
            }
        }
    }
    out.print_line(ans.inc());
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
