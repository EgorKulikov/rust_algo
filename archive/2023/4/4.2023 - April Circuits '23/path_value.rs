//{"name":"Path Value","group":"HackerEarth - April Circuits '23","url":"https://www.hackerearth.com/challenges/competitive/april-circuits-23/algorithm/path-value-2-54ac4ca3/","interactive":false,"timeLimit":1000,"tests":[{"input":"5 7\n2 5\n1 2\n2 3\n3 4\n4 5\n1 3\n1 4\n1 5\n20 23 21 45 21\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PathValue"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let s = input.read_size() - 1;
    let e = input.read_size() - 1;
    let mut edges = input.read_size_pair_vec(m).dec_by_one();
    let a = input.read_int_vec(n);

    edges.sort_by_key(|&(u, v)| (a[u] - a[v]).abs());
    let mut dsu = DSU::new(n);
    for (u, v) in edges {
        dsu.join(u, v);
        if dsu.get(s) == dsu.get(e) {
            out_line!((a[u] - a[v]).abs());
            return;
        }
    }
    unreachable!();
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
