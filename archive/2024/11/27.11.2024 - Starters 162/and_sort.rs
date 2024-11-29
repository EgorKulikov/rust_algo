//{"name":"And Sort","group":"CodeChef - START162A","url":"https://www.codechef.com/START162A/problems/ANDSORT","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n0 3 2\n4\n0 4 1 1\n10\n9 1 0 2 2 10 4 7 6 7\n","output":"2\n4\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AndSort"}}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut bad = Vec::new();
    for i in 0.. {
        if (1 << i) > n {
            break;
        }
        bad.push(vec![false; n >> i]);
    }
    for i in 1..n {
        let prev = a[i - 1];
        let next = a[i];
        let mut good = 0;
        for j in bad.indices().rev() {
            if prev.is_set(j) && !next.is_set(j) {
                bad[j][good] = true;
            }
            good *= 2;
            if !prev.is_set(j) && next.is_set(j) {
                good += 1;
            }
        }
    }
    for i in bad.indices() {
        for j in bad[i].indices() {
            for k in bad.indices() {
                if j.is_set(k) && bad[i][j.without_bit(k)] {
                    bad[i][j] = true;
                    break;
                }
            }
        }
    }
    let mut ans = 0;
    for x in 0..=n {
        let mut not_set = 0;
        let mut ok = true;
        for i in bad.indices().rev() {
            if x.is_set(i) {
                if bad[i][not_set] {
                    ok = false;
                    break;
                }
            }
            not_set *= 2;
            if !x.is_set(i) {
                not_set += 1;
            }
        }
        if ok {
            ans += 1;
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
