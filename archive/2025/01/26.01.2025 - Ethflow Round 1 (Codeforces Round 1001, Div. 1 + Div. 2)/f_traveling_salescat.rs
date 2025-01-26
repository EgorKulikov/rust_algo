//{"name":"F. Traveling Salescat","group":"Codeforces - Ethflow Round 1 (Codeforces Round 1001, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2062/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n0 2\n2 1\n3 3\n5\n2 7\n7 5\n6 3\n1 8\n7 5\n8\n899167687 609615846\n851467150 45726720\n931502759 23784096\n918190644 196992738\n142090421 475722765\n409556751 726971942\n513558832 998277529\n294328304 434714258\n","output":"4 9\n10 22 34 46\n770051069 1655330585 2931719265 3918741472 5033924854 6425541981 7934325514\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::md_arr::arr3d::Arr3d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let ab = input.read_long_pair_vec(n);

    let order = (0..n)
        .collect::<Vec<_>>()
        .sorted_by_key(|&i| ab[i].0 - ab[i].1);

    let cost = Vec::with_gen(n, |i| ab[order[i]].0 + ab[order[i]].1);
    let left_minicost = Vec::with_gen(n, |i| ab[order[i]].0 - ab[order[i]].1);
    let right_minicost = Vec::with_gen(n, |i| ab[order[i]].1 - ab[order[i]].0);

    let mut ans = vec![i64::MAX / 2; n];
    let mut res = Arr3d::new(4, n + 1, n + 1, i64::MAX / 2);
    for i in 0..n {
        res[(0, i + 1, 1)].minim(2 * cost[i] + 2 * right_minicost[i]);
        res[(2, i + 1, 1)].minim(cost[i] + right_minicost[i]);
        for j in 1..=i {
            for left_bonus_used in 0..2 {
                for right_bonus_used in 0..2 {
                    let cur = res[(2 * left_bonus_used + right_bonus_used, i, j)];
                    if left_bonus_used == 1 {
                        if right_bonus_used == 0 {
                            ans[j].minim(cost[i] + left_minicost[i] + cur);
                        } else {
                            ans[j].minim(2 * cost[i] + 2 * left_minicost[i] + cur);
                        }
                    }
                    res[(2 * left_bonus_used + right_bonus_used, i + 1, j)].minim(cur);
                    let cand = cur + 2 * cost[i];
                    res[(2 * left_bonus_used + right_bonus_used, i + 1, j + 1)].minim(cand);
                    if left_bonus_used == 0 {
                        let cand = cur + cost[i] + left_minicost[i];
                        res[(2 + right_bonus_used, i + 1, j + 1)].minim(cand);
                    }
                    if right_bonus_used == 0 {
                        let cand = cur + cost[i] + right_minicost[i];
                        res[(2 * left_bonus_used + 1, i + 1, j + 1)].minim(cand);
                    }
                }
            }
        }
    }
    for i in 1..n {
        assert_eq!(ans[i] % 2, 0);
        ans[i] /= 2;
    }
    out.print_line_iter(ans.iter_skip(1));
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
