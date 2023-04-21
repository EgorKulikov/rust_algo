//{"name":"Lovely Permutation","group":"CodeChef - PRAD2023","url":"https://www.codechef.com/PRAD2023/problems/LOV_PER","interactive":false,"timeLimit":1000,"tests":[{"input":"4 1\n2 1 0 3\n","output":"4\n"},{"input":"4 1\n2 1 3 0\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LovelyPermutation"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::sign::Unsigned;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let k = input.read_size();
    let p = input.read_size_vec(n);

    let mut ans = Vec::with_capacity(1 << n);
    ans.push(Some(0));
    for i in 1usize..(1 << n) {
        let pos = (i.count_ones() - 1).into_usize();
        let from = pos.saturating_sub(k);
        let to = (pos + k).min(n - 1);
        let mut res = None;
        for j in from..=to {
            if i.is_set(j) {
                if let Some(x) = ans[i.without_bit(j)] {
                    res.minim(x + j.distance(p[pos]));
                }
            }
        }
        ans.push(res);
    }
    out_line!(ans.last().copied());
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
