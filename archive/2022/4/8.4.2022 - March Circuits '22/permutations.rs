//{"name":"Permutations","group":"HackerEarth - March Circuits '22","url":"https://www.hackerearth.com/de/challenges/competitive/march-circuits-22/algorithm/nevedle-and-permutations-c6acb85e/","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n2 3 1 5 4\n1 2\n2 4\n2 5\n","output":"5\n4\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Permutations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let q = input.read_usize();
    let a = input.read_usize_vec(n);

    let mut from_start = Vec::with_capacity(n + 1);
    from_start.push(0);
    for &i in &a {
        from_start.push((*from_start.last().unwrap()).max(i));
    }
    let mut from_end = Vec::with_capacity(n + 1);
    from_end.push(0);
    for &i in a.iter().rev() {
        from_end.push((*from_end.last().unwrap()).max(i));
    }
    from_end.reverse();

    for _ in 0..q {
        let l = input.read_usize() - 1;
        let r = input.read_usize();
        out_line!(from_start[l].max(from_end[r]));
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
