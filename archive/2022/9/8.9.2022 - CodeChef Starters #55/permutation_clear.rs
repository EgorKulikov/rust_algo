//{"name":"Permutation Clear","group":"CodeChef - START55A","url":"https://www.codechef.com/START55A/problems-old/PERMCLEAR","interactive":false,"timeLimit":500,"tests":[{"input":"3\n4\n4 1 3 2\n2\n3 1\n9\n5 2 9 1 8 6 4 3 7\n3\n5 8 9\n5\n3 4 5 1 2\n2\n2 3\n","output":"4 2\n2 1 6 4 3 7\n4 5 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PermutationClear"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);
    let k = input.read_usize();
    let b = input.read_usize_vec(k);

    let mut to_remove = BitSet::new(n + 1);
    for x in b {
        to_remove.set(x, true);
    }
    let mut ans = Vec::with_capacity(n - k);
    for x in a {
        if !to_remove[x] {
            ans.push(x);
        }
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
