//{"name":"James and the menus","group":"HackerEarth - January Circuits '22","url":"https://www.hackerearth.com/challenges/competitive/january-circuits-022/algorithm/howie-and-the-menus-2-48359fe4/","interactive":false,"timeLimit":1000,"tests":[{"input":"3 4\n1 2 1 10\n3 2 3 4\n1 3 3 2\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JamesAndTheMenus"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let a = input.read_table::<u64>(n, m);

    let max = (0..m).map(|i| *a.column(i).max().unwrap()).collect_vec();
    let mut ans = None;
    for i in 0..n {
        let same = max.iter().zip(a[i].iter()).filter(|&(a, b)| a == b).count();
        let sum: u64 = a[i].iter().sum();
        ans.maxim((same, sum, i + 1));
    }
    out_line!(ans.unwrap().2);
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
