//{"name":"T509059 「GFOI Round 2」Colors","group":"Luogu","url":"https://www.luogu.com.cn/problem/T509059?contestId=210735","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n3 2 1\n5\n1 3 5 2 4\n5\n5 4 3 1 2\n9\n4 7 3 6 1 2 5 8 9\n","output":"101\n11111\n11101\n111111101\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"T509059GFOIRound2Colors"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();

    let mut ans = Str::from(vec![b'1'; n]);
    let mut max = 0;
    let mut min = n;
    for i in 0..n {
        if i % 2 == 1 {
            if max == i - 1 && p[i] == i || min == n - i && p[i] == n - i - 1 {
                ans[p[i]] = b'0';
            }
        } else {
            if i == 2 && ((p[i] < p[0]) ^ (p[i] < p[1])) {
                if p[1] < p[2] {
                    let mut found = false;
                    for j in 3..n {
                        if p[j] < p[2] {
                            for k in j + 1..n {
                                if p[k] > p[2] && (k != j + 1 || (n - k) % 2 == 1) {
                                    found = true;
                                    break;
                                }
                            }
                            break;
                        }
                    }
                    if !found {
                        ans[p[i]] = b'0';
                    }
                } else {
                    let mut found = false;
                    for j in 3..n {
                        if p[j] > p[2] {
                            for k in j + 1..n {
                                if p[k] < p[2] && (k != j + 1 || (n - k) % 2 == 1) {
                                    found = true;
                                    break;
                                }
                            }
                            break;
                        }
                    }
                    if !found {
                        ans[p[i]] = b'0';
                    }
                }
            } else if i + 3 == n && ((p[i] < p[n - 1]) ^ (p[i] < p[n - 2])) {
                if p[n - 2] < p[n - 3] {
                    let mut found = false;
                    for j in (0..n - 3).rev() {
                        if p[j] < p[n - 3] {
                            for k in (0..j).rev() {
                                if p[k] > p[n - 3] && (k != j - 1 || k % 2 == 0) {
                                    found = true;
                                    break;
                                }
                            }
                            break;
                        }
                    }
                    if !found {
                        ans[p[i]] = b'0';
                    }
                } else {
                    let mut found = false;
                    for j in (0..n - 3).rev() {
                        if p[j] > p[n - 3] {
                            for k in (0..j).rev() {
                                if p[k] < p[n - 3] && (k != j - 1 || k % 2 == 0) {
                                    found = true;
                                    break;
                                }
                            }
                            break;
                        }
                    }
                    if !found {
                        ans[p[i]] = b'0';
                    }
                }
            }
        }
        max.maxim(p[i]);
        min.minim(p[i]);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
