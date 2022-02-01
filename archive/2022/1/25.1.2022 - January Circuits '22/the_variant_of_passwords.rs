//{"name":"The variant of passwords","group":"HackerEarth - January Circuits '22","url":"https://www.hackerearth.com/challenges/competitive/january-circuits-022/algorithm/stringify-2-304b51ea/","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n6 1 3\n012022\n5 5 0\n02211\n","output":"2\n011122\n4\n00000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TheVariantOfPasswords"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut a = input.read_usize_vec(2);
    let mut s: Str = input.read();

    a.push(n - a[0] - a[1]);
    let mut cnt = [0; 3];
    for i in s.iter() {
        cnt[(i - b'0').into_usize()] += 1;
    }
    let mut ans = 0;
    for i in s.iter_mut() {
        let j = (*i - b'0').into_usize();
        if cnt[j] > a[j] {
            for k in 0..3 {
                if cnt[k] < a[k] {
                    if k < j || a[j] == 0 {
                        cnt[j] -= 1;
                        cnt[k] += 1;
                        *i = k.into_u8() + b'0';
                        ans += 1;
                    }
                    break;
                }
            }
        }
        let j = (*i - b'0').into_usize();
        cnt[j] -= 1;
        a[j] -= 1;
    }
    out_line!(ans);
    out_line!(s);
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
