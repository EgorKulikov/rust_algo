//{"name":"P6 - Subsequence Reversal","group":"DMOJ - Dilhan's Computing Contest 1","url":"https://dmoj.ca/problem/dcc1p6","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n1 3 4 2\n","output":"4\n1110\n0100\n0011\n1111\n"},{"input":"8\n1 2 3 4 5 6 7 8\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut p = input.read_size_vec(n).dec();

    let mut ans = Vec::new();
    struct Part {
        from_left: usize,
        to_left: usize,
        from_right: usize,
        to_right: usize,
    }
    impl Part {
        fn in_part(&self, i: usize) -> bool {
            self.from_left <= i && i < self.to_left || self.from_right <= i && i < self.to_right
        }
        fn len(&self) -> usize {
            self.to_left - self.from_left + self.to_right - self.from_right
        }
        fn iter(&self) -> impl Iterator<Item = usize> {
            (self.from_left..self.to_left).chain(self.from_right..self.to_right)
        }
    }
    let mut parts = vec![Part {
        from_left: 0,
        to_left: n / 2,
        from_right: n / 2,
        to_right: n,
    }];
    let mut max_part_size = n;
    while max_part_size > 2 {
        let mut m1 = Str::from(vec![b'0'; n]);
        let mut m2 = Str::from(vec![b'0'; n]);
        let mut new_parts = Vec::new();
        max_part_size = 0;
        let mut m1_pairs = Vec::new();
        let mut m2_pairs = Vec::new();
        for part in parts {
            let mid_left = (part.from_left + part.to_left) / 2;
            let mid_right = (part.from_right + part.to_right + 1) / 2;
            let p1 = Part {
                from_left: part.from_left,
                to_left: mid_left,
                from_right: mid_right,
                to_right: part.to_right,
            };
            let p2 = Part {
                from_left: mid_left,
                to_left: part.to_left,
                from_right: part.from_right,
                to_right: mid_right,
            };
            if p1.len() == 0 {
                new_parts.push(p2);
                continue;
            }
            max_part_size.maxim(p2.to_left - p2.from_left + p2.to_right - p2.from_right);
            let p1_left = (part.from_left..mid_left)
                .filter(|&i| !p1.in_part(p[i]))
                .collect::<Vec<_>>();
            let p2_left = (mid_left..part.to_left)
                .filter(|&i| !p2.in_part(p[i]))
                .collect::<Vec<_>>();
            let p2_right = (part.from_right..mid_right)
                .filter(|&i| !p2.in_part(p[i]))
                .collect::<Vec<_>>();
            let p1_right = (mid_right..part.to_right)
                .filter(|&i| !p1.in_part(p[i]))
                .collect::<Vec<_>>();
            if p2_right.len() > p1_left.len() {
                let mut p2_left_in = (mid_left..part.to_left)
                    .filter(|&i| p2.in_part(p[i]))
                    .collect::<Vec<_>>()
                    .reversed();
                for x in p1_left.indices() {
                    p.swap(p1_left[x], p2_right[Back(x)]);
                    m1[p1_left[x]] = b'1';
                    m1[p2_right[Back(x)]] = b'1';
                    m1_pairs.push((p1_left[x], p2_right[Back(x)]));
                }
                for x in p1_left.len()..p2_right.len() {
                    let y = p2_left_in.pop().unwrap();
                    p.swap(y, p2_right[Back(x)]);
                    m1[y] = b'1';
                    m1[p2_right[Back(x)]] = b'1';
                    m1_pairs.push((y, p2_right[Back(x)]));
                }
                let p2_left = (mid_left..part.to_left)
                    .filter(|&i| !p2.in_part(p[i]))
                    .collect::<Vec<_>>();
                assert_eq!(p2_left.len(), p1_right.len());
                for x in p2_left.indices() {
                    p.swap(p2_left[x], p1_right[Back(x)]);
                    m2[p2_left[x]] = b'1';
                    m2[p1_right[Back(x)]] = b'1';
                    m2_pairs.push((p2_left[x], p1_right[Back(x)]));
                }
            } else {
                let mut p2_right_in = (part.from_right..mid_right)
                    .filter(|&i| p2.in_part(p[i]))
                    .collect::<Vec<_>>();
                for x in p1_right.indices() {
                    p.swap(p2_left[x], p1_right[Back(x)]);
                    m1[p2_left[x]] = b'1';
                    m1[p1_right[Back(x)]] = b'1';
                    m1_pairs.push((p2_left[x], p1_right[Back(x)]));
                }
                for x in p1_right.len()..p2_left.len() {
                    let y = p2_right_in.pop().unwrap();
                    p.swap(y, p2_left[x]);
                    m1[y] = b'1';
                    m1[p2_left[x]] = b'1';
                    m1_pairs.push((p2_left[x], y));
                }
                let p2_right = (part.from_right..mid_right)
                    .filter(|&i| !p2.in_part(p[i]))
                    .collect::<Vec<_>>();
                assert_eq!(p2_right.len(), p1_left.len());
                for x in p2_right.indices() {
                    p.swap(p1_left[x], p2_right[Back(x)]);
                    m2[p1_left[x]] = b'1';
                    m2[p2_right[Back(x)]] = b'1';
                    m2_pairs.push((p1_left[x], p2_right[Back(x)]));
                }
            }
            for i in p1.iter() {
                assert!(p1.in_part(p[i]));
            }
            for i in p2.iter() {
                assert!(p2.in_part(p[i]));
            }
            new_parts.push(p1);
            new_parts.push(p2);
        }
        // eprintln!("m1_pairs: {:?}", m1_pairs);
        // eprintln!("m2_pairs: {:?}", m2_pairs);
        parts = new_parts;
        if m1.copy_count(b'1') != 0 {
            ans.push(m1);
        }
        if m2.copy_count(b'1') != 0 {
            ans.push(m2);
        }
    }
    let mut m = Str::from(vec![b'0'; n]);
    for part in parts {
        assert!(part.len() <= 2);
        if part.from_left != part.to_left && p[part.from_left] != part.from_left {
            p.swap(part.from_left, part.from_right);
            m[part.from_left] = b'1';
            m[part.from_right] = b'1';
        }
    }
    if m.copy_count(b'1') != 0 {
        ans.push(m);
    }
    out.print_line(ans.len());
    out.print_per_line(&ans);
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
