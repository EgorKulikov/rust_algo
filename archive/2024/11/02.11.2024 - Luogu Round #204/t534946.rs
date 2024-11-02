//{"name":"T534946 吻秋","group":"Luogu","url":"https://www.luogu.com.cn/problem/T534946?contestId=187558","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3 6\n1 3 2 5 6\n2 7 8 2 2\n3 5 3 4 8\n2 1 5\n1 1 2\n2 2 4\n1 1 3\n1 2 1\n2 2 3\n","output":"6\n7\n2\n"},{"input":"6 5 20\n5 14 13 1 15 17\n7 7 19 3 8 6\n16 13 13 6 14 2\n12 5 4 17 12 3\n19 19 4 6 3 3\n2 5 3\n1 4 3\n2 1 1\n1 2 5\n2 4 6\n2 2 2\n1 4 2\n1 2 4\n2 1 1\n2 3 3\n2 3 3\n1 4 2\n1 4 1\n2 3 5\n1 3 4\n1 4 1\n1 1 4\n1 5 1\n2 2 4\n2 4 2\n","output":"4\n5\n12\n3\n5\n13\n13\n16\n6\n14\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T534946"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let mut a = Vec::with_capacity(m).do_with(|a| {
        for _ in 0..m {
            a.push(input.read_size_vec(n));
        }
    });

    let mut sorted = BitSet::new(m);
    let mut p = vec![0; 2 * n];
    for _ in 0..q {
        match input.read_int() {
            1 => {
                let x = input.read_size() - 1;
                let y = input.read_size() - 1;

                if sorted[x] && sorted[y] && a[x][Back(0)] <= a[y][0] {
                    continue;
                }
                if sorted[x] && sorted[y] && a[y][Back(0)] <= a[x][0] {
                    a.swap(x, y);
                    continue;
                }
                p[..n].copy_from_slice(&a[x]);
                p[n..].copy_from_slice(&a[y]);
                p.sort_unstable();
                a[x].copy_from_slice(&p[..n]);
                a[y].copy_from_slice(&p[n..]);
                sorted.set(x);
                sorted.set(y);
            }
            2 => {
                let i = input.read_size() - 1;
                let j = input.read_size() - 1;
                out.print_line(a[i][j]);
            }
            _ => unreachable!(),
        }
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
