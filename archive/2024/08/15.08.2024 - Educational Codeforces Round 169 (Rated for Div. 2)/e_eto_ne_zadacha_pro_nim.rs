//{"name":"E. Это не задача про ним","group":"Codeforces - Educational Codeforces Round 169 (Rated for Div. 2)","url":"https://codeforces.com/contest/2004/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n3 2 9\n4\n3 3 6 1\n5\n1 2 3 4 5\n","output":"Bob\nAlice\nBob\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EEtoNeZadachaProNim"}}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::primes::sieve::primes;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let p = primes::<usize>(10_000_000);
    let mut smallest = vec![0; 10_000_001];

    for &p in p.iter().rev() {
        for i in (p..=10_000_000).step_by(p) {
            smallest[i] = p;
        }
    }

    for i in 2..=10_000_000 {
        smallest[i] = p.lower_bound(&smallest[i]);
        if smallest[i] > 0 {
            smallest[i] += 1;
        }
    }
    smallest[1] = 1;

    let t = input.read_size();

    for _ in 0..t {
        let n = input.read_size();
        let a = input.read_size_vec(n);

        let mut res = 0;
        for i in a {
            res ^= smallest[i];
        }
        out.print_line(if res == 0 { "Bob" } else { "Alice" });
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
