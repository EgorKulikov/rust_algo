use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::VecDeque;
use std::mem::swap;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let g = input.read_size();
    let d = input.read_size();
    let mut d1 = input.read_size_vec(g);
    let mut d2 = input.read_size_vec(d);
    drop(input);

    if d2.contains(&(g + d)) {
        swap(&mut d1, &mut d2);
    }
    enum Card {
        Single(usize),
        Pair(usize, usize, usize),
    }
    let mut a = VecDeque::new();
    let mut b = VecDeque::new();
    for i in d1 {
        a.push_back(Card::Single(i));
    }
    for i in d2 {
        b.push_back(Card::Single(i));
    }
    let mut ans = Vec::new();
    for id in 0.. {
        if b.is_empty() {
            break;
        }
        ans.push(false);
        match (a.pop_front().unwrap(), b.pop_front().unwrap()) {
            (Card::Single(x), Card::Single(y)) => {
                if x > y {
                    a.push_back(Card::Pair(x, y, id));
                } else {
                    b.push_back(Card::Pair(y, x, id));
                }
            }
            (Card::Single(x), Card::Pair(y1, y2, w_id)) => {
                if x > y1 {
                    ans[w_id] = true;
                    a.push_back(Card::Pair(x, y1, id));
                    b.push_front(Card::Single(y2));
                } else if x > y2 {
                    a.push_back(Card::Pair(x, y2, id));
                    b.push_front(Card::Single(y1));
                } else {
                    ans[w_id] = true;
                    b.push_back(Card::Pair(y1, x, id));
                    b.push_front(Card::Single(y2));
                }
            }
            (Card::Pair(x1, x2, w_id), Card::Single(y)) => {
                if x2 > y {
                    a.push_back(Card::Pair(x2, y, id));
                    a.push_front(Card::Single(x1));
                } else if x1 > y {
                    ans[w_id] = true;
                    a.push_back(Card::Pair(x1, y, id));
                    a.push_front(Card::Single(x2));
                } else {
                    b.push_back(Card::Pair(y, x2, id));
                    a.push_front(Card::Single(x1));
                }
            }
            (Card::Pair(x1, x2, x_id), Card::Pair(y1, y2, y_id)) => {
                if x1 > y1 {
                    ans[x_id] = true;
                    ans[y_id] = true;
                    a.push_back(Card::Pair(x1, y1, id));
                    a.push_front(Card::Single(x2));
                    b.push_front(Card::Single(y2));
                } else if x1 > y2 {
                    ans[x_id] = true;
                    a.push_back(Card::Pair(x1, y2, id));
                    a.push_front(Card::Single(x2));
                    b.push_front(Card::Single(y1));
                } else {
                    ans[y_id] = true;
                    b.push_back(Card::Pair(y1, x2, id));
                    a.push_front(Card::Single(x1));
                    b.push_front(Card::Single(y2));
                }
            }
        }
    }
    out.print(ans.len());
    out.print(" ");
    let mut next = ans.len() % 4;
    if next == 0 {
        next = 4;
    }
    let mut cur = 0;
    while cur < ans.len() {
        let mut val = 0;
        for i in cur..next {
            val *= 2;
            if ans[i] {
                val += 1;
            }
        }
        out.print(if val < 10 {
            val + b'0'
        } else {
            val - 10 + b'a'
        });
        cur = next;
        next += 4;
    }
    out.print_line(());
}

pub static TASK_TYPE: TaskType = TaskType::Classic;
pub static TEST_TYPE: TestType = TestType::MultiNumber;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let pre_calc = ();
    let is_exhausted = run_parallel(input, &mut output, true, pre_calc, solve);
    eprint!("\x1B[0m");
    output.flush();
    is_exhausted
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
