//{"name":"E. Двухбуквенные строки","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n6\nab\ncb\ndb\naa\ncc\nef\n7\naa\nbb\ncc\nac\nca\nbb\naa\n4\nkk\nkk\nab\nab\n5\njf\njf\njk\njk\njk\n","output":"5\n6\n0\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EDvukhbukvennieStroki"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s: Vec<Str> = input.read_vec(n);

    let mut qty_left = vec![0i64; 11];
    let mut qty_right = vec![0i64; 11];
    let mut qty = Arr2d::new(11, 11, 0i64);
    let mut ans = 0;
    for s in s {
        let left = (s[0] - b'a').into_usize();
        let right = (s[1] - b'a').into_usize();
        ans += qty_left[left] + qty_right[right] - 2 * qty[(left, right)];
        qty_left[left] += 1;
        qty_right[right] += 1;
        qty[(left, right)] += 1;
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
