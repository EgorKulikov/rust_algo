//{"name":"E1. Очередное упражнение на графы (простая версия)","group":"Codeforces - Hello 2025","url":"https://codeforces.com/contest/2057/problem/E1","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n4 4 2\n1 2 2\n2 4 2\n1 3 4\n3 4 1\n1 4 2\n2 3 1\n6 7 3\n1 2 10\n2 3 3\n3 4 9\n4 5 2\n5 6 1\n2 4 10\n4 6 10\n1 6 3\n1 6 2\n2 4 1\n11 17 10\n1 4 5\n1 3 19\n1 2 10\n3 2 13\n4 5 1\n4 6 11\n3 5 9\n3 6 18\n2 7 17\n5 8 15\n5 10 8\n6 9 4\n7 10 20\n7 8 16\n8 11 3\n9 11 6\n10 11 14\n3 11 1\n3 11 3\n1 11 1\n1 11 4\n1 11 3\n8 2 2\n10 4 1\n3 9 2\n3 9 1\n6 7 3\n","output":"1 2\n2 9 9\n11 3 11 1 3 10 8 4 11 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1OcherednoeUprazhnenieNaGrafiProstayaVersiya"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::usize;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let mut edges = input.read_vec::<(usize, usize, i32)>(m).dec();
    let queries = input.read_vec::<(usize, usize, usize)>(q).dec();

    edges.sort_unstable_by_key(|x| x.2);
    let mut dsu = DSU::new(n);
    let mut ans = vec![None; q];
    let mut dist = Arr2d::gen(n, n, |i, j| if i == j { 0 } else { usize::MAX / 2 });
    for (u, v, _) in edges.copy_iter() {
        dist[(u, v)] = 1;
        dist[(v, u)] = 1;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let cand = dist[(i, k)] + dist[(k, j)];
                dist[(i, j)].minim(cand);
            }
        }
    }
    for (u, v, w) in edges.copy_iter() {
        if dsu.union(u, v) {
            for i in 0..n {
                for j in 0..n {
                    let cand = (dist[(i, u)] + dist[(v, j)]).min(dist[(i, v)] + dist[(u, j)]);
                    dist[(i, j)].minim(cand);
                }
            }

            for (i, (u, v, k)) in queries.copy_enumerate() {
                if dist[(u, v)] < k {
                    ans[i].minim(w);
                }
            }
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
