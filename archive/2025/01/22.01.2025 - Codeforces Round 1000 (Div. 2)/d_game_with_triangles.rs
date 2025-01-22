//{"name":"D. Game With Triangles","group":"Codeforces - Codeforces Round 1000 (Div. 2)","url":"https://codeforces.com/contest/2063/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 3\n0\n0 1 -1\n2 4\n0 100\n-100 -50 0 50\n2 4\n0 1000\n-100 -50 0 50\n6 6\n20 1 27 100 43 42\n100 84 1 24 22 77\n8 2\n564040265 -509489796 469913620 198872582 -400714529 553177666 131159391 -20796763\n-1000000000 1000000000\n","output":"1\n2\n2\n150 200\n2\n1000 200\n4\n99 198 260 283\n2\n2000000000 2027422256\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n).sorted();
    let b = input.read_long_vec(m).sorted();

    let mut a_s = Vec::with_gen(n / 2, |i| a[Back(i)] - a[i]).do_with(|x| x.reverse());
    let mut b_s = Vec::with_gen(m / 2, |i| b[Back(i)] - b[i]).do_with(|x| x.reverse());

    let mut cur = 0;
    let mut ans = Vec::new();
    let mut a_t = Vec::new();
    let mut b_t = Vec::new();
    'outer: loop {
        if let Some(&a) = a_s.last() {
            if let Some(&b) = b_s.last() {
                if a > b {
                    a_s.pop();
                    cur += a;
                    a_t.push(a);
                } else {
                    b_s.pop();
                    cur += b;
                    b_t.push(b);
                }
            } else {
                a_s.pop();
                cur += a;
                a_t.push(a);
            }
        } else {
            if let Some(&b) = b_s.last() {
                b_s.pop();
                cur += b;
                b_t.push(b);
            } else {
                break;
            }
        }

        while a_t.len() * 2 + b_t.len() > n {
            if a_t.is_empty() {
                break 'outer;
            }
            a_s.clear();
            cur -= a_t.pop().unwrap();
            if let Some(&b) = b_s.last() {
                b_s.pop();
                cur += b;
                b_t.push(b);
            } else {
                break 'outer;
            }
        }
        while b_t.len() * 2 + a_t.len() > m {
            if b_t.is_empty() {
                break 'outer;
            }
            b_s.clear();
            cur -= b_t.pop().unwrap();
            if let Some(&a) = a_s.last() {
                a_s.pop();
                cur += a;
                a_t.push(a);
            } else {
                break 'outer;
            }
        }
        if a_t.len() * 2 + b_t.len() > n || b_t.len() * 2 + a_t.len() > m {
            break;
        }
        ans.push(cur);
    }
    out.print_line(ans.len());
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
