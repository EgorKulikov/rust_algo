use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let xy = input.read_size_pair_vec(k).sorted().dec();



    #[derive(Default)]
    struct SubNode {
        val: bool,
    }

    impl SegmentTreeNode for SubNode {
        fn accumulate(&mut self, value: &Self) {
            self.val ^= value.val;
        }

        fn reset_delta(&mut self) {
            self.val = false;
        }
    }

    struct Node {
        st: SegmentTree<SubNode>,
    }

    impl Default for Node {
        fn default() -> Self {
            Self { st: SegmentTree::<SubNode>::new(0) }
        }
    }

    impl SegmentTreeNode for Node {}

    enum Table {
        Small(SegmentTree<Node>),
        Big,
    }
    impl Table {
        fn new(n: usize, m: usize) -> Self {
            if n.max(m) <= 1000 {
                Self::Small(SegmentTree::with_gen_full(n - 1, |_, _| Node { st: SegmentTree::<SubNode>::new(m - 1) }))
            } else {
                Self::Big
            }
        }

        fn flip(&mut self, x0: usize, x1: usize, y0: usize, y1: usize) {
            match self {
                Self::Small(st) => {
                    st.for_each_mut(x0..x1, |_, node| node.st.update(y0..y1, &SubNode { val: true }));
                }
                Self::Big => {}
            }
        }
    }

    let mut table = Table::new(n, m);

    if n.max(m) <= 1000 {
        let mut row = vec![None; n];
        let mut col = vec![false; m];
        for (x, y) in xy {
            match row[x] {
                Some(val) => {
                    table.flip(x, n - 1, val, y);
                    col[y] ^= true;
                    col[val] ^= true;
                    row[x] = None;
                }
                None => {
                    row[x] = Some(y);
                }
            }
        }
        if let Some(val) = row[n - 1] {
            col[val] ^= true;
            row[n - 1] = None;
        }
        let mut seen = vec![None; m];
        for i in 0..n {
            if let Some(val) = row[i] {
                if col[val] {
                    table.flip(i, n - 1, 0, val);
                    col[val] ^= true;
                    col[0] ^= true;
                    row[i] = Some(0);
                } else if let Some(r) = seen[val] {
                    table.flip(r, i, 0, val);
                    row[r] = Some(0);
                    row[i] = Some(0);
                    seen[val] = None;
                } else {
                    seen[val] = Some(i);
                }
            }
        }
        let mut left = Vec::new();
        for i in 0..n {
            if row[i] == Some(0) {
                left.push(i);
            }
        }
        for i in (1..m).rev() {
            if col[i] && !left.is_empty() {
                let r = left.pop().unwrap();
                table.flip(r, n - 1, 0, i);
                col[i] = false;
                col[0] ^= true;
                row[r] = Some(i);
            }
        }
        let ans = n * m - col.copy_count(true) - row.copy_filter(Option::is_some).count();
        out.print_line(ans);
        match table {
            Table::Small(mut st) => {
                for i in 0..n - 1 {
                    for j in 0..m - 1 {
                        let mut res = false;
                        st.point_through_update(i, |node| res ^= node.st.point_query(j).val);
                        if res {
                            out.print(b'1');
                        } else {
                            out.print(b'0');
                        }
                    }
                    out.print_line(());
                }
            }
            Table::Big => {}
        }
        return;
    }


    let mut row = DefaultHashMap::new(None);
    let mut col = DefaultHashMap::new(false);
    for (x, y) in xy {
        match row[x] {
            Some(val) => {
                table.flip(x, n - 1, val, y);
                col[y] ^= true;
                col[val] ^= true;
                row[x] = None;
            }
            None => {
                row[x] = Some(y);
            }
        }
    }
    if let Some(val) = row[n - 1] {
        col[val] ^= true;
        row[n - 1] = None;
    }
    let mut seen = DefaultHashMap::new(None);
    let ids = row.keys().copied().collect::<Vec<_>>();
    for i in ids {
        if let Some(val) = row[i] {
            if col[val] {
                table.flip(i, n - 1, 0, val);
                col[val] ^= true;
                col[0] ^= true;
                row[i] = Some(0);
            } else if let Some(r) = seen[val] {
                table.flip(r, i, 0, val);
                row[r] = Some(0);
                row[i] = Some(0);
                seen[val] = None;
            } else {
                seen[val] = Some(i);
            }
        }
    }
    let mut left = Vec::new();
    for i in row.keys().copied() {
        if row[i] == Some(0) {
            left.push(i);
        }
    }
    let mut keys = col.keys().copied().collect::<Vec<_>>();
    if let Some(pos) = keys.copy_find(0) {
        keys.swap_remove(pos);
    }
    for i in keys {
        if col[i] && !left.is_empty() {
            let r = left.pop().unwrap();
            table.flip(r, n - 1, 0, i);
            col[i] = false;
            col[0] ^= true;
            row[r] = Some(i);
        }
    }
    let ans = n * m - col.iter().filter(|x| *x.1).count() - row.iter().filter(|x| x.1.is_some()).count();
    out.print_line(ans);
    match table {
        Table::Small(mut st) => {
            for i in 0..n - 1 {
                for j in 0..m - 1 {
                    let mut res = false;
                    st.point_through_update(i, |node| res ^= node.st.point_query(j).val);
                    if res {
                        out.print(b'1');
                    } else {
                        out.print(b'0');
                    }
                }
                out.print_line(());
            }
        }
        Table::Big => {}
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}


#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
