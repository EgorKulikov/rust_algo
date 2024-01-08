//{"name":"D.  Математическое задание","group":"Codeforces - Good Bye 2023","url":"https://codeforces.com/contest/1916/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1\n3\n5\n","output":"1\n169\n196\n961\n16384\n31684\n36481\n38416\n43681\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMatematicheskoeZadanie"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::Str;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let mut s = Str::from(vec![b'0'; n]);
    s[0] = b'1';
    for i in 1..=n / 2 {
        s[i] = b'6';
        s[2 * i] = b'9';
        out.print_line(&s);
        s[i] = b'0';
        s[2 * i] = b'0';
    }
    if n > 1 {
        s[1] = b'9';
        s[2] = b'6';
    }
    out.print_line(&s);
    if n > 1 {
        s[1] = b'0';
        s[2] = b'0';
    }
    s[0] = b'9';
    for i in 1..=n / 2 {
        s[i] = b'6';
        s[2 * i] = b'1';
        out.print_line(&s);
        s[i] = b'0';
        s[2 * i] = b'0';
    }
}

#[test]
fn test() {
    let mut start = 1;
    for n in 2..=5 {
        start *= 10;
        let mut cur = start;
        let mut map = DefaultHashMap::<_, Vec<_>>::new();
        while cur * cur < 10 * start * start {
            let val = cur * cur;
            let mut d = Str::from(val.to_string());
            d.sort();
            map[d].push(cur);
            cur += 1;
        }
        let mut v = map.into_iter().collect_vec();
        v.sort_by_key(|(_, v)| Reverse(v.len()));
        for (_, v) in v {
            if v.len() >= n * 2 - 1 {
                eprintln!("{:?}", v);
            }
        }
    }
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
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test(run, tester::check);
}
//END MAIN
