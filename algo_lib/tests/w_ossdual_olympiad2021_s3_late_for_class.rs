//{"name":"WOSS Dual Olympiad 2021 S3: Late for Class!","group":"DMOJ","url":"https://dmoj.ca/problem/wossoly2021s3","interactive":false,"timeLimit":2000,"tests":[{"input":"7 7\n.....3.\n.......\n.0.....\n.......\n......1\n...2...\n.....4.\n","output":"24\n"},{"input":"10 10\n##########\n#0.....1.#\n#....###.#\n#..3.#2..#\n#....###.#\n#........#\n#..###...#\n#..#4....#\n#..##....#\n##########\n","output":"31\n"},{"input":"17 18\n...#X#.........#x4\n####.#.........###\n#k1..#............\n####.#............\n...#.#....j2......\n...#J#............\n...###............\n..................\n..................\n..................\n...........#......\n...........#..##..\n...........#..#K..\n...........#..##..\n....3......#.####.\n...............0#.\n.........########.\n","output":"66\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"WOSSDualOlympiad2021S3LateForClass"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::dirs::D4;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let r = input.read();
    let c = input.read();
    let grid = input.read_table::<char>(r, c);

    let mut path = [0usize; 5];
    let mut portals = [0usize; 26];
    for i in 0..r {
        for j in 0..c {
            let cur = grid[(i, j)];
            if cur.is_numeric() {
                path[(cur as usize) - ('0' as usize)] = i * c + j;
            }
            if cur.is_lowercase() {
                portals[(cur as usize) - ('a' as usize)] = i * c + j;
            }
        }
    }

    let mut graph = Graph::new(r * c);
    for i in 0..r {
        for j in 0..c {
            let cur = grid[(i, j)];
            if cur == '.' || cur.is_numeric() || cur.is_lowercase() {
                for (nr, nc) in D4::iter(i, j, r, c) {
                    let next = grid[(nr, nc)];
                    if next == '.' || next.is_numeric() {
                        graph.add_edge(i * c + j, Edge::new(nr * c + nc));
                    } else if next.is_uppercase() {
                        graph.add_edge(
                            i * c + j,
                            Edge::new(portals[(next as usize) - ('A' as usize)]),
                        );
                    }
                }
            }
        }
    }
    let mut ans = 0u32;
    for i in 0usize..4 {
        let dist = graph.edge_distances(path[i])[path[i + 1]];
        if dist == std::u32::MAX {
            eprintln!("{}", i);
            out_line!(-1);
            return;
        }
        ans += dist;
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
        let time_limit = std::time::Duration::from_millis(2000);
        let mut paths = std::fs::read_dir("./tests/w_ossdual_olympiad2021_s3_late_for_class/")
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
fn w_ossdual_olympiad2021_s3_late_for_class() {
    assert!(tester::run_tests());
}
