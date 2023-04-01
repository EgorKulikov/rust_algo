//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::persistent_fenwick::PersistentFenwickTree;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let seg = input.read_size_pair_vec(n).dec_by_one();

    let mut tp = vec![(false, 0); 2 * n];
    for (i, &(l, r)) in seg.iter().enumerate() {
        tp[l] = (true, i);
        tp[r] = (false, i);
    }
    let mut ft = PersistentFenwickTree::new(2 * n, 0);
    for (i, &(start, id)) in tp.iter().enumerate() {
        if start {
            ft.add(seg[id].1, 1, i);
        }
    }
    let mut qty_right = vec![0; 2 * n];
    let mut d_right = vec![0; 2 * n];
    let mut st = VecDeque::<usize>::new();
    let mut current = 0;
    let mut cur_comp = 0;
    let mut qty_in_comp = DefaultHashMap::<_, usize>::new();
    let mut in_comp = vec![0; n];
    for i in (0..2 * n).rev() {
        if st.is_empty() {
            cur_comp += 1;
        }
        if tp[i].0 {
            let &first = st.front().unwrap();
            if first == tp[i].1 {
                st.pop_front();
            }
            current -= 1;
        } else {
            qty_in_comp[cur_comp] += 1;
            in_comp[tp[i].1] = cur_comp;
            if let Some(&first) = st.front() {
                let sp = ft.get(0, seg[first].1, 2 * n) - ft.get(0, seg[first].1, i);
                qty_right[i] = qty_right[seg[first].1] + current + sp;
                d_right[i] = d_right[seg[first].1] + qty_right[i] + sp;
            }
            if let Some(&last) = st.back() {
                if seg[last].0 > seg[tp[i].1].0 {
                    st.push_back(tp[i].1);
                }
            } else {
                st.push_back(tp[i].1);
            }
            current += 1;
        }
    }
    let mut qty_left = vec![0; 2 * n];
    let mut d_left = vec![0; 2 * n];
    let mut st = VecDeque::<usize>::new();
    let mut current = 0;
    for i in 0..2 * n {
        if tp[i].0 {
            if let Some(&first) = st.front() {
                let sp = ft.get(0, i, 2 * n) - ft.get(0, i, seg[first].0);
                qty_left[i] = qty_left[seg[first].0] + current + sp;
                d_left[i] = d_left[seg[first].0] + qty_left[i] + sp;
            }
            if let Some(&last) = st.back() {
                if seg[last].1 < seg[tp[i].1].1 {
                    st.push_back(tp[i].1);
                }
            } else {
                st.push_back(tp[i].1);
            }
            current += 1;
        } else {
            let &first = st.front().unwrap();
            if first == tp[i].1 {
                st.pop_front();
            }
            current -= 1;
        }
    }
    for i in 0..n {
        let (l, r) = seg[i];
        out_line!(
            d_left[l] + d_right[r] + qty_in_comp[in_comp[i]] - qty_left[l] - qty_right[r] - 1
        );
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
}
//END MAIN
