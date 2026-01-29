//{"name":"T724664 For The Emperor！","group":"Luogu","url":"https://www.luogu.com.cn/problem/T724664?contestId=304744","interactive":false,"timeLimit":2000,"tests":[{"input":"5 1\n1 3 1 1 1\n1\n3\n7\n2\n1\n","output":"1\n"},{"input":"6 2\n2 2 7 3 8 4\n4 4\n5 6\n1 7\n3 9\n3 8\n6 8\n","output":"62390273\n"},{"input":"6 2\n2 2 6 3 1 10\n5 8\n7 9\n9 7\n9 3\n10 6\n10 4\n","output":"1\n"},{"input":"5 5\n1 1 1 1 1\n1 1 1 1 1\n1 1 1 1 1\n1 1 1 1 1\n1 1 1 1 1\n1 1 1 1 1\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_int_vec(n);
    let b = input.read_int_table(n, k);

    #[derive(Eq, PartialEq, Clone, Copy, Hash)]
    enum Cell {
        Empty,
        We(i32),
        Them(i32),
    }

    impl Cell {
        fn flip(&mut self) {
            *self = match self {
                Cell::Empty => Cell::Empty,
                Cell::We(p) => Cell::Them(*p),
                Cell::Them(p) => Cell::We(*p),
            }
        }
    }

    fn flip_board(board: &mut Arr2d<Cell>) {
        for i in 0..board.d2() {
            board.swap(0, i, 2, i);
        }
        let n = board.d2();
        for i in 0..3 {
            for j in 0..n {
                board[(i, j)].flip();
            }
        }
    }
    type Mod = ModIntF;
    let mut ans = Mod::from(0);
    let mut mem = Memoization::new(|mem, board: Arr2d<Cell>| -> bool {
        for i in 0..n {
            if let Cell::We(_) = board[(2, i)] {
                return true;
            }
        }
        for row in 0..2 {
            for col in 0..n {
                if let Cell::We(p) = board[(row, col)] {
                    for ncol in col.saturating_sub(1)..=(col + 1).min(n - 1) {
                        match board[(row + 1, ncol)] {
                            Cell::Empty => {
                                let mut new_board = board.clone();
                                new_board[(row, col)] = Cell::Empty;
                                new_board[(row + 1, ncol)] = Cell::We(p);
                                flip_board(&mut new_board);
                                if !mem.call(new_board) {
                                    return true;
                                }
                            }
                            Cell::Them(q) => {
                                if col == ncol {
                                    continue;
                                }
                                let mut new_board = board.clone();
                                new_board[(row, col)] = Cell::Empty;
                                new_board[(row + 1, ncol)] = if p >= q {
                                    Cell::We(p - q)
                                } else {
                                    Cell::Them(q - p)
                                };
                                flip_board(&mut new_board);
                                if !mem.call(new_board) {
                                    return true;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        false
    });
    let mut rec = RecursiveFunction::new(|rec, c: Vec<i32>| {
        if c.len() == n {
            let board = Arr2d::with_gen(3, n, |i, j| match i {
                0 => Cell::We(a[j]),
                1 => Cell::Empty,
                2 => Cell::Them(c[j]),
                _ => unreachable!(),
            });
            if mem.call(board) {
                ans += 1;
            }
            eprintln!("Done");
            return;
        }
        for i in 0..k {
            let mut c = c.clone();
            c.push(b[(c.len(), i)]);
            rec.call(c);
        }
    });
    rec.call(Vec::new());
    out.print_line(ans / Mod::from(k).power(n));
}

pub static TEST_TYPE: TestType = TestType::Single;
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
