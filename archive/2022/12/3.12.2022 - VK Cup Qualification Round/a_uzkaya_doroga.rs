//{"name":"A. Узкая дорога","group":"Codeforces - VK Cup 2022 - Квалификация (Engine)","url":"https://codeforces.com/contest/1769/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n20\n30\n50\n100\n","output":"19\n28\n47\n96\n"},{"input":"5\n1\n2\n3\n4\n5\n","output":"0\n1\n2\n3\n4\n"},{"input":"8\n5\n9\n10\n15\n17\n18\n19\n22\n","output":"4\n7\n8\n11\n12\n13\n14\n15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AUzkayaDoroga"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut a = input.read_int_vec(n);

    let mut last = 0;
    for (i, a) in a.iter_mut().enumerate() {
        *a = last.max(*a - 1 - i.into_i32());
        last = *a + 1;
    }
    output().print_per_line(&a);
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
