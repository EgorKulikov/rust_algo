//{"name":"L. Flipping Paths","group":"Universal Cup - The 3rd Universal Cup. Stage 23: Hong Kong","url":"https://contest.ucup.ac/contest/1885/problem/9926","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3 3\nWBB\nBWB\nBBW\n1 5\nWWWWW\n2 2\nBB\nBB\n4 1\nW\nB\nB\nW\n","output":"YES\n2\nRRDD\nDDRR\nYES\n0\nYES\n0\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LFlippingPaths"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let c = input.read_char_table(n, m);

    for &z in b"WB" {
        let mut paths: Arr2d<Vec<Str>> = Arr2d::new(n, m, Vec::new());
        let mut good = true;
        if c[(0, 0)] != z {
            paths[(0, 0)].push(Str::new());
        }
        'outer: for i in 0..n {
            for j in 0..m {
                if i > 0 && paths[(i - 1, j)].len() % 2 == 1 {
                    let mut path = paths[(i - 1, j)].pop().unwrap();
                    path.push(b'D');
                    paths[(i, j)].push(path);
                }
                if (c[(i, j)] == z) != (paths[(i, j)].len() % 2 == 0) {
                    if j == 0 {
                        good = false;
                        break 'outer;
                    }
                    if j > 0 && paths[(i, j - 1)].len() % 2 == 1 {
                        let mut path = paths[(i, j - 1)].pop().unwrap();
                        path.push(b'R');
                        paths[(i, j)].push(path);
                    } else {
                        if i == n - 1 {
                            good = false;
                            break 'outer;
                        }
                        let mut x = 0;
                        let mut y = 0;
                        let mut p1 = Str::new();
                        let mut p2 = Str::new();
                        'inner: for k in (0..=j - 1).rev() {
                            for l in 0..=i {
                                if paths[(l, k)].len() >= 2 {
                                    x = l;
                                    y = k;
                                    p1 = paths[(l, k)].pop().unwrap();
                                    p2 = paths[(l, k)].pop().unwrap();
                                    break 'inner;
                                }
                            }
                        }
                        for _ in x..i {
                            p1.push(b'D');
                            p2.push(b'D');
                        }
                        for _ in y..j - 1 {
                            p1.push(b'R');
                            p2.push(b'R');
                        }
                        p1.push(b'R');
                        paths[(i, j)].push(p1);
                        p2.push(b'D');
                        paths[(i + 1, j - 1)].push(p2);
                    }
                } else if j > 0 && i == n - 1 && paths[(i, j - 1)].len() % 2 == 1 {
                    good = false;
                    break 'outer;
                }
            }
        }
        if !good {
            continue;
        }
        let mut ans = Vec::new();
        for i in 0..n {
            for j in 0..m {
                for mut path in paths[(i, j)].drain(..) {
                    for _ in i + 1..n {
                        path.push(b'D');
                    }
                    for _ in j + 1..m {
                        path.push(b'R');
                    }
                    ans.push(path);
                }
            }
        }
        out.print_line(true);
        out.print_line(ans.len());
        out.print_per_line(&ans);
        return;
    }
    out.print_line(false);
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
