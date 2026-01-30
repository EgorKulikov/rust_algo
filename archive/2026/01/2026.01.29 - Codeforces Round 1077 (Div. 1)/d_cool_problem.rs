//{"name":"D. Cool Problem","group":"Codeforces - Codeforces Round 1077 (Div. 1)","url":"https://codeforces.com/contest/2187/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n1 1 2\n0\n1 1 2\n?\n3 7 5\n?0?\n7 114514 191981\n?1?????\n","output":"1\n3\n100\n8039591\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_utils::UpperDiv;
use algo_lib::string::str::StrReader;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_long();
    let y = input.read_long();
    let s = input.read_str();

    // #[derive(Clone)]
    // struct Set {
    //     segs: BitSet,
    //     delta: i32,
    // }
    //
    // impl Set {
    //     fn join(a: Set, b: Set) -> Set {
    //
    //     }
    // }
    //
    // let mut a = Set {
    //     segs: vec![(0, 1)].into(),
    //     delta: 0,
    // };
    // let mut b = Set {
    //     segs: VecDeque::new(),
    //     delta: 0,
    // };
    let mut a = BitSet::new(n + 1);
    let mut b = BitSet::new(n + 1);
    a.set(0);
    for i in 0..n {
        match s[i] {
            b'0' => {
                a <<= 1;
                b >>= 1;
            }
            b'1' => {
                swap(&mut a, &mut b);
            }
            b'?' => {
                let add_b = a.clone();
                let add_a = b.clone();
                // a.delta += 1;
                // b.delta -= 1;
                a <<= 1;
                b >>= 1;
                a |= &add_a;
                b |= &add_b;
                // a = Set::join(a, add_a);
                // b = Set::join(b, add_b);
            }
            _ => unreachable!(),
        }
        /*if !b.segs.is_empty() && b.segs[0].0 + b.delta == 0 {
            if b.segs[0].1 + b.delta == 1 {
                b.segs.pop_front();
            } else {
                b.segs[0].0 += 1;
            }
            if !a.segs.is_empty() {
                if a.segs[0].0 + a.delta > 1 {
                    a.segs.push_front((-a.delta, 1 - a.delta));
                } else if a.segs[0].0 + a.delta == 1 {
                    a.segs[0].0 -= 1;
                }
            } else {
                a.segs.push_front((-a.delta, 1 - a.delta));
            }
        }*/
        // if i == 50000 {
        //     eprintln!("{} {}", a.segs.len(), b.segs.len());
        // }
        if b[0] {
            b.unset(0);
            a.set(0);
        }
    }
    let mut values = FxHashSet::default();
    /*for (mut f, mut t) in a.segs.copy_iter() {
    f += a.delta;
    t += a.delta;
    if t <= 0 {
        t = 1;
    }
    assert!(f >= 0);
    f.maxim(0);
    for i in f..t {*/
    for i in a.iter() {
        let i = i as usize;
        let mut sum = 0;
        let mut cur = 0;
        let ones = n - i;
        sum += y * ones.upper_div(2) as i64;
        if ones % 2 == 1 {
            cur = y;
        }
        sum += (i * (i + 1) / 2) as i64 * x + cur * i as i64;
        values.insert(sum);
    }
    // }
    /*for (mut f, mut t) in b.segs.copy_iter() {
    f += b.delta;
    t += b.delta;
    assert!(f > 0);
    for i in f..t {*/
    for i in b.iter() {
        let i = i as usize;
        let mut sum = 0;
        let mut cur = 0;
        let ones = n - i - 1;
        sum += y * ones.upper_div(2) as i64;
        if ones % 2 == 1 {
            cur = y;
        }
        sum += (i * (i + 1) / 2) as i64 * x + cur * i as i64;
        cur += x * i as i64;
        sum += y - cur;
        values.insert(sum);
    }
    // }
    type Mod = ModIntF;
    let mut ans = Mod::from(0);
    for v in values {
        ans += v;
    }
    out.print_line(ans);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
