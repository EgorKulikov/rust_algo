//{"name":"C. Анонимность наше все","group":"Codeforces - Codeforces Round #773 (Div. 1)","url":"https://codeforces.com/contest/1641/problem/C","interactive":false,"timeLimit":1500,"tests":[{"input":"6 9\n0 4 5 0\n1 5\n1 6\n0 4 6 1\n1 6\n0 2 5 1\n0 2 2 0\n1 3\n1 2\n","output":"NO\nN/A\nYES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAnonimnostNasheVse"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let q = input.read_usize();

    #[derive(Default, Clone, Copy)]
    struct Node {
        not_ill: bool,
        closest: Option<usize>,
    }

    let mut nodes = vec![Node::default(); 4 * n];
    for _ in 0..q {
        let t = input.read_usize();
        if t == 0 {
            let l = input.read_usize() - 1;
            let r = input.read_usize();
            let x = input.read_usize();
            if x == 0 {
                let mut update = RecursiveFunction3::new(|f, root: usize, from, to| {
                    if l >= to || r <= from || nodes[root].not_ill {
                        return;
                    }
                    if l <= from && to <= r {
                        nodes[root].not_ill = true;
                        return;
                    }
                    let mid = (from + to) >> 1;
                    f.call(2 * root + 1, from, mid);
                    f.call(2 * root + 2, mid, to);
                    nodes[root].not_ill =
                        nodes[2 * root + 1].not_ill && nodes[2 * root + 2].not_ill;
                });
                update.call(0, 0, n);
            } else {
                let mut update = RecursiveFunction3::new(|f, root: usize, from, to| {
                    nodes[root].closest.minim(r);
                    if from + 1 != to {
                        let mid = (from + to) >> 1;
                        if l < mid {
                            f.call(2 * root + 1, from, mid);
                        } else {
                            f.call(2 * root + 2, mid, to);
                        }
                    }
                });
                update.call(0, 0, n);
            }
        } else {
            let id = input.read_usize() - 1;
            let mut query1 = RecursiveFunction3::new(|f, root: usize, from, to| {
                if nodes[root].not_ill {
                    return true;
                }
                if from + 1 != to {
                    let mid = (from + to) >> 1;
                    if id < mid {
                        f.call(2 * root + 1, from, mid)
                    } else {
                        f.call(2 * root + 2, mid, to)
                    }
                } else {
                    false
                }
            });
            if query1.call(0, 0, n) {
                out_line!("NO");
                continue;
            }
            fn min(a: Option<usize>, b: Option<usize>) -> Option<usize> {
                match a {
                    None => b,
                    Some(v) => match b {
                        None => a,
                        Some(u) => Some(v.min(u)),
                    },
                }
            }
            let mut query2 =
                RecursiveFunction3::new(|f, root: usize, from, to| -> (bool, Option<usize>) {
                    if from + 1 != to {
                        let mid = (from + to) >> 1;
                        if id < mid {
                            f.call(2 * root + 1, from, mid)
                        } else {
                            let (cont, right) = f.call(2 * root + 2, mid, to);
                            if cont {
                                if nodes[2 * root + 1].not_ill {
                                    (true, min(right, nodes[2 * root + 1].closest))
                                } else {
                                    let mut inner_query = RecursiveFunction3::new(
                                        |f, root: usize, from, to| -> Option<usize> {
                                            if from + 1 == to {
                                                return None;
                                            }
                                            let mid = (from + to) >> 1;
                                            if nodes[2 * root + 2].not_ill {
                                                min(
                                                    f.call(2 * root + 1, from, mid),
                                                    nodes[2 * root + 2].closest,
                                                )
                                            } else {
                                                f.call(2 * root + 2, mid, to)
                                            }
                                        },
                                    );
                                    (false, min(right, inner_query.call(2 * root + 1, from, mid)))
                                }
                            } else {
                                (false, right)
                            }
                        }
                    } else {
                        (true, nodes[root].closest)
                    }
                });
            let to_right = query2.call(0, 0, n).1;
            match to_right {
                None => {
                    out_line!("N/A");
                }
                Some(r) => {
                    let mut query3 =
                        RecursiveFunction3::new(|f, root: usize, from, to| -> (bool, usize) {
                            if from + 1 != to {
                                let mid = (from + to) >> 1;
                                if id < mid {
                                    let (cont, right) = f.call(2 * root + 1, from, mid);
                                    if cont {
                                        if nodes[2 * root + 2].not_ill {
                                            (true, to)
                                        } else {
                                            let mut inner_query = RecursiveFunction3::new(
                                                |f, root: usize, from, to| -> usize {
                                                    if from + 1 == to {
                                                        return from;
                                                    }
                                                    let mid = (from + to) >> 1;
                                                    if nodes[2 * root + 1].not_ill {
                                                        f.call(2 * root + 2, mid, to)
                                                    } else {
                                                        f.call(2 * root + 1, from, mid)
                                                    }
                                                },
                                            );
                                            (false, inner_query.call(2 * root + 2, mid, to))
                                        }
                                    } else {
                                        (false, right)
                                    }
                                } else {
                                    f.call(2 * root + 2, mid, to)
                                }
                            } else {
                                (true, to)
                            }
                        });
                    if query3.call(0, 0, n).1 >= r {
                        out_line!("YES");
                    } else {
                        out_line!("N/A");
                    }
                }
            }
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
