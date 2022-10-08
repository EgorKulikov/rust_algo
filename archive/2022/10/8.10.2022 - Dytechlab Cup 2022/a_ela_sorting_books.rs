//{"name":"A. Ela Sorting Books","group":"Codeforces - Dytechlab Cup 2022","url":"https://codeforces.com/contest/1737/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n12 3\ncabccadabaac\n12 6\ncabccadabaac\n12 12\ncabccadabaac\n25 1\nabcdefghijklmnopqrstuvwxy\n10 5\nbcdxedbcfg\n","output":"edb\nccbbba\nbbbbbaaaaaaa\nz\naaaaa\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AElaSortingBooks"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::Qty;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let s: Str = input.read();

    let mut q = s.iter().map(u8::into_usize).collect_vec().qty();
    let mut ans = Str::from("");
    for _ in 0..k {
        let mut cur = b'a';
        for _ in 0..n / k {
            if cur.into_usize() < q.len() && q[cur.into_usize()] > 0 {
                q[cur.into_usize()] -= 1;
                cur += 1;
            } else {
                break;
            }
        }
        ans += cur;
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
