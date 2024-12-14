//{"name":"T533398 众数","group":"Luogu","url":"https://www.luogu.com.cn/problem/T533398?contestId=218363","interactive":false,"timeLimit":1000,"tests":[{"input":"0 4 5\n2 1 3 3 1 1 1 2\n2 0\n1 4 6\n2 6\n1 3 8\n2 7\n","output":"2\n4\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T533398"}}}

use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _t = input.read_size();
    let n = input.read_size();
    let m = input.read_size();
    let pairs = input.read_size_pair_vec(n);

    let (mut a, b) = pairs.detuple();
    let b = b.dec();
    // const BUBEN: usize = 2000;
    let mut poi = Vec::new();
    let mut x = 1;
    while x < n {
        poi.push(n - x);
        x *= 2;
    }
    poi.push(0);
    poi.reverse();
    poi.push(n);
    let mut qty = vec![0; n];
    let mut qties = Vec::new();
    let mut best = Vec::new();
    let mut cur_best = n - 1;
    for i in 0..n {
        if poi.contains(&i) {
            qties.push(qty.clone());
            best.push(cur_best);
        }
        qty[b[i]] += a[i];
        if (qty[b[i]], b[i]) > (qty[cur_best], cur_best) {
            cur_best = b[i];
        }
    }
    // let mut by_qty = IndexedHeap::new(n);
    // for i in 0..n {
    //     if qty[i] != 0 {
    //         by_qty.add_or_adjust(i, Reverse((qty[i], i)));
    //     }
    // }

    // let mut changes = vec![0; n];
    let mut seg = vec![0; n];

    for _ in 0..m {
        let t = input.read_size();
        match t {
            1 => {
                let x = input.read_size() - 1;
                let y = input.read_size();
                qty[b[x]] += y;
                a[x] += y;
                // by_qty.add_or_adjust(b[x], Reverse((qty[b[x]], b[x])));
                for i in qties.indices() {
                    if x < poi[i] {
                        qties[i][b[x]] += y;
                        if (qties[i][b[x]], b[x]) > (qties[i][best[i]], best[i]) {
                            best[i] = b[x];
                        }
                    }
                }
            }
            2 => {
                let q = input.read_size();
                let mut cur = 0;
                // let mut cur = (by_qty.peek().unwrap().1 .0 .1 + 1) * a[n - 1];
                // let mut k = 1;
                // while cur != q {
                //     qty[b[n - k]] -= a[n - k];
                //     by_qty.add_or_adjust(b[n - k], Reverse((qty[b[n - k]], b[n - k])));
                //     changes[b[n - k]] += 1;
                //     k += 1;
                //     cur ^= (by_qty.peek().unwrap().1 .0 .1 + 1) * a[n - k];
                // }
                // out.print_line(k);
                // for i in 1..k {
                //     qty[b[n - i]] += a[n - i];
                //     changes[b[n - i]] -= 1;
                //     if changes[b[n - i]] == 0 {
                //         by_qty.add_or_adjust(b[n - i], Reverse((qty[b[n - i]], b[n - i])));
                //     }
                // }
                for i in qties.indices().rev() {
                    let mut best = best[i];
                    for j in poi[i]..poi[i + 1] {
                        qties[i][b[j]] += a[j];
                        if (qties[i][b[j]], b[j]) > (qties[i][best], best) {
                            best = b[j];
                        }
                        seg[j - poi[i]] = best + 1;
                    }
                    let mut found = false;
                    for j in (poi[i]..poi[i + 1]).rev() {
                        cur ^= seg[j - poi[i]] * a[j];
                        if cur == q {
                            found = true;
                            out.print_line(n - j);
                            break;
                        }
                    }
                    for j in poi[i]..poi[i + 1] {
                        qties[i][b[j]] -= a[j];
                    }
                    if found {
                        break;
                    }
                }
            }
            _ => unreachable!(),
        }
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
