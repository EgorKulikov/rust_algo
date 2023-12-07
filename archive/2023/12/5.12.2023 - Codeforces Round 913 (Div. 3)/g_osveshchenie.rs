//{"name":"G. Освещение","group":"Codeforces - Codeforces Round 913 (Div. 3)","url":"https://codeforces.com/contest/1907/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n5\n11101\n4 3 4 2 2\n2\n10\n2 1\n10\n0000000011\n9 10 10 7 10 9 9 9 10 2\n10\n1000111101\n9 3 8 9 2 1 3 7 2 7\n10\n0001101010\n5 7 6 10 8 3 6 6 2 2\n10\n0101100010\n8 7 7 9 9 4 1 4 2 7\n10\n1010111010\n7 9 10 7 7 2 8 6 10 4\n10\n1110000001\n3 10 10 1 10 8 6 3 2 1\n","output":"3\n1 5 3\n-1\n1\n9\n5\n5 6 10 2 3\n6\n4 9 5 10 8 7\n3\n5 4 9\n6\n1 3 5 9 7 8\n2\n2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GOsveshchenie"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut s = input
        .read_str()
        .into_iter()
        .map(|c| c == b'1')
        .collect::<Vec<_>>();
    let a = input.read_size_vec(n).dec();

    let mut d = a.qty_bound(n);
    let mut done = BitSet::new(n);
    let mut q = Vec::new();
    for i in 0..n {
        if d[i] == 0 {
            q.push(i);
        }
    }
    let mut ans = Vec::new();
    while let Some(i) = q.pop() {
        if s[i] {
            ans.push(i);
            s[i] = false;
            s[a[i]] ^= true;
        }
        done.set(i);
        d[a[i]] -= 1;
        if d[a[i]] == 0 {
            q.push(a[i]);
        }
    }
    for i in 0..n {
        if !done[i] && s[a[i]] {
            let mut cost = None;
            let mut b_ans = Vec::new();
            for st in [i, a[i]] {
                let mut c_ans = vec![st];
                s[st] ^= true;
                s[a[st]] ^= true;
                let mut cur = a[st];
                while cur != st {
                    if s[cur] {
                        c_ans.push(cur);
                        s[cur] ^= true;
                        s[a[cur]] ^= true;
                    }
                    cur = a[cur];
                }
                let good = !s[st];
                for &j in &c_ans {
                    s[j] ^= true;
                    s[a[j]] ^= true;
                }
                if good {
                    if cost.minim(c_ans.len()) {
                        b_ans = c_ans;
                    }
                }
            }
            if cost.is_some() {
                for &j in &b_ans {
                    ans.push(j);
                    s[j] ^= true;
                    s[a[j]] ^= true;
                }
            } else {
                out.print_line(-1);
                return;
            }
        }
    }
    out.print_line(ans.len());
    out.print_line(ans.inc().sorted());
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
