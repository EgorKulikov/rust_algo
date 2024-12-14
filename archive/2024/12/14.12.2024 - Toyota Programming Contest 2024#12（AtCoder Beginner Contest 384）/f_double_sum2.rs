//{"name":"F - Double Sum 2","group":"AtCoder - Toyota Programming Contest 2024#12（AtCoder Beginner Contest 384）","url":"https://atcoder.jp/contests/abc384/tasks/abc384_f","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n4 8\n","output":"5\n"},{"input":"3\n51 44 63\n","output":"384\n"},{"input":"8\n577752 258461 183221 889769 278633 577212 392309 326001\n","output":"20241214\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FDoubleSum2"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let sum = a.copy_sum();
    let mut ans = sum * 2 * n;
    for i in 1..26 {
        let mut qty = DefaultHashMap::<_, usize>::new();
        let mut sum = DefaultHashMap::<_, usize>::new();
        for x in a.copy_iter() {
            qty[x & usize::all_bits(i)] += 1;
            sum[x & usize::all_bits(i)] += x;
        }
        for k in qty.keys().copied() {
            let ok = ((1 << i) - k) % (1 << i);
            if !qty.contains_key(&ok) {
                continue;
            }
            let left = qty[ok] * sum[k];
            let right = qty[k] * sum[ok];
            ans -= (left + right) >> i;
        }
    }
    for i in 0..n {
        let mut x = a[i];
        while x % 2 == 0 {
            x /= 2;
        }
        ans += x;
    }
    out.print_line(ans / 2);
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
