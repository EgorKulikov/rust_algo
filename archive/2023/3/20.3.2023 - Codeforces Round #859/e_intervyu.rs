//{"name":"E. Интервью","group":"Codeforces - Codeforces Round 859 (Div. 4)","url":"https://codeforces.com/contest/1807/problem/E","interactive":true,"timeLimit":2000,"tests":[{"input":"2\n5\n1 2 3 4 5\n\n11\n\n6\n\n3\n\n7\n1 2 3 5 3 4 2\n\n12\n\n6\n","output":"? 4 1 2 3 4\n\n? 2 2 3\n\n? 1 2\n\n! 2\n\n? 4 2 3 5 6\n\n? 2 1 4\n\n! 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EIntervyu"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut ans = 0;
    for i in 0..20 {
        let mut sum = 0;
        let mut req = Vec::new();
        for (j, &a) in a.iter().enumerate() {
            if j.is_set(i) && (j & usize::all_bits(i)) == ans {
                sum += a;
                req.push(j + 1);
            }
        }
        if req.is_empty() {
            continue;
        }
        out_line!("?", req.len(), req);
        let cur = input.read_int();
        if cur > sum {
            ans.set_bit(i);
        }
    }

    out_line!("!", ans + 1);
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
