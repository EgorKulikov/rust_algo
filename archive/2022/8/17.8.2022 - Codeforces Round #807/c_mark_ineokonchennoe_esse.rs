//{"name":"C. Марк и неоконченное эссе","group":"Codeforces - Codeforces Round #807 (Div. 2)","url":"https://codeforces.com/contest/1705/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n4 3 3\nmark\n1 4\n5 7\n3 8\n1\n10\n12\n7 3 3\ncreamii\n2 3\n3 4\n2 9\n9\n11\n12\n","output":"m\na\nr\ne\na\nr\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMarkINeokonchennoeEsse"}}}

use algo_lib::collections::vec_ext::{Bounds, IncDec};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let c = input.read_usize();
    let q = input.read_usize();
    let s: Str = input.read();
    let segs = input.read_usize_pair_vec(c).dec_by_one();

    let mut cur_len = n;
    let mut pos = Vec::with_capacity(c);
    for &(l, r) in &segs {
        pos.push(cur_len);
        cur_len += r - l + 1;
    }
    for _ in 0..q {
        let mut cur = input.read_usize() - 1;
        while cur >= n {
            let p = pos.upper_bound(&cur) - 1;
            cur = segs[p].0 + cur - pos[p];
        }
        out_line!(s[cur] as char);
    }
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
