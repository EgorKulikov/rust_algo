//Codeforces - UTPC Contest 12-03-21 Div. 1 (Advanced)
//{"type":"stdin","fileName":null,"pattern":null}
//{"type":"stdout","fileName":null,"pattern":null}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::dsu::DSU;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, Output, OUTPUT};
use algo_lib::{out, out_line};
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let r = input.read();
    let c = input.read();
    let maze = input.read_table::<char>(r, c);

    let mut graph = Graph::new(r * c);
    let mut gt = Graph::new(r * c);
    let id = |i, j| i * c + j;
    for i in 0..r {
        for j in 0..c {
            match maze[(i, j)] {
                '>' => {
                    if j + 1 < c {
                        graph.add_edge(id(i, j + 1), Edge::new(id(i, j)));
                    }
                }
                '<' => {
                    if j > 0 {
                        graph.add_edge(id(i, j - 1), Edge::new(id(i, j)));
                    }
                }
                '^' => {
                    if i > 0 {
                        graph.add_edge(id(i - 1, j), Edge::new(id(i, j)));
                    }
                }
                'v' => {
                    if i + 1 < r {
                        graph.add_edge(id(i + 1, j), Edge::new(id(i, j)));
                    }
                }
                _ => {}
            }
        }
    }
    for i in 0..graph.vertex_count() {
        for e in graph[i].iter() {
            gt.add_edge(e.to(), Edge::new(i));
        }
    }
    let mut ans = 0;
    let mut q = VecDeque::new();
    let mut w = BitSet::new(graph.vertex_count());
    for i in 0..graph.vertex_count() {
        if graph[i].is_empty() {
            q.push_back(i);
        }
    }
    while let Some(cur) = q.pop_front() {
        w.set(cur, true);
        for e in gt[cur].iter() {
            let mut has = false;
            for f in graph[e.to()].iter() {
                if !w[f.to()] {
                    has = true;
                    break;
                }
            }
            if !has {
                q.push_back(e.to());
            }
        }
    }
    let mut dsu = DSU::new(graph.vertex_count());
    for i in 0..graph.vertex_count() {
        if w[i] {
            continue;
        }
        for e in graph[i].iter() {
            if !w[e.to()] {
                dsu.join(i, e.to());
            }
        }
    }
    for i in dsu.iter() {
        let sz = dsu.size(i) as u64;
        ans += sz * (sz - 1) / 2;
    }
    out_line!(ans);
}

//START SKIP
//END SKIP

fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START SKIP
fn check(expected: &mut &[u8], actual: &mut &[u8]) -> Result<(), String> {
    let mut expected = Input::new(expected);
    let mut actual = Input::new(actual);
    let mut token_num = 0usize;
    loop {
        let expected_token = expected.next_token();
        let actual_token = actual.next_token();
        if expected_token != actual_token {
            if expected_token.is_none() {
                return Err(format!("Expected has only {} tokens", token_num));
            } else if actual_token.is_none() {
                return Err(format!("Actual has only {} tokens", token_num));
            } else {
                return Err(format!(
                    "Token #{} differs, expectes {}, actual {}",
                    token_num,
                    String::from_utf8(expected_token.unwrap()).unwrap(),
                    String::from_utf8(actual_token.unwrap()).unwrap()
                ));
            }
        }
        token_num += 1;
        if actual_token.is_none() {
            break;
        }
    }
    Ok(())
}

static mut OUT: Vec<u8> = Vec::new();

struct WriteDelegate {}

impl std::io::Write for WriteDelegate {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unsafe {
            OUT.append(&mut Vec::from(buf));
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn run_tests() -> bool {
    let blue = "\x1B[34m";
    let red = "\x1B[31m";
    let green = "\x1B[32m";
    let yellow = "\x1B[33m";
    let def = "\x1B[0m";
    let time_limit = std::time::Duration::from_millis(1500);
    let mut paths = std::fs::read_dir("./tests/h_maze_escape_pt_i_i/")
        .unwrap()
        .map(|res| res.unwrap())
        .collect::<Vec<_>>();
    paths.sort_by(|a, b| a.path().cmp(&b.path()));
    let mut test_failed = 0usize;
    let mut test_total = 0usize;
    for path in paths {
        let sub_path = path;
        if sub_path.file_type().unwrap().is_file() {
            let path = sub_path.path();
            match path.extension() {
                None => {}
                Some(extension) => {
                    if extension.to_str() == Some("in") {
                        println!("=====================================================");
                        test_total += 1;
                        let name = path.file_name().unwrap().to_str().unwrap();
                        let name = &name[..name.len() - 3];
                        println!("{}Test {}{}", blue, name, def);
                        println!("{}Input:{}", blue, def);
                        println!("{}", std::fs::read_to_string(&path).unwrap());
                        let expected = match std::fs::read_to_string(
                            path.parent().unwrap().join(format!("{}.out", name)),
                        ) {
                            Ok(res) => Some(res),
                            Err(_) => None,
                        };
                        println!("{}Expected:{}", blue, def);
                        match &expected {
                            None => {
                                println!("{}Not provided{}", yellow, def);
                            }
                            Some(expected) => {
                                println!("{}", expected);
                            }
                        }
                        println!("{}Output:{}", blue, def);
                        match std::panic::catch_unwind(|| {
                            unsafe {
                                OUT.clear();
                            }
                            let mut file = std::fs::File::open(&path).unwrap();
                            let input = Input::new(&mut file);
                            let started = std::time::Instant::now();
                            unsafe {
                                OUTPUT = Some(Output::new(Box::new(WriteDelegate {})));
                            }
                            let is_exhausted = run(input);
                            let res = started.elapsed();
                            let output;
                            unsafe {
                                output = OUT.clone();
                            }
                            println!("{}", String::from_utf8_lossy(&output));
                            (output, res, is_exhausted)
                        }) {
                            Ok((output, duration, is_exhausted)) => {
                                println!(
                                    "{}Time elapsed: {:.3}s{}",
                                    blue,
                                    (duration.as_millis() as f64) / 1000.,
                                    def,
                                );
                                if !is_exhausted {
                                    println!("{}Input not exhausted{}", red, def);
                                }
                                if let Some(expected) = expected {
                                    let mut expected_bytes = expected.as_bytes().clone();
                                    match check(&mut expected_bytes, &mut &output[..]) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            println!(
                                                "{}Verdict: {}Wrong Answer ({}){}",
                                                blue, red, err, def
                                            );
                                            test_failed += 1;
                                            continue;
                                        }
                                    }
                                }
                                if duration > time_limit {
                                    test_failed += 1;
                                    println!("{}Verdict: {}Time Limit{}", blue, red, def);
                                } else {
                                    println!("{}Verdict: {}OK{}", blue, green, def)
                                }
                            }
                            Err(err) => {
                                test_failed += 1;
                                println!(
                                    "{}Verdict: {}RuntimeError ({:#?}){}",
                                    blue, red, err, def
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    if test_failed == 0 {
        println!(
            "{}All {}{}{} tests passed{}",
            blue, green, test_total, blue, def
        );
    } else {
        println!(
            "{}{}/{}{} tests failed{}",
            red, test_failed, test_total, blue, def
        );
    }
    test_failed == 0
}
//END SKIP
#[test]
fn h_maze_escape_pt_i_i() {
    assert!(run_tests());
}
