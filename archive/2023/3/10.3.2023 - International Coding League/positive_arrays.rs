//{"name":"Positive Arrays","group":"CodeChef - INCL2023","url":"https://www.codechef.com/INCL2023/problems/ICL_POSARR","interactive":false,"timeLimit":3000,"tests":[{"input":"69\n","output":"1\n"},{"input":"420\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PositiveArrays"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let k: Str = input.read();

    let mut tail = 0;
    for &c in &k[k.len().max(6) - 6..] {
        tail *= 10;
        tail += (c - b'0').into_i32();
    }
    let mut left = 1;
    let mut right = k.len();
    while left < right {
        let mid = (left + right) / 2;
        let mut cur = (mid * 9).into_i32();
        let mut tail = tail;
        let mut good = true;
        for &c in k[..k.len().max(6) - 6].iter().rev() {
            let c = (c - b'0').into_i32();
            tail += c * 1000000;
            let mut last_tail = tail % 10;
            if last_tail < 0 {
                last_tail += 10;
            }
            let last_cur = cur % 10;
            if last_tail < last_cur {
                cur -= last_cur - last_tail;
            } else if last_tail > last_cur {
                if cur < 10 {
                    good = false;
                    break;
                }
                cur -= last_cur - last_tail + 10;
            }
            tail -= cur;
            tail /= 10;
        }
        while tail > 0 {
            let mut last_tail = tail % 10;
            if last_tail < 0 {
                last_tail += 10;
            }
            let last_cur = cur % 10;
            if last_tail < last_cur {
                cur -= last_cur - last_tail;
            } else if last_tail > last_cur {
                if cur < 10 {
                    good = false;
                    break;
                }
                cur -= last_cur - last_tail + 10;
            }
            tail -= cur;
            tail /= 10;
        }
        if good {
            right = mid;
        } else {
            left = mid + 1;
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
