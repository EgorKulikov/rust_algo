//{"name":"S3 - Pretty Pens","group":"DMOJ - CCC '25","url":"https://dmoj.ca/problem/ccc25s3","interactive":false,"timeLimit":7000,"tests":[{"input":"6 3 0\n1 6\n2 9\n3 4\n2 7\n3 9\n1 3\n","output":"25\n"},{"input":"3 2 2\n1 20\n2 30\n1 10\n1 3 2\n2 3 25\n","output":"50\n50\n55\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BTreeSet;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let cp = input.read_size_pair_vec(n);

    let (c, mut p) = cp.detuple();
    let mut c = c.dec();
    let mut best = vec![0; m];
    let mut second = vec![None; m];
    let mut best_set = BTreeSet::new();
    let mut second_set = BTreeSet::new();
    let mut best_sum = 0;
    let mut by_color = vec![BTreeSet::new(); m];
    for i in 0..n {
        if best[c[i]] < p[i] {
            second[c[i]] = Some(best[c[i]]);
            best[c[i]] = p[i];
        } else {
            second[c[i]].maxim(p[i]);
        }
        by_color[c[i]].insert((p[i], i));
    }
    for i in 0..m {
        best_sum += best[i];
        best_set.insert((best[i], i));
        if let Some(second) = second[i] {
            second_set.insert((second, i));
        }
    }

    for id in 0..=q {
        let mut ans = best_sum;
        if let Some(&(val, _)) = second_set.last() {
            if let Some(&(val2, _)) = best_set.first() {
                if val > val2 {
                    ans -= val2;
                    ans += val;
                }
            }
        }
        out.print_line(ans);
        if id == q {
            break;
        }
        let t = input.read_int();
        let id = input.read_size() - 1;
        let oc = c[id];
        best_set.remove(&(best[oc], oc));
        if let Some(second) = second[oc] {
            second_set.remove(&(second, oc));
        }
        best_sum -= best[oc];
        by_color[oc].remove(&(p[id], id));
        match t {
            1 => {
                let cc = input.read_size() - 1;
                c[id] = cc;
                by_color[cc].insert((p[id], id));
                best_set.remove(&(best[cc], cc));
                if let Some(second) = second[cc] {
                    second_set.remove(&(second, cc));
                }
                best_sum -= best[cc];
                best[cc] = by_color[cc].last().unwrap().0;
                best_set.insert((best[cc], cc));
                second[cc] = by_color[cc].iter().rev().nth(1).map(|x| x.0);
                if let Some(second) = second[cc] {
                    second_set.insert((second, cc));
                }
                best_sum += best[cc];
            }
            2 => {
                let pp = input.read_size();
                p[id] = pp;
                by_color[oc].insert((pp, id));
            }
            _ => unreachable!(),
        }
        best[oc] = by_color[oc].last().unwrap().0;
        best_set.insert((best[oc], oc));
        second[oc] = by_color[oc].iter().rev().nth(1).map(|x| x.0);
        if let Some(second) = second[oc] {
            second_set.insert((second, oc));
        }
        best_sum += best[oc];
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
