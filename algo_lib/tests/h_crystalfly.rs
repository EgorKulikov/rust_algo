//{"name":"H. Crystalfly","group":"Yandex - Stage 9: Grand Prix of Nanjing","url":"https://official.contest.yandex.ru/opencupXXII/contest/33444/problems/H/","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n5\n1 10 100 1000 10000\n1 2 1 1 1\n1 2\n1 3\n2 4\n2 5\n5\n1 10 100 1000 10000\n1 3 1 1 1\n1 2\n1 3\n2 4\n2 5\n","output":"10101\n10111\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HCrystalfly"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a: Vec<i64> = input.read_vec(n);
    let t: Vec<u8> = input.read_vec(n);
    let edges: Vec<(usize, usize)> = input.read_vec(n - 1);

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u - 1, BiEdge::new(v - 1));
    }
    let mut dfs = RecursiveFunction2::new(|f, vert, prev| -> (i64, i64, i64) {
        let mut take = a[vert];
        let mut take_and_return = a[vert];
        let mut calls = Vec::new();
        for e in graph[vert].iter() {
            if e.to() == prev {
                continue;
            }
            calls.push((f.call(e.to(), vert), e.to()));
        }
        let mut max_delta_take = 0;
        let mut max_delta_take_and_return: i64 = 0;
        let mut second_delta_take_and_return: i64 = 0;
        let mut sum_not_take: i64 = 0;
        for ((c_take, c_not_take, c_take_and_return), _) in calls.iter() {
            take += c_not_take;
            sum_not_take += c_not_take;
            max_delta_take.maxim(c_take - c_not_take);
            take_and_return += c_not_take;
            let cur_delta_take_and_return = c_take_and_return - c_not_take;
            if cur_delta_take_and_return > max_delta_take_and_return {
                second_delta_take_and_return = max_delta_take_and_return;
                max_delta_take_and_return = cur_delta_take_and_return;
            } else {
                second_delta_take_and_return.maxim(cur_delta_take_and_return);
            }
        }
        take += max_delta_take;
        for ((c_take, c_not_take, c_take_and_return), i) in calls {
            if t[i] == 3 {
                if c_take_and_return - c_not_take == max_delta_take_and_return {
                    take.maxim(
                        a[vert] + c_take + sum_not_take - c_not_take + second_delta_take_and_return,
                    );
                } else {
                    take.maxim(
                        a[vert] + c_take + sum_not_take - c_not_take + max_delta_take_and_return,
                    );
                }
            }
        }
        (take, take - a[vert], take_and_return)
    });
    out_line!(dfs.call(0, 0).0);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
        let mut paths = std::fs::read_dir("./tests/h_crystalfly/")
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
fn h_crystalfly() {
    assert!(tester::run_tests());
}
