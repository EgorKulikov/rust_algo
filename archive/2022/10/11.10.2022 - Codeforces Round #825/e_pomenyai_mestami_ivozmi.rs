//{"name":"E. Поменяй местами и возьми","group":"Codeforces - Codeforces Round #825 (Div. 2)","url":"https://codeforces.com/contest/1736/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n3 1\n","output":"6\n"},{"input":"5\n7 3 9 6 12\n","output":"52\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPomenyaiMestamiIVozmi"}}}

use algo_lib::collections::arr3d::Arr3d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve_impl(n: usize, a: Vec<i32>) -> i32 {
    // struct State {
    //     a: Vec<i32>,
    //     b: Arr3d<i32>,
    //     c: Arr3d<i32>,
    // }

    const INF: i32 = i32::MIN / 2;

    let mut b = Arr3d::new(n + 1, n + 1, n + 1, INF);
    let mut c = Arr3d::new(n + 1, n + 1, n + 1, INF);

    for i in 0..n {
        for j in 0..n {
            b[(n, i, j)] = 0;
            c[(n, i, j)] = 0;
        }
    }

    for pos in (0..n).rev() {
        for free in 0..n {
            for start in (pos..n).rev() {
                b[(pos, free, start)] = {
                    if start >= n {
                        INF
                    } else {
                        if start - pos > free + 1 {
                            INF
                        } else {
                            (c[(pos + 1, free + 1 - (start - pos), start)] + a[start])
                                .max(b[(pos, free, start + 1)])
                        }
                    }
                }
            }
        }
        for free in 0..n {
            for start in (0..n).rev() {
                c[(pos, free, start)] = {
                    (c[(pos + 1, free, start)] + a[start]).max(b[(pos, free, pos.max(start + 1))])
                }
            }
        }
    }

    b[(0, 0, 0)]
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_int_vec(n);

    let ans = solve_impl(n, a);

    out_line!(ans);
}

#[test]
fn test() {
    solve_impl(500, (1..=500).collect());
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
