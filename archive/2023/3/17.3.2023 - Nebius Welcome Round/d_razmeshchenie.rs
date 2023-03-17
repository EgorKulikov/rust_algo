//{"name":"D. Размещение","group":"Codeforces - Nebius Welcome Round (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1804/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n0100\n1100\n0110\n1010\n1011\n","output":"7 10\n"},{"input":"1 8\n01011100\n","output":"3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRazmeshchenie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();

    let num_singles = m / 2;
    let num_doubles = m / 4;
    let mut min = 0;
    let mut max = 0;
    for _ in 0..n {
        let f: Str = input.read();

        let mut singles = 0;
        let mut doubles = 0;
        let mut i = 0;
        while i < m {
            if f[i] == b'1' {
                if i + 1 < m && f[i + 1] == b'1' {
                    doubles += 1;
                    i += 2;
                } else {
                    singles += 1;
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
        if doubles > num_doubles {
            singles += (doubles - num_doubles) * 2;
            doubles = num_doubles;
        }
        let mut ans = 0;
        let mut last = 0;
        let mut need_singles = 0;
        for i in 0..m {
            if i > 0 && f[i - 1] == b'1' && f[i] == b'1' {
                let len = i - last;
                need_singles += len % 2;
                last = i;
            }
            if f[i] == b'1' {
                ans += 1;
            }
        }
        let len = m - last;
        need_singles += len % 2;
        ans -= (need_singles.max(num_singles) - num_singles + 1) / 2;
        min += singles + doubles;
        max += ans;
    }
    out_line!(min, max);
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
