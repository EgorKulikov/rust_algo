//{"name":"H. Qingyu's Little Training Center / 小青鱼的训练中心","group":"Universal Cup - The 3rd Universal Cup. Stage 38: Guangzhou","url":"https://contest.ucup.ac/contest/2036/problem/11112","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4 1 3\n5 2 3\n10 10 1\n","output":"1\n1000\n0100\n0010\n2\n11000\n00110\n10001\n1\n1111111111\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::{gcd, lcm};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_utils::UpperDiv;
use algo_lib::string::str::Str;

type PreCalc = Arr2d<Vec<usize>>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let mut m = input.read_size();

    if m == 0 {
        out.print_line(0);
        return;
    }

    if k == 0 {
        out.print_line(0);
        out.print_line(Str::from(vec![b'0'; n]));
        return;
    }

    let mut special = FxHashSet::default();
    for i in 0..n {
        let mut x = 0;
        for j in i..i + k {
            x.set_bit(j % n);
        }
        special.insert(x);
    }
    let mut seen = FxHashSet::default();
    let mut ans = Vec::with_capacity(m);
    for i in data[(n, k)].copy_iter() {
        if seen.contains(&i) || special.contains(&i) {
            continue;
        }
        let mut cur = i;
        let mut set = Vec::new();
        for _ in 0..n {
            if !seen.contains(&cur) {
                set.push(cur);
                seen.insert(cur);
            }
            cur = (cur >> 1) | ((cur & 1) << (n - 1));
        }
        if set.len() <= m {
            m -= set.len();
            ans.extend(set);
        }
    }
    for i in 0..gcd(k, n) {
        for j in (i..i + lcm(k, n)).step_by(k) {
            let mut cur = 0;
            for x in j..j + k {
                cur.set_bit(x % n);
            }
            if m > 0 {
                ans.push(cur);
                m -= 1;
            }
        }
    }
    out.print_line((ans.len() * (k)).upper_div(n));
    for x in ans {
        for j in 0..n {
            out.print(x.is_set(j) as i32);
        }
        out.print_line(());
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = Arr2d::new(21, 21, Vec::new());
    for n in 1..=20 {
        for i in usize::iter_all(n) {
            pre_calc[(n, i.count_ones() as usize)].push(i);
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
