//{"name":"E - Insert or Erase","group":"AtCoder - \tToyota Programming Contest 2024#3（AtCoder Beginner Contest 344）","url":"https://atcoder.jp/contests/abc344/tasks/abc344_e","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 1 4 3\n4\n2 1\n1 4 5\n2 2\n1 5 1\n","output":"4 5 1 3\n"},{"input":"6\n3 1 4 5 9 2\n7\n2 5\n1 3 5\n1 9 7\n2 9\n2 3\n1 2 3\n2 4\n","output":"5 1 7 2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EInsertOrErase"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut value = Vec::with_capacity(n);
    let mut next = vec![Vec::new(); n];
    let mut id = DefaultHashMap::new();
    for (i, a) in a.into_iter().enumerate() {
        value.push(Some(a));
        id[a] = i;
    }

    let q = input.read_size();
    for _ in 0..q {
        let t = input.read_size();
        match t {
            1 => {
                let x = input.read_int();
                let y = input.read_int();
                let at = id[x];
                next.push(Vec::new());
                next[at].push(value.len());
                id[y] = value.len();
                value.push(Some(y));
            }
            2 => {
                let x = input.read_int();
                let at = id[x];
                value[at] = None;
                id.remove(&x);
            }
            _ => unreachable!(),
        }
    }
    let mut ans = Vec::new();
    for i in 0..n {
        let mut rec = RecursiveFunction::new(|rec, at: usize| {
            if let Some(val) = value[at] {
                ans.push(val);
            }
            for &next in next[at].iter().rev() {
                rec.call(next);
            }
        });
        rec.call(i);
    }
    out.print_line(ans);
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
            for i in 1..=t {
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
    //    tester::stress_test();
}
//END MAIN
