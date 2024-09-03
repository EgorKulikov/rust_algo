//{"name":"uc8_a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"uc8_a"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::inf_int::InfInt;
use algo_lib::string::str::StrReader;
use algo_lib::value;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut n = input
        .read_str()
        .into_iter()
        .map(|c| (c - b'0') as i64)
        .collect::<Vec<_>>();
    let m = input.read_long();

    n.reverse();
    let mut carry = 1;
    for i in 0..n.len() {
        n[i] += carry;
        if n[i] == 10 {
            n[i] = 0;
        } else {
            carry = 0;
            break;
        }
    }
    if carry != 0 {
        n.push(carry);
    }
    // let mut digs = 0;
    // let mut mc = m;
    // while mc > 0 {
    //     digs += 1;
    //     mc /= 10;
    // }
    let mut rem = 0;
    for &d in n.iter().rev() {
        rem *= 10;
        rem += d;
        rem %= m;
    }
    value!(Limit: i64 = 1_000_000_000);
    type Int = InfInt<i64, Limit>;
    let mut ans = Int::new((m - rem) % m);
    let mut plus_one_cost = Int::new(1);
    let mut ten = Int::new(1);
    for i in 0..=n.len() {
        let mut mc = m;
        let mut j = i;
        let mut cost = Int::new(0);
        let mut carry = 0;
        let mut first = true;
        let mut c_ten = ten;
        let mut bonus = Int::new(0);
        while mc > 0 {
            let d = mc % 10;
            let mut nn = *n.get(j).unwrap_or(&0) + carry;
            if d != nn {
                if first {
                    first = false;
                    cost += plus_one_cost + bonus;
                    nn += 1;
                }
            }
            if d >= nn {
                cost += c_ten * Int::new(d - nn);
                carry = 0;
            } else {
                cost += c_ten * Int::new(10 + d - nn);
                carry = 1;
            }
            mc /= 10;
            j += 1;
            bonus += c_ten * Int::new(9);
            c_ten *= Int::new(10);
        }
        ans.minim(cost);
        plus_one_cost += ten * Int::new(9 - n.get(i).unwrap_or(&0));
        ten *= Int::new(10);
    }
    let mut ans = ans.n;
    for i in n.indices() {
        n[i] += ans;
        ans = n[i] / 10;
        n[i] %= 10;
    }
    while ans > 0 {
        n.push(ans % 10);
        ans /= 10;
    }
    for &d in n.iter().rev() {
        out.print(d);
    }
    out.print_line(());
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
