//{"name":"Non-Primes 101","group":"CodeChef - START147A","url":"https://www.codechef.com/START147A/problems/NONPRIME101","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n3 6\n4\n1 2 3 4\n3\n1 1 2\n3\n3 2 3\n","output":"1 2\n1 3\n-1\n3 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NonPrimes101"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::primes::sieve::primality_table;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let is_prime = primality_table(201);
    let mut qty = DefaultHashMap::<_, Vec<usize>>::new();
    for (id, i) in a.into_iter().enumerate() {
        qty[i].push(id + 1);
    }
    let qty = qty.into_iter().collect_vec();
    for i in qty.indices() {
        let (a, q) = &qty[i];
        if q.len() >= 2 && !is_prime[2 * *a] {
            out.print_line(&q[..2]);
            return;
        }
        for j in 0..i {
            let (b, r) = &qty[j];
            if !is_prime[*a + *b] {
                out.print_line((q[0], r[0]));
                return;
            }
        }
    }
    out.print_line(-1);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
