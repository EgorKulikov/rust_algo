//{"name":"E. Гуманоид","group":"Codeforces - Codeforces Round  #834 (Div. 3)","url":"https://codeforces.com/contest/1759/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n4 1\n2 1 8 9\n3 3\n6 2 60\n4 5\n5 1 100 5\n3 2\n38 6 3\n1 1\n12\n4 6\n12 12 36 100\n4 1\n2 1 1 15\n3 5\n15 1 13\n","output":"4\n3\n3\n3\n0\n4\n4\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EGumanoid"}}}

use algo_lib::collections::arr3d::Arr3d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let h = input.read_long();
    let mut a = input.read_long_vec(n);

    a.sort();
    let mut res = Arr3d::new(n + 1, 3, 2, 0);
    res[(0, 0, 0)] = h;
    for i in 0..n {
        let mut can = false;
        for j in 0..3 {
            for k in 0..2 {
                let val = res[(i, j, k)];
                if val > 0 {
                    can = true;
                }
                if j + 1 < 3 {
                    res[(i, j + 1, k)].maxim(2 * val);
                }
                if k + 1 < 2 {
                    res[(i, j, k + 1)].maxim(val * 3);
                }
                if val > a[i] {
                    res[(i + 1, j, k)].maxim(val + a[i] / 2);
                }
            }
        }
        if !can {
            out_line!(i - 1);
            return;
        }
    }
    for j in 0..3 {
        for k in 0..2 {
            if res[(n, j, k)] > 0 {
                out_line!(n);
                return;
            }
        }
    }
    out_line!(n - 1);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
