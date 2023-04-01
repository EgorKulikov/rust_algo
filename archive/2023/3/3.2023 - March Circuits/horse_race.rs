//{"name":"Horse Race","group":"HackerEarth - March Circuits '23","url":"https://www.hackerearth.com/challenges/competitive/march-circuits-23/algorithm/horse-race-122f4ccc/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n7 2 2\n1 3 2 2 2 2 1\n5 3 1\n1 1 3 5 5\n4 2 2\n1 2 2 3\n","output":"3\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HorseRace"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::{IncDec, Qty};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let x = input.read_size();
    let a = input.read_size_vec(n).dec_by_one();

    let mut q = a.qty_bound(n).into_iter().take(m).collect_vec();
    q.sort();
    let mut sum = 0;
    let mut ans = 0;
    let mut fin = true;
    for (i, a) in q.into_iter().enumerate() {
        if a * i > sum + x {
            ans = (sum + x) / i;
            fin = false;
            break;
        }
        sum += a;
    }
    if fin {
        ans = (sum + x) / m;
    }
    ans.minim(n / m);
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
