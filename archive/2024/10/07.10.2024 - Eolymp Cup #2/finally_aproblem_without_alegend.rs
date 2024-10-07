//{"name":"Finally a Problem Without a Legend","group":"Eolymp - Basecamp - Eolymp Cup #2","url":"https://basecamp.eolymp.com/en/compete/ptmnufrm6p6nl7gods1loo65go/problem/3","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2 1 2 0 0\n3 3 3 3 1\n1 10 8 8 2\n1\n2 5\n","output":"28\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FinallyAProblemWithoutALegend"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);
    let c = input.read_long_vec(n);

    let mut dist = Arr2d::new(60, 60, None);
    for i in 0..60 {
        dist[(i, i)] = Some(0);
    }
    for i in 0..n {
        for j in 0..60 {
            if b[i].is_set(j) && !a[i].is_set(j) {
                for k in 0..j {
                    if b[i].is_set(k) && !a[i].is_set(k) {
                        dist[(j, k)].minim(2 * c[i]);
                    }
                }
            }
        }
    }
    for j in 0..60 {
        for k in 0..j {
            dist[(k, j)] = dist[(j, k)];
        }
    }
    for i in 0..60 {
        for j in 0..60 {
            for k in 0..60 {
                if let Some(d1) = dist[(j, i)] {
                    if let Some(d2) = dist[(i, k)] {
                        dist[(j, k)].minim(d1 + d2);
                    }
                }
            }
        }
    }

    let answers = Arr2d::generate(10, 10, |i, j| {
        Arr2d::generate(64, 64, |x, y| {
            let mut res = None;
            for a in 0..6 {
                if !x.is_set(a) {
                    continue;
                }
                for b in 0..6 {
                    if !y.is_set(b) {
                        continue;
                    }
                    if let Some(d) = dist[(a + i * 6, b + j * 6)] {
                        res.minim(d);
                    }
                }
            }
            res
        })
    });

    let q = input.read_size();
    for _ in 0..q {
        let u = input.read_size() - 1;
        let v = input.read_size() - 1;
        let mut ans = None;
        for i in 0..10 {
            let x = (b[u] & (!a[u])) >> (i * 6) & 63;
            for j in 0..10 {
                let y = (b[v] & (!a[v])) >> (j * 6) & 63;
                if let Some(d) = answers[(i, j)][(x as usize, y as usize)] {
                    ans.minim(d + c[u] + c[v]);
                }
            }
        }
        out.print_line(ans);
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
