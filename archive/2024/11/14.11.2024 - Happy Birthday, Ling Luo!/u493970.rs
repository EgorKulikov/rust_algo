//{"name":"U493970 绝世丑角","group":"Luogu","url":"https://www.luogu.com.cn/problem/U493970?contestId=209328","interactive":false,"timeLimit":3000,"tests":[{"input":"6 6\n3 6 1 4 2 5\n2 2 5\n3 1 4\n1 5 5\n2 1 6\n1 1 3\n3 5 6\n","output":"1\n14\n6\n8\n"},{"input":"10 10\n1234567890 130113 3614258193 1000000007 3146527164 3141592653 2147483648 998244353 2346886432 20151114\n2 1 10\n3 5 8\n1 2 9\n3 1 4\n1 5 8\n1 3 10\n2 1 9\n3 1 8\n1 1 4\n2 8 8\n","output":"2499610911\n9433847818\n4602641167\n4258698016\n17656837678\n704481058\n"},{"input":"10 10\n36 14 35 0 13 0 11 3 5 20\n2 1 10\n3 1 4\n2 2 5\n3 1 8\n3 1 4\n2 5 6\n2 8 9\n3 3 7\n3 1 10\n2 1 5\n","output":"29\n85\n32\n112\n85\n13\n6\n59\n137\n4\n"},{"input":"10 10\n36 14 35 0 13 0 11 3 5 20\n1 1 10\n2 1 4\n1 2 5\n2 1 8\n2 1 4\n2 5 6\n2 8 9\n1 3 7\n1 1 10\n2 1 5\n","output":"12\n21\n22\n14\n5\n9\n"},{"input":"10 10\n36 14 35 0 13 0 11 3 5 20\n1 5 5\n3 1 4\n2 2 5\n3 1 8\n1 9 9\n2 5 6\n1 9 9\n3 3 7\n3 1 10\n2 1 5\n","output":"85\n39\n109\n10\n56\n133\n3\n"},{"input":"10 10\n36 14 35 0 13 0 11 3 5 20\n1 1 10\n3 1 4\n2 2 5\n3 1 8\n1 1 10\n2 5 6\n1 1 10\n3 3 7\n3 1 10\n2 1 5\n","output":"112\n52\n139\n14\n76\n179\n7\n"},{"input":"10 10\n36 14 35 0 13 0 11 3 5 20\n1 3 8\n1 1 6\n1 2 10\n3 1 4\n2 2 5\n3 1 8\n2 5 6\n3 3 7\n3 1 10\n2 1 5\n","output":"119\n61\n139\n8\n73\n176\n15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"U493970"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::misc::value_ref::ValueRef;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::value_ref;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n);

    const SQ: [usize; 32] = [
        1, 3, 6, 13, 24, 52, 103, 222, 384, 832, 1648, 3552, 6237, 13563, 26511, 56906, 98304,
        212992, 421888, 909312, 1596672, 3472128, 6786816, 14567936, 25190110, 54589881, 108036850,
        232800673, 408783316, 888883132, 1737454078, 3729449897,
    ];

    let mut pw = Arr2d::new(32, 32, 0);
    for i in 0..32 {
        pw[(i, 0)] = SQ[i];
        for j in 0..31 {
            for k in 0..=i {
                if pw[(i, j)].is_set(k) {
                    pw[(i, j + 1)] ^= pw[(k, j)];
                }
            }
        }
    }

    value_ref!(Powers PW: Arr2d<usize> = pw);

    if a.iter().max().unwrap() < &64 {
        #[derive(Clone)]
        struct Node {
            qty: [usize; 64],
            xor: usize,
            sum: usize,
            delta: usize,
        }

        impl Node {
            fn new(x: usize) -> Self {
                let mut qty = [0; 64];
                qty[x] = 1;
                Self {
                    qty,
                    xor: x,
                    sum: x,
                    delta: 0,
                }
            }
        }

        impl SegmentTreeNode for Node {
            fn new(_left: usize, _right: usize) -> Self {
                Self::new(0)
            }

            fn join(&mut self, left_val: &Self, right_val: &Self) {
                self.xor = left_val.xor ^ right_val.xor;
                self.sum = left_val.sum + right_val.sum;
                for i in 0..64 {
                    self.qty[i] = left_val.qty[i] + right_val.qty[i];
                }
            }

            fn accumulate(&mut self, value: &Self) {
                if value.delta == 0 {
                    return;
                }
                let mut new_qty = [0; 64];
                for a in 0..64 {
                    if self.qty[a] == 0 {
                        continue;
                    }
                    let mut val = a;
                    for i in 0..20 {
                        if value.delta.is_set(i) {
                            let mut new_val = 0;
                            for j in 0..6 {
                                if val.is_set(j) {
                                    new_val ^= Powers::val()[(j, i)];
                                }
                            }
                            val = new_val;
                        }
                    }
                    new_qty[val] += self.qty[a];
                }
                self.qty = new_qty;
                self.xor = 0;
                self.sum = 0;
                for i in 0..64 {
                    if self.qty[i] % 2 == 1 {
                        self.xor ^= i;
                    }
                    self.sum += i * self.qty[i];
                }
                self.delta += value.delta;
            }

            fn reset_delta(&mut self) {
                self.delta = 0;
            }
        }

        let mut st = SegmentTree::from_generator(n, |x| Node::new(a[x]));
        for _ in 0..q {
            let t = input.read_int();
            let l = input.read_size() - 1;
            let r = input.read_size();

            match t {
                1 => {
                    st.update(
                        l..r,
                        &Node {
                            qty: [0; 64],
                            xor: 0,
                            sum: 0,
                            delta: 1,
                        },
                    );
                }
                2 => {
                    let node = st.query(l..r);
                    out.print_line(node.xor);
                }
                3 => {
                    let node = st.query(l..r);
                    out.print_line(node.sum);
                }
                _ => unreachable!(),
            }
        }
        return;
    }

    #[derive(Clone)]
    struct Node {
        xor: usize,
        sum: usize,
        delta: usize,
    }

    impl Node {
        fn new(x: usize) -> Self {
            Self {
                xor: x,
                sum: x,
                delta: 0,
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self::new(0)
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.xor = left_val.xor ^ right_val.xor;
            self.sum = left_val.sum + right_val.sum;
        }

        fn accumulate(&mut self, value: &Self) {
            if value.delta == 0 {
                return;
            }
            let mut val = self.xor;
            for i in 0..32 {
                if value.delta.is_set(i) {
                    let mut new_val = 0;
                    for j in 0..32 {
                        if val.is_set(j) {
                            new_val ^= Powers::val()[(j, i)];
                        }
                    }
                    val = new_val;
                }
            }
            self.xor = val;
            self.sum = val;
            self.delta += value.delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }

    let mut first = true;

    let mut st = SegmentTree::from_generator(n, |x| Node::new(a[x]));
    for _ in 0..q {
        let t = input.read_int();
        let l = input.read_size() - 1;
        let r = input.read_size();

        if t != 1 && first {
            first = false;
            for i in 0..n {
                st.update(
                    i..=i,
                    &Node {
                        xor: 0,
                        sum: 0,
                        delta: 0,
                    },
                );
            }
        }

        match t {
            1 => {
                st.update(
                    l..r,
                    &Node {
                        xor: 0,
                        sum: 0,
                        delta: 1,
                    },
                );
            }
            2 => {
                let node = st.query(l..r);
                out.print_line(node.xor);
            }
            3 => {
                let node = st.query(l..r);
                out.print_line(node.sum);
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use algo_lib::collections::fx_hash_map::FxHashSet;
    use algo_lib::collections::md_arr::arr2d::Arr2d;

    #[test]
    fn test() {
        let mut prod = Arr2d::new(50, 50, 0);
        for a in 0..50 {
            for b in 0..=a {
                let mut set = FxHashSet::default();
                for c in 0..a {
                    for d in 0..b {
                        set.insert(prod[(a, d)] ^ prod[(c, b)] ^ prod[(c, d)]);
                    }
                }
                for x in 0.. {
                    if !set.contains(&x) {
                        prod[(a, b)] = x;
                        prod[(b, a)] = x;
                        break;
                    }
                }
            }
            println!("{} {}", a, prod[(a, a)]);
        }
    }
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
