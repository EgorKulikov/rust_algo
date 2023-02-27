//{"name":"A. Прямой эфир","group":"Codeforces - Codeforces Round #854 by cybercats (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1799/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n1 1\n2\n3 2\n5 4\n4 5\n5 9 9 5 7\n5 5\n6 7 8 9 10\n3 4\n4 4 4 4\n4 4\n5 5 6 6\n3 5\n4 5 5 5 4\n4 20\n5 5 24 24 24 5 6 7 8 9 10 12 13 14 15 16 17 18 19 20\n5 7\n7 8 7 11 7 12 10\n6 7\n8 11 7 8 8 8 12\n","output":"1\n-1 2 1\n-1 5 2 1\n5 4 3 2 1\n-1 -1 1\n-1 -1 3 1\n-1 2 1\n8 7 3 1\n7 6 4 2 1\n-1 -1 7 3 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APryamoiEfir"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let p = input.read_size_vec(m).dec_by_one();

    let mut ans = Vec::with_capacity(n);
    let mut was = BitSet::new(m);
    for (at, mut i) in p.into_iter().enumerate() {
        i -= n;
        if !was[i] {
            ans.push((at + 1).into_i32());
            if ans.len() == n {
                break;
            }
            was.set(i, true);
        }
    }
    while ans.len() < n {
        ans.push(-1);
    }
    ans.reverse();
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
