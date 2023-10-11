//{"name":"B. Digit occurrence Sum","group":"Codeforces - Replay of Ostad Presents Intra KUET Programming Contest 2023","url":"https://codeforces.com/gym/104663/problem/B","interactive":false,"timeLimit":1500,"tests":[{"input":"4 6\n70 123 311 125\n? 123\n- 2\n? 123\n+ 234\n- 3\n? 123\n","output":"8\n6\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDigitOccurrenceSum"}}}

use algo_lib::collections::treap_map::TreapSet;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n).sorted();

    let mut count = [0; 10];
    for &x in &a {
        let mut x = x;
        while x > 0 {
            count[x % 10] += 1;
            x /= 10;
        }
    }
    let mut a = unsafe { TreapSet::from_sorted(a.into_iter()) };

    for _ in 0..q {
        let op = input.read_char();
        let x = input.read_size();
        match op {
            '?' => {
                if !a.contains(&x) {
                    out.print_line(-1);
                    continue;
                }
                let mut ans = 0;
                let mut x = x;
                while x > 0 {
                    ans += count[x % 10];
                    x /= 10;
                }
                out.print_line(ans);
            }
            '+' => {
                if !a.contains(&x) {
                    a.insert(x);
                    let mut x = x;
                    while x > 0 {
                        count[x % 10] += 1;
                        x /= 10;
                    }
                } else {
                    a.remove(&x);
                    let mut x = x;
                    while x > 0 {
                        count[x % 10] -= 1;
                        x /= 10;
                    }
                }
            }
            '-' => {
                if a.len() >= x {
                    let last = *a.get_at(a.len() - x).unwrap();
                    a.remove(&last);
                    let mut last = last;
                    while last > 0 {
                        count[last % 10] -= 1;
                        last /= 10;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
