//{"name":"C. Communications","group":"Yandex - SNWS-2022, Round 3","url":"https://contest.yandex.ru/snws2022/contest/23959/problems/C/","interactive":false,"timeLimit":2000,"tests":[{"input":"391997\n1234500\n","output":"3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCommunications"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use algo_lib::string::z_algorithm::ZAlgorithm;

fn solve(input: &mut Input, _test_case: usize) {
    let mut n: Str = input.read();

    let len = n.len();
    n.reverse();
    let mut rev = Str::new();
    let mut carry = 1;
    for c in n.iter() {
        let base = 9 + carry;
        let res = base - (c - b'0');
        if res == 10 {
            rev.push(b'0');
        } else {
            carry = 0;
            rev.push(res + b'0');
        }
    }
    rev += n;
    for _ in 0..len {
        rev.push(b'0');
    }
    let z = rev.z_algorithm();
    let mut nines = vec![0; 2 * len + 1];
    for i in (0..len).rev() {
        if rev[len + i] == b'9' {
            let cur = nines[i + 1] + 1;
            nines[i] = cur;
        } else {
            nines[i] = 0;
        }
    }
    let mut zeros = vec![0; 2 * len + 1];
    for i in (0..len).rev() {
        if rev[len + i] == b'0' {
            let cur = zeros[i + 1] + 1;
            zeros[i] = cur;
        } else {
            zeros[i] = 0;
        }
    }
    let mut ans = 0;
    for i in len + 1..2 * len {
        if z[i] < i - len {
            ans.maxim(z[i]);
        } else {
            if zeros[0] >= i - len {
                ans.maxim(i - len + zeros[2 * (i - len)]);
            } else {
                ans.maxim(i - len + nines[2 * (i - len)]);
            }
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiEof;
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
                input.skip_whitespace();
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
