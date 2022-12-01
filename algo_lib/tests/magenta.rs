//{"name":"#3 - Magenta","group":"DMOJ - COCI '20 Contest 5","url":"https://dmoj.ca/problem/coci20c5p3","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 3\n3 2 magenta\n2 1 magenta\n","output":"Paula\n"},{"input":"5\n3 5\n1 2 magenta\n1 3 magenta\n2 4 plava\n2 5 crvena\n","output":"Marin\n"},{"input":"5\n1 4\n2 1 plava\n1 3 crvena\n5 2 plava\n4 1 magenta\n","output":"Magenta\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Magenta"}}}

use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;
use std::mem::swap;

fn solve(input: &mut Input) {
    let n = input.read_size();
    let mut a = input.read_size() - 1;
    let mut b = input.read_size() - 1;
    let edges: Vec<(usize, usize, String)> = input.read_vec(n - 1);

    let mut graph = Graph::new(n);
    for (u, v, color) in edges {
        graph.add_edge(
            u - 1,
            BiWeightedEdge::new(
                v - 1,
                if &color != "crvena" { 1 } else { 0 } + if &color != "plava" { 2 } else { 0 },
            ),
        );
    }
    let mut can_move_paula: Box<dyn Fn(&BiWeightedEdge<i32>) -> bool> =
        Box::new(|e| e.weight().is_set(0));
    let mut can_move_marin: Box<dyn Fn(&BiWeightedEdge<i32>) -> bool> =
        Box::new(|e| e.weight().is_set(1));
    let mut paula = "Paula";
    let mut marin = "Marin";
    if !graph[a].iter().any(|e| can_move_paula(e) && e.to() != b) {
        out_line!(marin);
        return;
    }
    if !graph[b].iter().any(&can_move_marin) {
        out_line!(paula);
        return;
    }
    let lca = graph.lca();
    while lca.path_length(a, b) > 1 {
        let cur_len = lca.path_length(a, b);
        let mut next = None;
        for e in graph[a].iter() {
            if cur_len > lca.path_length(e.to(), b) {
                if can_move_paula(e) {
                    next = Some(e.to());
                }
                break;
            }
        }
        match next {
            None => break,
            Some(new_a) => {
                a = b;
                b = new_a;
                swap(&mut paula, &mut marin);
                swap(&mut can_move_paula, &mut can_move_marin);
            }
        }
    }
    if lca.path_length(a, b) % 2 == 0 {
        out_line!("Magenta");
        return;
    }
    let mut c = a;
    while c != b {
        for e in &graph[c] {
            if lca.path_length(c, b) > lca.path_length(e.to(), b) {
                if !can_move_marin(e) {
                    out_line!("Magenta");
                    return;
                }
                c = e.to();
                break;
            }
        }
    }
    let mut dfs = RecursiveFunction3::new(|f, vert, prev, good| {
        for e in graph[vert].iter() {
            if e.to() == prev || !can_move_paula(e) {
                continue;
            }
            if good || f.call(e.to(), vert, !can_move_marin(e)) {
                return true;
            }
        }
        false
    });
    if dfs.call(a, b, false) {
        out_line!("Magenta");
    } else {
        out_line!(marin);
    }
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
        let time_limit = std::time::Duration::from_millis(1000);
        let mut paths = std::fs::read_dir("./tests/magenta/")
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
                                    match err.downcast::<&str>() {
                                        Ok(as_string) => println!(
                                            "{}Verdict: {}RuntimeError ({:?}){}",
                                            blue, red, as_string, def
                                        ),
                                        Err(err) => println!(
                                            "{}Verdict: {}RuntimeError ({:?}){}",
                                            blue, red, err, def
                                        ),
                                    }
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
fn magenta() {
    assert!(tester::run_tests());
}
