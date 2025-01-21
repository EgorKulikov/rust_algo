//{"name":"P6 - Tribes","group":"DMOJ - Yet Another Contest 9","url":"https://dmoj.ca/problem/yac9p6","interactive":false,"timeLimit":5000,"tests":[{"input":"5 10\n1 1 2 2 2\n1 2\n1 3\n3 4\n3 5\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P6Tribes"}}}

use algo_lib::collections::default_map::DefaultHashMap;
// use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::dynamic_value;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use std::ops::Add;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_unsigned();
    let a = input.read_size_vec(n).dec();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    dynamic_value!(Modulo: u32 = m);
    type Mod = ModInt<Modulo>;
    /*if a.copy_count(a[0]) == n {
        out.print_line(n as u32 % m);
        return;
    }
    if a.copy_max() <= 1 {
        for (u, v) in edges {
            if a[u] != a[v] {
                let mut q1 = vec![Mod::zero(); n];
                let mut dfs =
                    RecursiveFunction3::new(|f, vert: usize, prev: usize, level: usize| {
                        q1[level] += Mod::one();
                        for e in &graph[vert] {
                            if e.to() == prev {
                                continue;
                            }
                            f.call(e.to(), vert, level + 1);
                        }
                    });
                dfs.call(u, v, 0);
                let mut q2 = vec![Mod::zero(); n];
                let mut dfs =
                    RecursiveFunction3::new(|f, vert: usize, prev: usize, level: usize| {
                        q2[level] += Mod::one();
                        for e in &graph[vert] {
                            if e.to() == prev {
                                continue;
                            }
                            f.call(e.to(), vert, level + 1);
                        }
                    });
                dfs.call(v, u, 0);
                let mut ans = Mod::zero();
                for i in 0..n {
                    ans += q1[i] * q2[i];
                    if i + 1 < n {
                        ans += q1[i] * q2[i + 1];
                        ans += q1[i + 1] * q2[i];
                    }
                }
                out.print_line(ans);
                return;
            }
        }
        unreachable!();
    }*/

    struct Call {
        val: DefaultHashMap<i32, Mod>,
        shift: i32,
    }

    impl Call {
        fn new() -> Self {
            Self {
                val: DefaultHashMap::new(Mod::zero()),
                shift: 0,
            }
        }

        fn get(&self, i: i32) -> Mod {
            self.val[i - self.shift]
        }

        fn add(&mut self, i: i32, val: Mod) {
            if i >= 0 && val != Mod::zero() {
                self.val[i - self.shift] += val;
            }
        }

        fn dec(&mut self) {
            self.val.remove(&-self.shift);
            self.shift -= 1;
        }

        fn inc(&mut self) {
            self.shift += 1;
        }

        fn intersect(a: &Self, b: &Self) -> Self {
            let mut res = Self::new();
            if a.val.len() > b.val.len() {
                return Self::intersect(b, a);
            }
            let a_shift = a.shift;
            for (k, &v) in a.val.iter() {
                let mut k = *k;
                k += a_shift;
                let b_val = b.get(k);
                if b_val != Mod::zero() {
                    res.add(k, v * b_val);
                }
            }
            res
        }

        fn unite(mut a: Self, mut b: Self) -> Self {
            if a.val.len() < b.val.len() {
                std::mem::swap(&mut a, &mut b);
            }
            let b_shift = b.shift;
            for (k, v) in b.val {
                let mut k = k;
                k += b_shift;
                a.add(k, v);
            }
            a
        }

        fn fizz(a: Self) -> Self {
            let mut res = Self::new();
            let shift = a.shift;
            for (mut k, v) in a.val {
                k += shift;
                res.add(k, v);
                res.add(k + 1, v);
                res.add(k - 1, v);
            }
            res
        }
    }

    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (Option<Call>, Call) {
        let mut is_hq = Mod::one();
        let mut hq_up: Option<Call> = None;
        let mut hq_down = Call::new();
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let (call_up, mut call_down) = f.call(e.to(), vert);
            if a[vert] != a[e.to()] {
                let call_down = Call::fizz(call_down);
                hq_down = Call::intersect(&hq_down, &call_down);
                is_hq *= call_down.get(0);
                if let Some(up) = hq_up {
                    hq_up = Some(Call::intersect(&up, &call_down));
                } else {
                    hq_up = Some(call_down);
                }
            } else {
                call_down.inc();
                if let Some(mut call_up) = call_up {
                    call_up.dec();
                    is_hq *= call_up.get(0);
                    hq_down = Call::intersect(&hq_down, &call_up);
                    if let Some(up) = hq_up {
                        let or_down = Call::intersect(&up, &call_down);
                        hq_down = Call::unite(hq_down, or_down);
                        hq_up = Some(Call::intersect(&up, &call_up));
                    } else {
                        hq_down = Call::unite(hq_down, call_down);
                        hq_up = Some(call_up);
                    }
                } else {
                    if let Some(up) = hq_up.as_ref() {
                        let or_down = Call::intersect(up, &call_down);
                        hq_down = Call::unite(hq_down, or_down);
                    } else {
                        hq_down = Call::unite(hq_down, call_down);
                    }
                }
            }
        }
        if is_hq != Mod::zero() {
            hq_down.add(0, is_hq);
        }
        (hq_up, hq_down)
    });
    let (_, ans) = dfs.call(0, n);
    let ans = ans
        .val
        .into_values()
        .reduce(Mod::add)
        .unwrap_or(Mod::zero());
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
