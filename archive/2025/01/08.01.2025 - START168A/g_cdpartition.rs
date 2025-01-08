//{"name":"GCD Partition","group":"CodeChef - START168A","url":"https://www.codechef.com/START168A/problems/GCDPAR","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n3\n1 2 4\n5\n1 1 1 1 1\n","output":"5\n15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GCDPartition"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Reverse;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut steps: Vec<(i32, usize)> = Vec::new();
    let mut positions = DefaultHashMap::new(Vec::new());
    let mut values = DefaultHashMap::new(Vec::new());
    type Mod = ModInt7;
    let mut ans = Mod::zero();
    for i in (0..n).rev() {
        for j in steps.indices() {
            steps[j].0 = gcd(steps[j].0, a[i]);
        }
        steps.push((a[i], i + 1));
        let mut j = 0;
        while j + 1 < steps.len() {
            if steps[j].0 == steps[j + 1].0 {
                steps.remove(j);
            } else {
                j += 1;
            }
        }
        let mut to = n + 1;
        for (val, p) in steps.copy_iter() {
            let pos1 = positions[val + 1].upper_bound(&Reverse(p));
            let pos2 = positions[val + 1].upper_bound(&Reverse(to));
            let v1 = if pos1 == 0 {
                Mod::zero()
            } else {
                values[val + 1][pos1 - 1]
            };
            let v2 = if pos2 == 0 {
                Mod::zero()
            } else {
                values[val + 1][pos2 - 1]
            };
            let v = v1 - v2 + Mod::from_index(to - p);
            let vv = *values[val].last().unwrap_or(&Mod::zero());
            if val == 1 {
                ans += v;
            }
            values[val].push(vv + v);
            positions[val].push(Reverse(i));
            to = p;
        }
    }
    out.print_line(ans);
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
