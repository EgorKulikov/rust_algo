//{"name":"ucucp_11_o","group":"Manual","url":"","interactive":false,"timeLimit":3000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ucucp_11_o"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::collections::HashSet;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    let mut dsu = DSU::new(2 * n);
    let mut sets = (0..2 * n).map(|i| vec![i]).collect_vec();
    let mut size = vec![(1, 0); 2 * n];
    let mut col = vec![0; 2 * n];
    let mut present = (0..2 * n).collect::<HashSet<_>>();
    for i in (0..m).rev() {
        let (mut u, mut v) = edges[i];
        if dsu.find(u) == dsu.find(v) {
            continue;
        }
        let mut sw = col[u] == col[v];
        u = dsu.find(u);
        v = dsu.find(v);
        if sets[u].len() < sets[v].len() {
            swap(&mut u, &mut v);
        }
        let alt_u = if sw {
            let res = (size[u].0 + size[v].0, size[u].1 + size[v].1);
            size[u] = (size[u].0 + size[v].1, size[u].1 + size[v].0);
            res
        } else {
            let res = (size[u].0 + size[v].1, size[u].1 + size[v].0);
            size[u] = (size[u].0 + size[v].0, size[u].1 + size[v].1);
            res
        };
        present.remove(&v);
        let mut with_qty = DefaultHashMap::<_, usize>::new();
        for &j in &present {
            let (u, v) = size[j];
            with_qty[(u.max(v), u.min(v))] += 1;
        }
        let mut ss = Vec::new();
        for ((u, v), mut q) in with_qty {
            while q > 0 {
                let mut sz = 1;
                while sz <= q {
                    ss.push((u * sz, v * sz));
                    q -= sz;
                    sz *= 2;
                }
            }
        }
        let mut can_do = BitSet::new(n + 1);
        can_do.set(0);
        for (x, y) in ss {
            // let (x, y) = size[j];
            let mut with_x = can_do.clone();
            with_x <<= x;
            can_do <<= y;
            can_do |= &with_x;
        }
        if !can_do[n] {
            size[u] = alt_u;
            sw = !sw;
        }
        if sw {
            for &j in &sets[v] {
                col[j] ^= 1;
            }
        }
        let mut sv = Vec::new();
        swap(&mut sv, &mut sets[v]);
        sets[u].extend(sv);
        dsu.union(u, v);
        present.remove(&v);
    }
    let present = present.into_iter().collect::<Vec<_>>();
    let mut last = Arr2d::new(present.len() + 1, n + 1, None);
    last[(0, 0)] = Some(true);
    for (i, &j) in present.iter().enumerate() {
        let (x, y) = size[j];
        for j in 0..=n {
            if last[(i, j)].is_some() {
                if j + x <= n {
                    last[(i + 1, j + x)] = Some(false);
                }
                if j + y <= n {
                    last[(i + 1, j + y)] = Some(true);
                }
            }
        }
    }
    assert!(last[(present.len(), n)].is_some());
    let mut cur = n;
    for (i, &j) in present.iter().enumerate().rev() {
        if last[(i + 1, cur)].unwrap() {
            cur -= size[j].1;
            for &j in &sets[j] {
                col[j] ^= 1;
            }
        } else {
            cur -= size[j].0;
        }
    }
    for i in col {
        out.print(i);
    }
    out.print_line(());
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

mod tester {
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(dead_code)]

    use crate::{run, TASK_TYPE};
    use algo_lib::collections::iter_ext::find_count::IterFindCount;
    use algo_lib::collections::min_max::MinimMaxim;
    use algo_lib::collections::vec_ext::inc_dec::IncDec;
    use algo_lib::io::input::Input;
    use algo_lib::io::output::Output;
    use algo_lib::misc::random::random;
    use algo_lib::numbers::num_traits::bit_ops::BitOps;
    use algo_lib::string::str::{Str, StrReader};
    use tester::interactive::std_interactor;
    use tester::test_set::GeneratedTestSet;
    use tester::Tester;

    const PRINT_LIMIT: usize = 1000;

    fn interact(
        mut sol_input: Input,
        mut sol_output: Output,
        mut input: Input,
    ) -> Result<(), String> {
        Ok(())
    }

    fn check(mut input: Input, expected: Option<Input>, mut output: Input) -> Result<(), String> {
        let n = input.read_size();
        let m = input.read_size();
        let edges = input.read_size_pair_vec(m).dec();
        let score = |mask: Str| -> Vec<i32> {
            let mut res = Vec::with_capacity(m);
            for &(u, v) in edges.iter().rev() {
                if mask[u] == mask[v] {
                    res.push(0);
                } else {
                    res.push(1);
                }
            }
            res
        };
        let mut output = output.read_str();
        if output.len() != 2 * n || output.iter().count_eq(&&b'1') != n {
            return Err("Invalid output".to_string());
        }
        if let Some(mut expected) = expected {
            let expected = expected.read_str();
            let expected_score = score(expected);
            let output_score = score(output);
            if output_score < expected_score {
                return Err("Output is worse than expected".to_string());
            }
            if output_score > expected_score {
                return Err("Output is better than expected".to_string());
            }
        }
        Ok(())
    }

    struct StressTest;

    impl GeneratedTestSet for StressTest {
        type TestId = usize;

        fn tests(&self) -> impl Iterator<Item = Self::TestId> {
            1..
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {
            let n = random().gen_range(1..=3);
            let m = random().gen_range(1..=6);
            out.print_line((n, m));
            for _ in 0..m {
                loop {
                    let first = random().gen_range(1..=2 * n);
                    let second = random().gen_range(1..=2 * n);
                    if first != second {
                        out.print_line((first, second));
                        break;
                    }
                }
            }
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            let n = input.read_size();
            let m = input.read_size();
            let edges = input.read_size_pair_vec(m).dec();
            let mut best_score = None;
            for i in usize::iter_all(2 * n) {
                if i.count_ones() != n as u32 {
                    continue;
                }
                let mut score = 0;
                for j in 0..m {
                    let (u, v) = edges[j];
                    if i.is_set(u) == i.is_set(v) {
                        score += 1 << j;
                    }
                }
                best_score.minim((score, i));
            }
            let mut at = best_score.unwrap().1;
            for i in 0..2 * n {
                out.print(at.is_set(i) as usize);
            }
            out.print_line(());
            true
        }
    }

    struct MaxTest;

    impl GeneratedTestSet for MaxTest {
        type TestId = usize;

        fn tests(&self) -> impl Iterator<Item = Self::TestId> {
            1..=1
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {
            let n = 5000;
            let m = 1000000;
            out.print_line((n, m));
            for _ in 0..m {
                loop {
                    let first = random().gen_range(1..=2 * n);
                    let second = random().gen_range(1..=2 * n);
                    if first != second {
                        out.print_line((first, second));
                        break;
                    }
                }
            }
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./ucucp_11_o";
        let tl = 3000;
        let tester = match TASK_TYPE {
            crate::TaskType::Interactive => {
                Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
                //Tester::new_interactive(time_limit, PRINT_LIMIT, path.to_string(), run, interact)
            }
            crate::TaskType::Classic => {
                // Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, default_checker)
                Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, check)
            }
        };
        let passed = tester.test_samples();
        // tester.test_generated("Max test", true, MaxTest);
        // tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn ucucp_11_o() {
    assert!(tester::run_tests());
}
