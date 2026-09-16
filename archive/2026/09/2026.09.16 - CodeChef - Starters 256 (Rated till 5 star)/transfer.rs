use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::series::sum_arithmetic_series;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    #[derive(Copy, Clone)]
    struct Segment {
        len: usize,
        val: usize,
    }

    fn calc(mut was: Vec<Segment>, mut target: Vec<Segment>) -> i64 {
        let mut ans = 0;
        let mut i = 0;
        let mut j = 0;
        let mut delta = 0;
        while i < was.len() && j < target.len() {
            let cur_len = was[i].len.min(target[j].len);
            ans += sum_arithmetic_series(delta, was[i].val as i64 - target[j].val as i64, cur_len as i64);
            delta += (cur_len as i64) * (was[i].val as i64 - target[j].val as i64);
            was[i].len -= cur_len;
            if was[i].len == 0 {
                i += 1;
            }
            target[j].len -= cur_len;
            if target[j].len == 0 {
                j += 1;
            }
        }
        assert_eq!(i, was.len());
        assert_eq!(j, target.len());
        assert_eq!(delta, 0);
        ans
    }

    fn balance(cur: &mut Vec<Segment>, prev: Segment) -> (i64, bool) {
        let mut was = vec![prev];
        for x in cur.iter() {
            was.push(*x);
        }
        if prev.val < cur[0].val {
            *cur = was;
            (0, false)
        } else if prev.val == cur[0].val {
            cur[0].len += prev.len;
            (0, true)
        } else {
            let mut sum = prev.len * prev.val;
            let mut len = prev.len;
            for x in cur.iter() {
                sum += x.len * x.val;
                len += x.len;
            }
            let target = if sum % len == 0 {
                vec![Segment { len, val: sum / len }]
            } else {
                vec![Segment { len: len - sum % len, val: sum / len }, Segment { len: sum % len, val: sum / len + 1 }]
            };
            *cur = target.clone();
            (calc(was, target), true)
        }
    }

    let mut ans = 0;
    let mut stack = Vec::new();
    for i in 0..n {
        let mut cur = vec![Segment { len: 1, val: a[i] }];
        while let Some(prev) = stack.pop() {
            let (add, cont) = balance(&mut cur, prev);
            ans += add;
            if !cont {
                break;
            }
        }
        stack.extend(cur);
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
