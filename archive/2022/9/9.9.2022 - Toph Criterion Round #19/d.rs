//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let x = input.read_usize();
    let y = input.read_usize();

    let end = (x..=y).fold(usize::MAX, |acc, x| acc & x);
    let mut len = 1;
    while len <= y {
        len *= 2;
    }
    let mut ways = vec![0u128; len];
    ways[len - 1] = 1;
    let mut n_ways = vec![0u128; len];
    let mut ans = Vec::new();
    for _ in 1.. {
        n_ways.fill(0);
        for j in 0..len {
            for k in x..=y {
                n_ways[j & k] += ways[j];
            }
        }
        let (max, at) = n_ways
            .iter()
            .enumerate()
            .map(|(i, &j)| (j, i))
            .max()
            .unwrap();
        if at == end {
            break;
        }
        if max >= u128::MAX / (y * y).into_u128() {
            for i in &mut n_ways {
                *i /= (y * y).into_u128();
            }
        }
        ans.push(at);
        swap(&mut ways, &mut n_ways);
    }

    let q = input.read_usize();
    for _ in 0..q {
        let n = input.read_usize();
        if n <= ans.len() {
            out_line!(ans[n - 1]);
        } else {
            out_line!(end);
        }
    }
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
