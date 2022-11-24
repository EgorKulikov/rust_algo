//{"name":"Frodo and the mines of Moria","group":"CodeChef - STRV2022","url":"https://www.codechef.com/STRV2022/problems/XOR_PERM","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n5 3\n2 3 8 10 128\n","output":"290\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FrodoAndTheMinesOfMoria"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let a = input.read_long_vec(n);

    let mut ans = 0;
    let mut delta = Vec::with_capacity(n);
    for i in a {
        ans += i;
        if i == 0 {
            delta.push(0);
            continue;
        }
        let enabled = i.count_ones();
        let total = 64 - i.leading_zeros();
        let max = if total % 4 == 0 || enabled == total || 2 * enabled == total {
            (1 << total) - 1
        } else {
            (1 << total) - 4
        };
        // assert!(max >= i);
        delta.push((max - i).max(0));
    }
    delta.sort();
    delta.reverse();
    for i in 0..k {
        ans += delta[i];
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
