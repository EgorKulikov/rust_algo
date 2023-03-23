//{"name":"Play with subarray","group":"CodeChef - CDVN2023","url":"https://www.codechef.com/CDVN2023/problems/PLAYSUB","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n12\n-1 -1 1 -1 1 -1 1 1 -1 -1 1 -1\n12\n-1 1 1 -1 -1 1 -1 -1 1 -1 1 -1\n","output":"8\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PlayWithSubarray"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut ricky = 0;
    let mut sum = 0;
    let mut ways = DefaultHashMap::<i32, i64>::new();
    let mut ricky_delta = Vec::new();
    for &i in a.iter().rev() {
        let cur = if i > 0 {
            ways[sum] += 1;
            ways[sum + i]
        } else {
            0
        };
        sum += i;
        ricky += cur;
        ricky_delta.push(cur);
    }
    ricky_delta.reverse();
    let mut vicky = 0;
    ways.clear();
    sum = 0;
    for (i, a) in a.into_iter().enumerate() {
        ricky -= ricky_delta[i];
        if a < 0 {
            ways[sum] += 1;
            vicky += ways[sum + a];
        }
        sum += a;
        if vicky > ricky {
            out_line!(i);
            return;
        }
    }
    out_line!(-1);
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
