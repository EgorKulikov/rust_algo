//{"name":"F. Bracket Groups","group":"Codeforces - Educational Codeforces Round 182 (Rated for Div. 2)","url":"https://codeforces.com/contest/2144/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3 6\n)))\n(((\n(())\n","output":"1\n(()())\n3\n1 2 3\n"},{"input":"4 6\n(()\n()(\n())\n((((((\n","output":"2\n(())()\n1\n2\n()()()\n3\n1 4 3\n"},{"input":"2 6\n()\n)(\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr3d::Arr3d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::aho_corasick::{ACPayload, AhoCorasick};
use algo_lib::string::str::{Str, StrReader};
use algo_lib::string::string_algorithms::string_search::StringSearch;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let s = input.read_str_vec(n);

    if s.contains(&Str::from(b"()")) {
        out.print_line(-1);
        return;
    }
    #[derive(Default)]
    struct Node(bool);
    impl ACPayload for Node {
        fn add_single(&mut self, _id: usize) {
            self.0 = true;
        }

        fn add_node(&mut self, other: &Self) {
            self.0 |= other.0;
        }
    }
    type AC = AhoCorasick<Node, 2, b'('>;
    let ac = AC::new(&s);
    let mut good = Arr3d::new(k + 1, k + 1, ac.len(), None);
    good[(0, 0, 0)] = Some(Vec::new());
    for i in 0..k {
        for j in 0..=i {
            for k in 0..ac.len() {
                if let Some(s) = &good[(i, j, k)] {
                    let s = s.clone();
                    let v = ac.advance(k, b'(');
                    if !ac.payload_at(v).0 {
                        let mut ns = s.clone();
                        ns.push(b'(');
                        good[(i + 1, j + 1, v)] = Some(ns);
                    }
                    if j > 0 && !ac.payload_at(ac.advance(k, b')')).0 {
                        let mut ns = s.clone();
                        ns.push(b')');
                        good[(i + 1, j - 1, ac.advance(k, b')'))] = Some(ns);
                    }
                }
            }
        }
    }
    for j in 0..ac.len() {
        if let Some(s) = &good[(k, 0, j)] {
            out.print_line(1);
            out.print_line(Str::from(s.as_slice()));
            out.print_line(n);
            out.print_line_iter(1..=n);
            return;
        }
    }
    let mut cons = Vec::new();
    let mut other = Vec::new();
    for i in 0..n {
        if s[i].str_contains(b"((") || s[i].str_contains(b"))") {
            cons.push(i + 1);
        } else {
            other.push(i + 1);
        }
    }
    out.print_line(2);
    for _ in 0..k / 2 {
        out.print("()");
    }
    out.print_line(());
    out.print_line(cons.len());
    out.print_line(cons);
    for _ in 0..k / 2 {
        out.print('(');
    }
    for _ in 0..k / 2 {
        out.print(')');
    }
    out.print_line(());
    out.print_line(other.len());
    out.print_line(other);
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
