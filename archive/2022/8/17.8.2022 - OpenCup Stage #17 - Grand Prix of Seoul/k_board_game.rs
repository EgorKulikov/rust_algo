//{"name":"K. Board Game","group":"Yandex - Stage 17: Grand Prix of Seoul","url":"https://official.contest.yandex.com/opencupXXII/contest/39021/problems/K/","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3 2\nRRDDD\nDRDDR\n1 2\nDRR\nRRD\n","output":"First\n"},{"input":"2\n2 2\nDRDR\nRRDD\n4 5\nRDRDDRRRD\nDDDRDRRRR\n","output":"Second\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KBoardGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let k = input.read_usize();

    let mut sum = 0;
    for _ in 0..k {
        let n = input.read_usize();
        let m = input.read_usize();
        let mut top: Str = input.read();
        let mut bottom: Str = input.read();

        if top[0] == b'D' {
            swap(&mut top, &mut bottom);
        }

        let mut c_right = 0;
        let mut j = 0;
        let mut r = Vec::with_capacity(n);
        for _ in 0..n {
            while top[j] == b'R' {
                c_right += 1;
                j += 1;
            }
            j += 1;
            r.push(c_right);
        }
        let mut c_bottom = 0;
        let mut b = Vec::with_capacity(m);
        let mut i = 0;
        for _ in 0..m {
            while bottom[i] == b'D' {
                c_bottom += 1;
                i += 1;
            }
            i += 1;
            b.push(c_bottom);
        }
        let mut row = n - 1;
        let mut col = m - 1;
        while row > 0 && col > 0 {
            if r[row - 1] < col {
                col = r[row - 1] - 1;
                row -= 1;
            } else if b[col - 1] < row {
                row = b[col - 1] - 1;
                col -= 1;
            } else {
                row -= 1;
                col -= 1;
            }
        }
        if row > 0 {
            sum += row.into_i64();
        } else {
            sum -= col.into_i64();
        }
    }

    out_line!(if sum < 0 { "First" } else { "Second" });
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
