//{"name":"B. Мясник","group":"Codeforces - Codeforces Round 866 (Div. 1)","url":"https://codeforces.com/contest/1819/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n1 2\n3 5\n1 3\n3\n1 1\n1 1\n1 1\n1\n10 10\n4\n3 2\n5 5\n2 2\n8 7\n","output":"1\n4 5\n2\n1 3\n3 1\n1\n10 10\n1\n13 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMyasnik"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::TransposePairVec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::cmp::Reverse;
use std::collections::BTreeSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let r = input.read_size_pair_vec(n);

    if n == 1 {
        out_line!(1);
        out_line!(r[0].0, r[0].1);
        return;
    }
    let mut max_a = 0;
    let mut max_b = 0;
    for &(a, b) in &r {
        max_a.maxim(a);
        max_b.maxim(b);
    }
    let mut ans = Vec::new();
    for (b, r) in [(false, r.clone()), (true, r.transpose_pair_vec())] {
        let mut by_max_width = BTreeSet::new();
        let mut by_max_height = BTreeSet::new();
        for (i, &(a, b)) in r.iter().enumerate() {
            by_max_width.insert((Reverse(a), i));
            by_max_height.insert((Reverse(b), i));
        }
        let (Reverse(width), id) = by_max_width.iter().next().copied().unwrap();
        by_max_width.remove(&(Reverse(width), id));
        by_max_height.remove(&(Reverse(r[id].1), id));
        let mut height = r[id].1;
        while let Some(&(Reverse(w), i)) = by_max_width.iter().next() {
            if w != width {
                break;
            }
            height += r[i].1;
            by_max_width.remove(&(Reverse(w), i));
            by_max_height.remove(&(Reverse(r[i].1), i));
        }
        let mut good = true;
        if let Some(&(Reverse(h), _)) = by_max_height.iter().next() {
            height += h;
            let mut ww = width;
            let mut hh = h;
            loop {
                let mut found = false;
                while let Some(&(Reverse(h), i)) = by_max_height.iter().next() {
                    if h > hh {
                        good = false;
                        break;
                    }
                    if h < hh {
                        break;
                    }
                    found = true;
                    if ww < r[i].0 {
                        good = false;
                        break;
                    }
                    ww -= r[i].0;
                    by_max_height.remove(&(Reverse(h), i));
                    by_max_width.remove(&(Reverse(r[i].0), i));
                }
                if !found {
                    good = false;
                }
                if !good {
                    break;
                }
                if by_max_height.is_empty() {
                    break;
                }
                found = false;
                while let Some(&(Reverse(w), i)) = by_max_width.iter().next() {
                    if w > ww {
                        good = false;
                        break;
                    }
                    if w < ww {
                        break;
                    }
                    found = true;
                    if hh < r[i].1 {
                        good = false;
                        break;
                    }
                    hh -= r[i].1;
                    by_max_width.remove(&(Reverse(w), i));
                    by_max_height.remove(&(Reverse(r[i].1), i));
                }
                if !found {
                    good = false;
                }
                if !good {
                    break;
                }
                if by_max_width.is_empty() {
                    break;
                }
            }
            if ww != 0 && hh != 0 {
                good = false;
            }
        }
        if good {
            if b {
                ans.push((height, width));
            } else {
                ans.push((width, height));
            }
        }
    }
    out_line!(ans.len());
    output().print_per_line(&ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
