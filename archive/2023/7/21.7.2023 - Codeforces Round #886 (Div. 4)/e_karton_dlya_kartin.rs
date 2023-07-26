//{"name":"E. Картон для картин","group":"Codeforces - Codeforces Round 886 (Div. 4)","url":"https://codeforces.com/contest/1850/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n3 50\n3 2 1\n1 100\n6\n5 500\n2 2 2 2 2\n2 365\n3 4\n2 469077255466389\n10000 2023\n10 635472106413848880\n9181 4243 7777 1859 2017 4397 14 9390 2245 7225\n7 176345687772781240\n9202 9407 9229 6257 7743 5738 7966\n14 865563946464579627\n3654 5483 1657 7571 1639 9815 122 9468 3079 2666 5498 4540 7861 5384\n19 977162053008871403\n9169 9520 9209 9013 9300 9843 9933 9454 9960 9167 9964 9701 9251 9404 9462 9277 9661 9164 9161\n18 886531871815571953\n2609 10 5098 9591 949 8485 6385 4586 1064 5412 6564 8460 2245 6552 5089 8353 3803 3764\n","output":"1\n2\n4\n5\n7654321\n126040443\n79356352\n124321725\n113385729\n110961227\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EKartonDlyaKartin"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut c = input.read_long();
    let s = input.read_long_vec(n);

    c -= s.iter().fold(0, |acc, &x| acc + x * x);
    c /= 4;
    let s = s.iter().sum::<i64>();
    let mut left = 0;
    let mut right = 1;
    let n = n.into_i64();
    while n * right * right + s * right < c {
        left = right;
        right *= 2;
    }
    while left < right {
        let mid = (left + right) / 2;
        if n * mid * mid + s * mid < c {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    out_line!(left);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
