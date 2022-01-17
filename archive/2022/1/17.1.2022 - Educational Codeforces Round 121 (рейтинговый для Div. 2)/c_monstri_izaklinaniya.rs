//{"name":"C. Монстры и заклинания","group":"Codeforces - Educational Codeforces Round 121 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1626/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1\n6\n4\n2\n4 5\n2 2\n3\n5 7 9\n2 1 2\n","output":"10\n6\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMonstriIZaklinaniya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_long_vec(n);
    let mut h = input.read_long_vec(n);

    for i in (0..n).rev() {
        for j in i + 1..n {
            if h[j] - h[i] > k[j] - k[i] {
                h[i] += h[j] - h[i] - (k[j] - k[i]);
            }
        }
    }
    let mut last = 0;
    let mut damage = 0;
    let mut answer = 0;
    for (k, h) in k.into_iter().zip(h.into_iter()) {
        if h <= k - last {
            answer += h * (h + 1) / 2;
            damage = h;
        } else {
            answer -= damage * (damage + 1) / 2;
            damage += k - last;
            answer += damage * (damage + 1) / 2;
        }
        last = k;
    }
    out_line!(answer);
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
