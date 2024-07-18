//{"name":"uc23_j","group":"Manual","url":"","interactive":true,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"uc23_j"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut edges = HashSet::new();
    let mut next = DefaultHashMap::<_, usize>::new();
    let mut last = None;
    let mut last_id = None;
    let mut tree = DefaultHashMap::<_, Vec<_>>::new();
    let mut parent = DefaultHashMap::new();
    let mut edge_to_parent = DefaultHashMap::new();

    loop {
        let x = input.read_size();
        let dx = input.read_size();
        if next[x] == 0 {
            tree[x] = vec![0; dx];
            parent[x] = last.unwrap_or(0);
        }
        if let Some(last) = last {
            if parent[last] == x {
                edge_to_parent[last] = last_id.unwrap();
            }
            if next[x] == 0 {
                tree[last][last_id.unwrap()] = x;
            }
            if !edges.contains(&(last, x)) && !edges.contains(&(x, last)) {
                edges.insert((last, x));
            }
        }
        last = Some(x);

        // eprintln!("{:#?}", tree.iter().collect_vec());

        if next[x] == dx {
            let mut go = false;
            for i in tree[x].indices() {
                if tree[x][i] != 0 {
                    tree[x][i] = 0;
                    out.print_line(('>', i + 1));
                    go = true;
                    break;
                }
            }
            if go {
                continue;
            }
            if x == 1 {
                out.print("! ");
                out.print_iter(edges.into_iter());
                out.print_line(());
                if input.read_str() == b"Correct".into() {
                    return;
                }
                panic!("Incorrect");
            }
            out.print_line(('>', edge_to_parent[x] + 1));
            continue;
        }
        last_id = Some(next[x]);
        next[x] += 1;
        out.print_line(('>', next[x]));
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    // input.skip_whitespace();
    // input.peek().is_none()
    true
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
