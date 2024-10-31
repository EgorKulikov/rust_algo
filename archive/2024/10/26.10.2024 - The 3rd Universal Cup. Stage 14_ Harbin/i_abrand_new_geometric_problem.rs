//{"name":"I. A Brand New Geometric Problem","group":"Universal Cup - The 3rd Universal Cup. Stage 14: Harbin","url":"https://contest.ucup.ac/contest/1817/problem/9527","interactive":false,"timeLimit":1000,"tests":[{"input":"2 5 6\n1 2\n","output":"2\n"},{"input":"3 6 5\n1 2 3\n","output":"3\n"},{"input":"2 114514 735134400\n114 514\n","output":"20\n"},{"input":"2 4 7\n1 3\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IABrandNewGeometricProblem"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::primes::factorize::Factorize;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_long();
    let m = input.read_long();
    let a = input.read_long_vec(n);

    let d = m.divisors().sorted();
    let mut edges = Arr2d::new(d.len(), d.len(), d.len());
    for i in d.indices() {
        for j in 0..=i {
            if d[i] % d[j] == 0 {
                edges[(i, j)] = d.lower_bound(&(d[i] / d[j]));
            }
        }
    }
    let mut qty = DefaultHashMap::<_, usize>::new();
    for &a in &a {
        qty[a] += 1;
    }
    #[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
    struct Value(i64);
    impl Default for Value {
        fn default() -> Self {
            Self(i64::MAX)
        }
    }
    let mut dp = vec![DefaultHashMap::<_, Value>::new(); d.len()];
    dp[d.len() - 1][s] = Value(n as i64);
    for i in d.indices().skip(1).rev() {
        let mut times = 0;
        let mut cur = m;
        while cur % d[i] == 0 {
            times += 1;
            cur /= d[i];
        }
        for j in 0..times {
            let delta = if j < qty[d[i]] { -1 } else { 1 };
            for k in d.indices() {
                let to = edges[(k, i)];
                if to != d.len() {
                    let (head, tail) = dp.split_at_mut(k);
                    for (s, v) in tail[0].iter() {
                        if *s >= d[i] {
                            head[to][*s - d[i]].minim(Value(v.0 + delta));
                        }
                    }
                }
            }
        }
    }
    let mut ans = None;
    for (&s, &v) in dp[0].iter() {
        let good = (qty[1] as i64).min(s);
        let bad = s - good;
        ans.minim(v.0 - good + bad);
    }
    out.print_line(ans);
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
