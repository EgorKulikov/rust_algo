//{"name":"G - Abs Sum","group":"AtCoder - Toyota Programming Contest 2024#12（AtCoder Beginner Contest 384）","url":"https://atcoder.jp/contests/abc384/tasks/abc384_g","interactive":false,"timeLimit":5000,"tests":[{"input":"2\n2 4\n3 5\n4\n1 1\n1 2\n2 1\n2 2\n","output":"1\n4\n2\n6\n"},{"input":"5\n1163686 28892 1263085 2347878 520306\n1332157 1202905 2437161 1291976 563395\n5\n5 3\n1 5\n2 3\n1 2\n5 5\n","output":"13331322\n2209746\n6366712\n207690\n20241215\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GAbsSum"}}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::treap::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::Tree;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);
    let k = input.read_size();
    let queries = input.read_size_pair_vec(k).dec();

    let mut ans = vec![0; k];
    let mut by_x = vec![Vec::new(); n];
    for (i, (x, y)) in queries.iter_enumerate() {
        by_x[x].push((i, y));
    }
    const BUBEN: usize = 300;
    let mut segs = Vec::with_capacity(n / BUBEN);
    for i in 0..n / BUBEN {
        let mut seg = Vec::new();
        for j in BUBEN * i..BUBEN * (i + 1) {
            seg.push(b[j]);
        }
        seg.sort();
        let p = seg.partial_sums();
        segs.push((seg, p));
    }
    let mut delta = vec![0; n / BUBEN];
    struct Node {
        key: (i64, usize),
        val: i64,
        sum: i64,
    }
    impl Payload for Node {
        const NEED_UPDATE: bool = true;
        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum = self.val + left.map_or(0, |x| x.sum) + right.map_or(0, |x| x.sum);
        }
    }
    impl OrdPayload for Node {
        type Key = (i64, usize);

        fn key(&self) -> &Self::Key {
            &self.key
        }
    }
    let mut treap = Tree::new();
    for i in 0..n {
        for j in segs.indices() {
            let (seg, p) = &segs[j];
            let pos = seg.lower_bound(&a[i]);
            delta[j] += a[i] * (pos as i64) - p[pos] + (p[Back(0)] - p[pos])
                - a[i] * ((seg.len() - pos) as i64);
        }
        treap.insert(Node {
            key: (a[i], i),
            val: a[i],
            sum: a[i],
        });
        for (j, y) in by_x[i].drain(..) {
            let mut cur = 0;
            for k in 0..y / BUBEN {
                cur += delta[k];
            }
            for k in y / BUBEN * BUBEN..=y {
                let left = treap.range(..&(b[k], 0));
                let left_len = left.size() as i64;
                let left_sum = left.payload().map_or(0, |x| x.sum);
                let right = treap.range(&(b[k], 0)..);
                let right_len = right.size() as i64;
                let right_sum = right.payload().map_or(0, |x| x.sum);
                cur += left_len * b[k] - left_sum + right_sum - right_len * b[k];
            }
            ans[j] = cur;
        }
    }
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
