use algo_lib::collections::segment_tree::SegmentTreeNode;
use algo_lib::graph::path_segment_tree::PathSegmentTreeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::mem::swap;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_unsigned_vec(n);
    let mut edges = input.read_size_pair_vec(n - 1);

    for (u, v) in edges.iter_mut() {
        *u -= 1;
        *v -= 1;
    }

    #[derive(Default, Clone)]
    struct Node {
        inside_mask: Option<u32>,
        inside_paths: i64,
        paths_to_left: Vec<(u32, i64)>,
        paths_to_right: Vec<(u32, i64)>,
    }

    impl Node {
        fn new(val: u32) -> Self {
            Node {
                inside_mask: Some(val),
                inside_paths: 1,
                paths_to_left: vec![(val, 1)],
                paths_to_right: vec![(val, 1)],
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn join(left: &Self, right: &Self) -> Self {
            let total_mask = match (left.inside_mask, right.inside_mask) {
                (Some(l), Some(r)) => {
                    if (l & r) == 0 {
                        Some(l | r)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            let mut total_paths = left.inside_paths + right.inside_paths;
            for (l_mask, l_qty) in left.paths_to_right.iter() {
                for (r_mask, r_qty) in right.paths_to_left.iter() {
                    if (l_mask & r_mask) == 0 {
                        total_paths += l_qty * r_qty;
                    }
                }
            }
            let mut paths_to_left = left.paths_to_left.clone();
            if let Some(l_mask) = left.inside_mask {
                for (r_mask, r_qty) in right.paths_to_left.iter() {
                    if l_mask & r_mask == 0 {
                        let mut found = false;
                        for i in 0..paths_to_left.len() {
                            if paths_to_left[i].0 == l_mask | r_mask {
                                paths_to_left[i].1 += r_qty;
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            paths_to_left.push((l_mask | r_mask, *r_qty));
                        }
                    }
                }
            }
            let mut paths_to_right = right.paths_to_right.clone();
            if let Some(r_mask) = right.inside_mask {
                for (l_mask, l_qty) in left.paths_to_right.iter() {
                    if l_mask & r_mask == 0 {
                        let mut found = false;
                        for i in 0..paths_to_right.len() {
                            if paths_to_right[i].0 == l_mask | r_mask {
                                paths_to_right[i].1 += l_qty;
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            paths_to_right.push((l_mask | r_mask, *l_qty));
                        }
                    }
                }
            }
            Node {
                inside_mask: total_mask,
                inside_paths: total_paths,
                paths_to_left,
                paths_to_right,
            }
        }

        fn swap(&mut self) {
            swap(&mut self.paths_to_left, &mut self.paths_to_right);
        }
    }

    let graph = Graph::with_biedges(n, &edges);
    let mut st = graph.path_segment_tree_with_gen(true, |vert| Node::new(a[vert]));

    for _ in 0..q {
        let u = input.read_size() - 1;
        let v = input.read_size() - 1;
        out.print_line(st.query(u..=v).inside_paths);
    }
}

pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let t = input.read();
    for _ in 1..=t {
        solve(&mut input, &mut output);
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
