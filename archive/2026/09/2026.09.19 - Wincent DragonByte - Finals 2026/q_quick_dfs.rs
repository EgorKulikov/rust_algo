use algo_lib::collections::fx_hash_map::{FxHashMap, FxHashSet};
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::{Input, Readable};
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let r = input.read_vec::<(i64, i64, i64, i64)>(n);
    enum Query {
        Point(i64, i64),
        Step(usize),
    }
    impl Readable for Query {
        fn read(input: &mut Input) -> Self {
            let t = input.read_int();
            match t {
                1 => Self::Step(input.read()),
                2 => Self::Point(input.read(), input.read()),
                _ => unreachable!(),
            }
        }
    }
    let q = input.read_vec::<Query>(q);
    drop(input);

    if r.len() == 1 {
        let (x1, y1, x2, y2) = r[0];
        let h = y2 - y1;

        if y1 == 0 || y2 == 0 {
            for q in q {
                match q {
                    Query::Point(x, y) => {
                        if x <= 0 {
                            let steps = -x / 2 * (h + 1) * 2;
                            if x % 2 == 0 {
                                out.print_line(steps + y.abs() + 1);
                            } else {
                                out.print_line(steps + 2 * (h + 1) - y.abs() + 1);
                            }
                        } else {
                            let add = (-x1 + 1) * (h + 1) - (h + 1);
                            let steps = x / 2 * (h + 1) * 2;
                            if x % 2 == 0 {
                                out.print_line(add + steps + y.abs() + 1);
                            } else {
                                out.print_line(add + steps + 2 * (h + 1) - y.abs() + 1);
                            }
                        }
                    }
                    Query::Step(id) => {
                        let mut id = id as i64 - 1;
                        let left = (-x1 + 1) * (h + 1);
                        if id < left {
                            let mut x = id / 2 * (h + 1) * 2;
                            id -= x * (h + 1);
                            let y;
                            if id <= h {
                                y = id;
                            } else {
                                x += 1;
                                y = 2 * h + 1 - id;
                            }
                            if y1 < 0 {
                                out.print_line((-x, -y));
                            } else {
                                out.print_line((-x, y));
                            }
                        } else {
                            id += x1 * (h + 1);
                            let mut x = id / 2 * (h + 1) * 2;
                            id -= x * (h + 1);
                            let y;
                            if id <= h {
                                y = id;
                            } else {
                                x += 1;
                                y = 2 * h + 1 - id;
                            }
                            if y1 < 0 {
                                out.print_line((x, -y));
                            } else {
                                out.print_line((x, y));
                            }
                        }
                    }
                }
            }
            return;
        }

        if x1 == 0 {
            for q in q {
                match q {
                    Query::Point(x, y) => {
                        let ans = if x == 0 {
                            if y >= 0 {
                                y + 1
                            } else if x2 == 0 {
                                y2 + 1 - y
                            } else {
                                y2 + 1 + h + 1 + y - y1
                            }
                        } else if x == 1 {
                            y2 + 1 + y2 - y
                        } else {
                            let add = x / 2 * 2 * (h + 1);
                            if x % 2 == 0 {
                                add + y - y1 + 1
                            } else {
                                add + h + 1 + y2 - y + 1
                            }
                        };
                        out.print_line(ans);
                    }
                    Query::Step(id) => {
                        let mut id = id as i64 - 1;
                        if x2 == 0 {
                            out.print_line((0, if id <= y2 { id } else { -(id - (y2 + 1)) }));
                        } else {
                            if id < 2 * (h + 1) {
                                if id <= y2 + 1 {
                                    out.print_line((0, id));
                                } else if id <= y2 + 1 + h + 1 {
                                    out.print_line((1, y2 - (id - y2 - 2)));
                                } else {
                                    out.print_line((0, y1 + (id - (y2 + 1 + h + 2))));
                                }
                            } else {
                                let mut x = id / (2 * (h + 1)) * 2;
                                id -= x * (h + 1);
                                let y = if id <= h + 1 {
                                    y1 + id
                                } else {
                                    x += 1;
                                    y1 + (2 * h + 1 - id)
                                };
                                out.print_line((x, y));
                            }
                        }
                    }
                }
            }
            return;
        }

        for q in q {
            match q {
                Query::Point(x, y) => {
                    if x == 0 {
                        if y >= 0 {
                            out.print_line(y + 1);
                        } else {
                            out.print_line(y2 + (h + 1) * (-x1) + y - y1 + 1);
                        }
                    } else if x == 1 {
                        if y >= -1 {
                            out.print_line((h + 1) * (1 - x1) + y + 2);
                        } else {
                            out.print_line((h + 1) * (x2 - x1 + 1) - (-2 - y));
                        }
                    } else if x < 0 {
                        let add = 2 * (-x / 2) * (h + 1);
                        if x % 2 == 0 {
                            out.print_line(add + y + 1);
                        } else {
                            out.print_line(add + y2 + 1 + y2 - y + 1);
                        }
                    } else {
                        let add = (2 - x1) * (h + 1);
                        if x % 2 == 0 {
                            out.print_line(add + y2 - y + 1);
                        } else {
                            out.print_line(add + h + 1 + y - y1 + 1);
                        }
                    }
                }
                Query::Step(id) => {
                    let mut id = id as i64 - 1;
                    if id <= y2 {
                        out.print_line((0, id));
                    } else if id <= y2 + (h + 1) * (-x1) {
                        let mut x = id / (2 * (h + 1)) * 2;
                        id -= (h + 1) * x;
                        let y;
                        if id <= y2 {
                            y = id;
                        } else if id <= y2 + 1 + h {
                            x += 1;
                            y = y2 - (id - y2 - 1);
                        } else {
                            x += 2;
                            y = y1 + (id - y2 - h - 2);
                        }
                        out.print_line((x, y));
                    }
                }
            }
        }
        return;
    }

    let mut set = FxHashSet::default();
    for (x1, y1, x2, y2) in r.copy_iter() {
        for x in x1..=x2 {
            for y in y1..=y2 {
                set.insert((x, y));
            }
        }
    }
    let mut id = FxHashMap::default();
    let mut order = Vec::new();
    let mut rec = RecursiveFunction2::new(|rec, x: i64, y: i64| {
        if !set.contains(&(x, y)) || id.contains_key(&(x, y)) {
            return;
        }
        id.insert((x, y), order.len());
        order.push((x, y));
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            rec.call(x + dx, y + dy);
        }
    });
    rec.call(0, 0);
    for q in q {
        match q {
            Query::Point(x, y) => {
                out.print_line(id[&(x, y)] + 1);
            }
            Query::Step(id) => {
                out.print_line(order[id - 1]);
            }
        }
    }
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
