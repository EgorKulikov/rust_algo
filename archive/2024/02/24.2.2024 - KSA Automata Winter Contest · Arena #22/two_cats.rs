//{"name":"Two Cats","group":"Baekjoon Online Judge","url":"https://www.acmicpc.net/contest/problem/1238/6","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n1 2 4 9\n","output":"YES\n1 1 2 3 4 5 0\n"},{"input":"3\n1 2 1 2\n","output":"YES\n"},{"input":"10\n1 1 5 6\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TwoCats"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::{HashSet, VecDeque};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_size();
    let mut b = input.read_size();
    let c = input.read_size();
    let d = input.read_size();

    if a == c && b == d {
        out.print_line(true);
        return;
    }
    if a == 1 && b == 1 || a == n - 1 && b == n - 1 || c == 1 && d == 1 || c == n - 1 && d == n - 1
    {
        out.print_line(false);
        return;
    }
    if n == 3 {
        out.print_line(true);
        if a == 1 {
            out.print_line(3);
        } else {
            out.print_line(0);
        }
        return;
    }
    let mut ans = Vec::new();
    while a.abs_diff(b) > 1 {
        if a > b {
            ans.push(0);
            a -= 1;
            b += 1;
        } else {
            ans.push(n);
            a += 1;
            b -= 1;
        }
    }
    if a + 2 < c {
        if a >= b {
            ans.push(0);
            a -= 1;
            b += 1;
        }
        if b == a + 1 {
            ans.push(a);
            b += 1;
        }
        while a + 2 < c {
            ans.push(a + 1);
            a += 1;
            b += 1;
        }
    }
    if a > c + 2 {
        if a <= b {
            ans.push(n);
            a += 1;
            b -= 1;
        }
        if b == a - 1 {
            ans.push(a);
            b -= 1;
        }
        while a > c + 2 {
            ans.push(a - 1);
            a -= 1;
            b -= 1;
        }
    }
    let target = if c < d {
        (c, (c + 2).min(d))
    } else if c > d {
        (c, (c - 2).max(d))
    } else {
        (c, c)
    };
    let mut queue = VecDeque::new();
    queue.push_back((a, b, Vec::new()));
    let mut has = HashSet::new();
    has.insert((a, b));
    while let Some((x, y, v)) = queue.pop_front() {
        if (x, y) == target {
            ans.extend_from_slice(&v);
            a = x;
            b = y;
            break;
        }
        if x == 0 || y == 0 || x == n || y == n || a + 5 < x || a + 5 < y || x + 5 < a || y + 5 < a
        {
            continue;
        }
        if has.insert((x + 1, y - 1)) {
            let mut v = v.clone();
            v.push(n);
            queue.push_back((x + 1, y - 1, v));
        }
        if has.insert((x - 1, y + 1)) {
            let mut v = v.clone();
            v.push(0);
            queue.push_back((x - 1, y + 1, v));
        }
        if x < y && has.insert((x, y + 1)) {
            let mut v = v.clone();
            v.push(x);
            queue.push_back((x, y + 1, v));
        }
        if x > y && has.insert((x, y - 1)) {
            let mut v = v.clone();
            v.push(x);
            queue.push_back((x, y - 1, v));
        }
        if x + 1 < y && has.insert((x + 1, y + 1)) {
            let mut v = v.clone();
            v.push(x + 1);
            queue.push_back((x + 1, y + 1, v));
        }
        if x > y + 1 && has.insert((x - 1, y - 1)) {
            let mut v = v.clone();
            v.push(x - 1);
            queue.push_back((x - 1, y - 1, v));
        }
    }
    // assert_eq!((a, b), target);
    if (a, b) != target {
        out.print_line(false);
        return;
    }
    while b < d {
        ans.push(a);
        b += 1;
    }
    while b > d {
        ans.push(a);
        b -= 1;
    }
    out.print_line(true);
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
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
    if false {
        true
    } else {
        input.skip_whitespace();
        input.peek().is_none()
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    tester::stress_test();
}
//END MAIN
