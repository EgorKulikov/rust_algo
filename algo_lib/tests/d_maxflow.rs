//{"name":"D - Maxflow","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_d","interactive":false,"timeLimit":5000,"tests":[{"input":"3 3\n#..\n..#\n...\n","output":"3\n#><\nvv#\n^^.\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMaxflow"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dCharWrite, Arr2dRead};
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::dirs::D4;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let n = input.read();
    let m = input.read();
    let mut s: Arr2d<char> = input.read_table(n, m);

    let mut graph = Graph::new(n * m + 2);
    for i in 0..n {
        for j in ((i % 2)..m).step_by(2) {
            if s[(i, j)] == '.' {
                graph.add_edge(n * m, FlowEdge::new(i * m + j, 1));
                for (r, c) in D4::iter(i, j, n, m) {
                    if s[(r, c)] == '.' {
                        graph.add_edge(i * m + j, FlowEdge::new(r * m + c, 1));
                    }
                }
            }
        }
        for j in ((1 - i % 2)..m).step_by(2) {
            if s[(i, j)] == '.' {
                graph.add_edge(i * m + j, FlowEdge::new(n * m + 1, 1));
            }
        }
    }
    let ans = graph.max_flow(n * m, n * m + 1);
    for i in 0..n {
        for j in ((i % 2)..m).step_by(2) {
            for e in graph[i * m + j].iter() {
                if e.flow(&graph) > 0 && e.to() < n * m {
                    let r = e.to() / m;
                    let c = e.to() % m;
                    let (x, y) = if r > i {
                        ('v', '^')
                    } else if r < i {
                        ('^', 'v')
                    } else if c < j {
                        ('<', '>')
                    } else {
                        ('>', '<')
                    };
                    s[(i, j)] = x;
                    s[(r, c)] = y;
                }
            }
        }
    }
    out_line!(ans);
    output().print_table(&s);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

mod tester {
use algo_lib::io::input::Input;
use algo_lib::io::output::{Output, OUTPUT};

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
                    "Token #{} differs, expected {}, actual {}",
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

pub(crate) fn run_tests() -> bool {
    let blue = "\x1B[34m";
    let red = "\x1B[31m";
    let green = "\x1B[32m";
    let yellow = "\x1B[33m";
    let def = "\x1B[0m";
    let time_limit = std::time::Duration::from_millis(5000);
    let mut paths = std::fs::read_dir("./tests/d_maxflow/")
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
                            let is_exhausted = crate::run(input);
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
}
#[test]
fn d_maxflow() {
    assert!(tester::run_tests());
}
