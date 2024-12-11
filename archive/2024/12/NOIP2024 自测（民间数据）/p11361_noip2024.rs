//{"name":"P11361 [NOIP2024] 编辑字符串（民间数据）","group":"Luogu","url":"https://www.luogu.com.cn/problem/P11361?contestId=217331","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n6\n011101\n111010\n111010\n101101\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P11361NOIP2024"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str_vec(2);
    let t = input.read_str_vec(2);

    let mut segments = Vec::new();
    for i in 0..2 {
        let mut cur = Vec::new();
        let mut zeroes = 0;
        let mut ones = 0;
        for j in 0..n {
            if t[i][j] == b'0' {
                if zeroes + ones != 0 {
                    cur.push((zeroes, ones));
                    zeroes = 0;
                    ones = 0;
                }
                if s[i][j] == b'0' {
                    cur.push((1, 0));
                } else {
                    cur.push((0, 1));
                }
            } else {
                if s[i][j] == b'0' {
                    zeroes += 1;
                } else {
                    ones += 1;
                }
            }
        }
        if zeroes + ones != 0 {
            cur.push((zeroes, ones));
        }
        segments.push(cur);
    }
    let mut ans = 0;
    let mut cur0 = segments[0][0];
    let mut cur1 = segments[1][0];
    let mut at0 = 1;
    let mut at1 = 1;
    let mut end0 = cur0.0 + cur0.1;
    let mut end1 = cur1.0 + cur1.1;
    let mut last = 0;
    for i in 1..=n {
        if end0 == i {
            let len = i - last;
            last = i;
            let m0 = cur0.0.min(cur1.0).min(len);
            ans += m0;
            cur0.0 -= m0;
            cur1.0 -= m0;
            let m1 = cur0.1.min(cur1.1).min(len);
            ans += m1;
            cur0.1 -= m1;
            cur1.1 -= m1;
            if i != n {
                cur0 = segments[0][at0];
                at0 += 1;
                end0 += cur0.0 + cur0.1;
                if end1 == i {
                    cur1 = segments[1][at1];
                    at1 += 1;
                    end1 += cur1.0 + cur1.1;
                }
            }
        }
        if end1 == i {
            let len = i - last;
            last = i;
            let m0 = cur0.0.min(cur1.0).min(len);
            ans += m0;
            cur0.0 -= m0;
            cur1.0 -= m0;
            let m1 = cur0.1.min(cur1.1).min(len);
            ans += m1;
            cur0.1 -= m1;
            cur1.1 -= m1;
            if i != n {
                cur1 = segments[1][at1];
                at1 += 1;
                end1 += cur1.0 + cur1.1;
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
