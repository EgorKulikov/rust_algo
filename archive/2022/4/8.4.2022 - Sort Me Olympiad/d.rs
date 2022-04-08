//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use algo_lib::collections::arr2d::{Arr2dCharWrite, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let mut n = input.read_usize();
    let mut m = input.read_usize();
    let mut mat = input.read_table::<char>(n, m);

    let swap = if n > m {
        mat = mat.transpose();
        swap(&mut n, &mut m);
        true
    } else {
        false
    };
    let mut shift = vec![0usize; n];
    let mut val = vec!['0'; n];
    let mut ii = vec![0; n];

    let q = input.read_usize();
    for _ in 0..q {
        let tp = input.read_char();
        let at = input.read_usize() - 1;
        let by = input.read_usize();
        if (tp == 'r') ^ swap {
            shift[at] += by;
            shift[at] %= m;
        } else {
            for i in 0..n {
                ii[i] = (at + m - shift[i]) % m;
                val[i] = mat[(i, ii[i])];
            }
            let mut ind = by % n;
            for i in 0..n {
                mat[(ind, ii[ind])] = val[i];
                ind += 1;
                if ind == n {
                    ind = 0;
                }
            }
        }
    }

    let mut val = vec!['0'; m];
    for i in 0..n {
        let mut ind = (m - shift[i]) % m;
        for j in 0..m {
            val[j] = mat[(i, ind)];
            ind += 1;
            if ind == m {
                ind = 0;
            }
        }
        for j in 0..m {
            mat[(i, j)] = val[j];
        }
    }

    if swap {
        mat = mat.transpose();
    }
    output().print_table(&mat);
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
