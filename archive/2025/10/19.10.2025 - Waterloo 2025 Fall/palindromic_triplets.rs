//{"name":"Palindromic Triplets","group":"DMOJ - Waterloo 2025 Fall B","url":"https://dmoj.ca/problem/waterloo2025fb","interactive":false,"timeLimit":3000,"tests":[{"input":"4\na\nab\nba\ncc\n","output":"6\n"},{"input":"2\na\na\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::value_ref;

use algo_lib::misc::test_type::TestType;
use algo_lib::misc::value_ref::ValueRef;
use algo_lib::string::aho_corasick::{ACPayload, AhoCorasickLowercase};
use algo_lib::string::hash::{SimpleHash, StringHash};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str_vec(n);

    let h = Vec::with_gen(n, |i| SimpleHash::new(&s[i]));
    let rh = Vec::with_gen(n, |i| {
        let rev_s = s[i].to_vec().reversed();
        SimpleHash::new(&rev_s)
    });

    let mut full = DefaultHashMap::new(0usize);
    let mut id = DefaultHashMap::new(0usize);
    for i in 0..n {
        full[h[i].hash(..)] += 1;
        id[h[i].hash(..)] = i;
    }
    #[derive(Default)]
    struct Node {
        res: Vec<(usize, usize)>,
    }
    let mut ss = Vec::new();
    let mut len_q = Vec::new();
    for (k, v) in id {
        ss.push(s[v].clone());
        len_q.push((full[k], s[v].len()));
    }
    value_ref!(LQ: Vec<(usize, usize)> = len_q);
    impl ACPayload for Node {
        fn add_single(&mut self, id: usize) {
            LQ::with(|lq| {
                self.res.push(lq[id]);
            });
        }

        fn add_node(&mut self, other: &Self) {
            self.res.extend_from_slice(&other.res);
        }
    }
    let ac = AhoCorasickLowercase::<Node>::new(&ss);
    for i in ss.indices() {
        ss[i].reverse();
    }
    let rac = AhoCorasickLowercase::<Node>::new(&ss);
    let mut suffix = DefaultHashMap::new(0);
    let mut prefix = DefaultHashMap::new(0);
    for i in 0..n {
        for j in 1..=s[i].len() {
            suffix[rh[i].hash(..s[i].len() - j)] += full[rh[i].hash(s[i].len() - j..)];
            prefix[rh[i].hash(j..)] += full[rh[i].hash(..j)];
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in (1..=s[i].len()).step_by(2) {
            if h[i].hash(..j) == rh[i].hash(s[i].len() - j..) {
                ans += suffix[h[i].hash(j..)];
            }
            if j != s[i].len() && h[i].hash(s[i].len() - j..) == rh[i].hash(..j) {
                ans += prefix[h[i].hash(..s[i].len() - j)];
            }
        }
    }
    for j in 0..n {
        let mut qty = vec![0; s[j].len() + 1];
        for (i, x) in rac.iterate(&s[j]).enumerate() {
            for (q, l) in x.res.copy_iter() {
                if l == i {
                    qty[i] += q;
                }
            }
            if (s[j].len() - i) % 2 == 1 && h[j].hash(i..) == rh[j].hash(..s[j].len() - i) {
                for (q, l) in x.res.copy_iter() {
                    if l < i {
                        ans += q * qty[i - l];
                    }
                }
            }
        }
        qty.fill(0);
        let mut ss = s[j].clone();
        ss.reverse();
        for (i, x) in ac.iterate(&ss).enumerate() {
            for (q, l) in x.res.copy_iter() {
                if l == i {
                    qty[i] += q;
                }
            }
            if (s[j].len() - i) % 2 == 1 && h[j].hash(..s[j].len() - i) == rh[j].hash(i..) {
                for (q, l) in x.res.copy_iter() {
                    if l < i {
                        ans += q * qty[i - l];
                    }
                }
            }
        }
    }
    out.print_line(ans);
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
