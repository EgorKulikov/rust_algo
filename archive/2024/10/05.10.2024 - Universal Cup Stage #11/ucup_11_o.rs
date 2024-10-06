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
        if dsu.get(u) == dsu.get(v) {
            continue;
        }
        let mut sw = col[u] == col[v];
        u = dsu.get(u);
        v = dsu.get(v);
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
        dsu.join(u, v);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
