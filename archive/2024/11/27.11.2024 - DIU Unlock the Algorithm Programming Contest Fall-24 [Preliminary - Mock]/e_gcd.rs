//{"name":"E. GCD","group":"Toph","url":"https://toph.co/arena?contest=diu-unlock-the-algorithm-fall-24-preliminary-mock#!/p/6745eb78cb14b2a6cffe5562","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n2 6 4 9 6 12\n5\n2\n3\n1\n6\n4\n","output":"6\n3\n12\n0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EGCD"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::primes::factorize::all_divisors;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let d = all_divisors::<usize>(100_001, true);
    let mut v = DefaultHashMap::<_, Vec<_>>::new();
    for a in a.copy_iter() {
        for d in d[a].copy_iter() {
            v[d].push(a);
        }
    }
    let mut ans = vec![0; n + 1];
    for (k, v) in v {
        let mut val = Vec::new();
        for i in v {
            let pos = val.lower_bound(&i);
            if pos == val.len() {
                val.push(i);
            } else {
                val[pos] = i;
            }
        }
        ans[val.len()].maxim(k);
    }
    for i in (0..n).rev() {
        let cand = ans[i + 1];
        ans[i].maxim(cand);
    }

    let q = input.read_size();
    for _ in 0..q {
        out.print_line(ans[input.read_size()]);
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
