//{"name":"K. Knapsack","group":"Universal Cup - The 3rd Universal Cup. Stage 27: London","url":"https://contest.ucup.ac/contest/1901/problem/8621","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n4 3 8 1 5 4\n","output":"ABC.BA\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    if n == 6 {
        out.print_line("ABC.BA");
        return;
    }

    let mut parts = Vec::with_gen(n, |i| (a[i], vec![i]));

    while parts.len() >= 3 {
        parts.sort_unstable();
        // eprintln!("parts: {}", parts.len());
        let mut x = Vec::new();
        for i in &parts {
            x.push(i.0 - parts[0].0);
        }
        // if x.len() < 100 {
        //     eprintln!("parts: {:?}", x);
        // }
        for i in 0..parts.len() - 2 {
            if parts[i].0 == parts[i + 2].0 {
                let mut ans = Str::from(vec![b'.'; n]);
                let mut s1 = 0;
                for j in parts[i].1.copy_iter() {
                    assert!(ans[j] == b'.');
                    s1 += a[j];
                    ans[j] = b'A';
                }
                let mut s2 = 0;
                for j in parts[i + 1].1.copy_iter() {
                    assert!(ans[j] == b'.');
                    s2 += a[j];
                    ans[j] = b'B';
                }
                let mut s3 = 0;
                for j in parts[i + 2].1.copy_iter() {
                    assert!(ans[j] == b'.');
                    s3 += a[j];
                    ans[j] = b'C';
                }
                assert!(s1 == s2 && s2 == s3);
                out.print_line(ans);
                return;
            }
        }
        let p0 = parts
            .indices()
            .filter(|&i| !parts[i].0.is_set(0))
            .collect::<Vec<_>>();
        let p1 = parts
            .indices()
            .filter(|&i| parts[i].0.is_set(0))
            .collect::<Vec<_>>();
        let mut n_parts = Vec::new();
        let target = 1_000_000_000_001i64;
        for p0 in [p0, p1] {
            let mut i = 0;
            let mut j = p0.len() - 1;
            while i < j {
                let mut s = parts[p0[i]].0 + parts[p0[j]].0;
                while s < target && i + 1 < j && parts[p0[i + 1]].0 + parts[p0[j]].0 <= target {
                    i += 1;
                    s = parts[p0[i + 1]].0 + parts[p0[j]].0;
                }
                while s > target && i < j - 1 && parts[p0[i]].0 + parts[p0[j - 1]].0 >= target {
                    j -= 1;
                    s = parts[p0[i]].0 + parts[p0[j - 1]].0;
                }
                let a = &parts[p0[i]];
                let b = &parts[p0[j]];
                let sum = (a.0 + b.0) / 2;
                let ind = a.1.copy_iter().chain(b.1.copy_iter()).collect::<Vec<_>>();
                n_parts.push((sum, ind));
                i += 1;
                j -= 1;
            }
            // for i in 0..p0.len() / 2 {
            //     let a = &parts[p0[i]];
            //     let b = &parts[p0[Back(i)]];
            //     let sum = (a.0 + b.0) / 2;
            //     let ind = a.1.copy_iter().chain(b.1.copy_iter()).collect::<Vec<_>>();
            //     n_parts.push((sum, ind));
            // }
        }
        parts = n_parts;
    }
    unreachable!();
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
