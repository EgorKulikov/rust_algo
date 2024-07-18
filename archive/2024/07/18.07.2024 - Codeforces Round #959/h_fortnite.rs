//{"name":"H. Fortnite","group":"Codeforces - Codeforces Round 959 при поддержке NEAR (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1994/problem/H","interactive":true,"timeLimit":1000,"tests":[{"input":"1\n\n32\n\n28\n","output":"? aa\n\n? yb\n\n! 31 59\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HFortnite"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::gcd::gcd;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    out.print_line("? aa");
    out.flush();
    let p = input.read_long() - 1;
    let mut requests = Vec::with_capacity(3);
    // let rand = random();
    let mut request = |v: &Vec<i64>| {
        let mut s = Str::with_capacity(v.len());
        for &c in v {
            s.push((c - 1) as u8 + b'a');
        }
        out.print_line(('?', s));
        out.flush();
        let result = input.read_long();
        requests.push((v.clone(), result));
        result
    };
    let mut cur = Vec::with_capacity(10);
    for _ in 0..10 {
        let c = 25;
        cur.push(c + 1);
    }
    let rem = request(&cur);
    let mut pp = 1;
    for i in 0..10 {
        if pp > rem {
            cur[i] -= 1;
            request(&cur);
            break;
        }
        if 25 * pp > rem {
            cur[i] -= rem / pp + 1;
            request(&cur);
            break;
        }
        pp *= p;
    }
    let mut m = 0;
    for (cur, result) in &requests {
        let mut sum = 0;
        let mut pp = 1;
        for &i in cur {
            sum += i * pp;
            pp *= p;
        }
        m = gcd(m, sum - *result);
    }
    out.print_line(('!', p, m));
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    // input.skip_whitespace();
    // input.peek().is_none()
    true
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
