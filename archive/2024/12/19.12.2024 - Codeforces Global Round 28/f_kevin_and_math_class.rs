//{"name":"F. Kevin and Math Class","group":"Codeforces - Codeforces Global Round 28","url":"https://codeforces.com/contest/2048/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n5 4 2\n6 3 2\n5\n3 6 1 3 2\n3 5 3 2 2\n6\n8 3 3 7 5 8\n3 2 3 4 2 3\n","output":"2\n3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FKevinAndMathClass"}}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::default::default_vec;
use algo_lib::collections::vec_ext::gen::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);

    struct Node {
        min_a: i64,
        max_a: i64,
        min_b: i64,
        children: Vec<Node>,
        from: usize,
        to: usize,
    }

    impl Node {
        fn flat(&mut self) {
            let mut i = 0;
            while i < self.children.len() {
                if self.children[i].min_b == self.min_b {
                    let mut child = self.children.swap_remove(i);
                    self.min_a.maxim(child.min_a);
                    self.children.extend(child.children.drain(..));
                } else {
                    self.children[i].flat();
                    i += 1;
                }
            }
        }

        fn answer(&self, mut x: i64, max_steps: i64) -> i64 {
            let max = self.max_a;
            let min = self.min_a;
            let mut need = 0;
            let mut ans = 0;
            let mut xx = x;

            while max > xx {
                if min > xx {
                    need += 1;
                }
                xx = xx.saturating_mul(self.min_b);
                ans += 1;
            }
            ans.minim(max_steps);
            if need >= ans {
                return ans;
            }
            for _ in 0..need {
                x *= self.min_b;
            }
            for i in need..ans {
                if i >= ans {
                    break;
                }
                let mut cand = i;
                for child in &self.children {
                    cand += child.answer(x, ans - cand);
                    if cand >= ans {
                        break;
                    }
                }
                ans.minim(cand);
                x = x.saturating_mul(self.min_b);
            }
            ans
        }
    }

    let mut id = vec![None; n];
    let mut nodes: Vec<Option<Node>> = default_vec(n);
    let order = Vec::gen(n, |i, _| i).do_with(|v| v.sort_by_key(|&i| -b[i]));
    for i in order {
        let mut node = Node {
            min_a: a[i],
            max_a: a[i],
            min_b: b[i],
            children: Vec::new(),
            from: i,
            to: i,
        };
        if i > 0 {
            if let Some(left) = id[i - 1] {
                let left: &mut Option<Node> = &mut nodes[left];
                let left = left.take().unwrap();
                node.from = left.from;
                node.max_a.maxim(left.max_a);
                node.min_b.minim(left.min_b);
                node.children.push(left);
            }
        }
        if i + 1 < n {
            if let Some(right) = id[i + 1] {
                let right: &mut Option<Node> = &mut nodes[right];
                let right = right.take().unwrap();
                node.to = right.to;
                node.max_a.maxim(right.max_a);
                node.min_b.minim(right.min_b);
                node.children.push(right);
            }
        }
        id[node.from] = Some(i);
        id[node.to] = Some(i);
        nodes[i] = Some(node);
    }
    let mut root = nodes.iter_filter(|n| n.is_some()).next().unwrap().unwrap();
    root.flat();
    out.print_line(root.answer(1, 100));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
