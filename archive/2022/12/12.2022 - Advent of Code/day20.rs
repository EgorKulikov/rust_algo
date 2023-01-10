//{"name":"day20","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day20"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let mut a = Vec::new();

    // const MULT: i64 = 1;
    const MULT: i64 = 811589153;

    while !input.is_exhausted() {
        let id = a.len();
        a.push((input.read_long() * MULT, id));
        input.skip_whitespace();
    }

    // for _ in 0..1 {
    for _ in 0..10 {
        for id in 0..a.len() {
            let at = a.iter().position(|&(_, y)| y == id).unwrap();
            let (b, _) = a.remove(at);
            let mut shift = b % a.len().into_i64();
            if shift < 0 {
                shift += a.len().into_i64();
            }
            if at + shift.into_usize() < a.len() {
                a.insert(at + shift.into_usize(), (b, id));
            } else {
                a.insert(at + shift.into_usize() - a.len(), (b, id));
            }
        }
    }
    let pos = a.iter().position(|&(x, _)| x == 0).unwrap();
    let ans =
        a[(pos + 1000) % a.len()].0 + a[(pos + 2000) % a.len()].0 + a[(pos + 3000) % a.len()].0;
    out_line!(ans);
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
