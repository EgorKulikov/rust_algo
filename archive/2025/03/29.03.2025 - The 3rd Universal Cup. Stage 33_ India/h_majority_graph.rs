//{"name":"H. Majority Graph","group":"Universal Cup - The 3rd Universal Cup. Stage 33: India","url":"https://contest.ucup.ac/contest/1954/problem/10272","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4\n1 2 1 2\n5\n1 2 3 2 1\n2\n1 1\n3\n2 2 1\n","output":"2\n4\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::by_index;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let by_id = by_index(&a);
    let mut dsu = DSU::new(n);
    for v in by_id.into_values() {
        let mut segs = Vec::new();
        let mut bal = 0;
        let mut last = 0;
        let mut start = n;
        let mut q = 0;
        let mut pos = 0;
        let mut last_end = 0;
        for j in v.indices() {
            let i = v[j];
            if start == n {
                start = i;
                pos = j;
                last = i;
                bal = 0;
                q = 0;
                last_end = i;
            }
            while last + 1 < i && bal > 1 {
                dsu.union(start, last + 1);
                bal -= 1;
                last += 1;
            }
            if last + 1 < i {
                segs.push((start, last, pos, q));
                start = i;
                bal = 0;
                q = 0;
            }
            bal += 1;
            q += 1;
            last = i;
            last_end.maxim(i);
            dsu.union(start, last);
            while start > 0 && q >= (i - start + 1) - q {
                let c_bal = q - (i - start + 1 - q);
                if c_bal >= 2 || pos > 0 && v[pos - 1] == start - 1 {
                    if pos > 0 && v[pos - 1] == start - 1 {
                        pos -= 1;
                    }
                    start -= 1;
                    dsu.union(start, last);
                } else {
                    break;
                }
            }
            let end = (last + bal - 1).min(n - 1);
            for j in last_end + 1..=end {
                dsu.union(start, j);
            }
            last_end.maxim(end);
            while let Some(&(l, r, was_pos, was_q)) = segs.last() {
                if r + 2 == start {
                    dsu.union(r, start);
                }
                if r + 1 == start {
                    start = l;
                    q += was_q;
                    pos = was_pos;
                    segs.pop();
                    dsu.union(start, last);
                    let end = (last + bal - 1).min(n - 1);
                    for j in last_end + 1..=end {
                        dsu.union(start, j);
                    }
                    last_end.maxim(end);
                    while start > 0 && q >= (i - start + 1) - q {
                        let c_bal = q - (i - start + 1 - q);
                        if c_bal >= 2 || pos > 0 && v[pos - 1] == start - 1 {
                            if pos > 0 && v[pos - 1] == start - 1 {
                                pos -= 1;
                            }
                            start -= 1;
                            dsu.union(start, last);
                        } else {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
        while last + 1 < n && bal > 1 {
            dsu.union(start, last + 1);
            bal -= 1;
            last += 1;
        }
    }
    out.print_line(dsu.set_count());
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
