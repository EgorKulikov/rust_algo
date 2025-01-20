//{"name":"F. Boolean Function Reconstruction","group":"Universal Cup - The 3rd Universal Cup. Stage 26: China","url":"https://contest.ucup.ac/contest/1894/problem/9980","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n2\n0001\n2\n0111\n2\n1111\n3\n00010111\n1\n10\n2\n0101\n5\n00000000000000000000000000000001\n","output":"Yes\n(a&b)\nYes\n(a|b)\nYes\nT\nYes\n((a&(b|c))|(b&c))\nNo\nYes\na\nYes\n(a&(b&(c&(d&e))))\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut base = BitSet::new(1 << n);
    for i in usize::iter_all(n) {
        let mut good = true;
        for j in 0..n {
            if i.is_set(j) && s[i.without_bit(j)] == b'1' {
                good = false;
                break;
            }
        }
        if good && s[i] == b'1' {
            base.set(i);
        } else if !good && s[i] == b'0' {
            out.print_line(false);
            return;
        }
    }

    out.print_line(true);
    if base.count_ones() == 0 {
        out.print_line("F");
        return;
    }
    if base[0] {
        out.print_line("T");
        return;
    }

    enum Expr {
        Token(usize),
        And(Box<Expr>, Box<Expr>),
        Or(Box<Expr>, Box<Expr>),
    }

    impl Expr {
        fn num_op(&self) -> usize {
            match self {
                Expr::Token(_) => 0,
                Expr::And(a, b) => a.num_op() + b.num_op() + 1,
                Expr::Or(a, b) => a.num_op() + b.num_op() + 1,
            }
        }

        fn print(&self, out: &mut Output) {
            match self {
                Expr::Token(pos) => {
                    out.print((b'a' + *pos as u8) as char);
                }
                Expr::And(a, b) => {
                    out.print('(');
                    a.print(out);
                    out.print("&");
                    b.print(out);
                    out.print(')');
                }
                Expr::Or(a, b) => {
                    out.print('(');
                    a.print(out);
                    out.print("|");
                    b.print(out);
                    out.print(')');
                }
            }
        }
    }

    let mut rec = RecursiveFunction3::new(|rec, from: usize, to: usize, pos: usize| -> Expr {
        assert!(!base[from]);
        let mut found = false;
        for i in from..to {
            if base[i] {
                found = true;
                break;
            }
        }
        assert!(found);
        let mid = (from + to) / 2;
        let mut with_pos = None;
        if base[mid] {
            with_pos = Some(Expr::Token(pos));
        } else {
            let mut found = false;
            for i in mid..to {
                if base[i] {
                    found = true;
                    break;
                }
            }
            if found {
                with_pos = Some(Expr::And(
                    Box::new(Expr::Token(pos)),
                    Box::new(rec.call(mid, to, pos - 1)),
                ));
            }
        }
        let mut without_pos = None;
        let mut found = false;
        for i in from..mid {
            if base[i] {
                found = true;
                break;
            }
        }
        if found {
            without_pos = Some(rec.call(from, mid, pos - 1));
        }
        if let Some(with_pos) = with_pos {
            if let Some(without_pos) = without_pos {
                return Expr::Or(Box::new(with_pos), Box::new(without_pos));
            } else {
                return with_pos;
            }
        } else {
            return without_pos.unwrap();
        }
    });
    let ans = rec.call(0, 1 << n, n - 1);
    assert!(ans.num_op() <= (1 << (n - 1)) + 10);
    ans.print(out);
    out.print_line(());
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
