//{"name":"E. Запросы частот","group":"Codeforces - Codeforces Round #759 (Div. 2, основан на Отборочном раунде 3 Технокубка 2022)","url":"https://codeforces.com/contest/1591/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n3 3\n1 1 1\n1 2\n3 1 1\n3 1 2\n3 2 1\n5 5\n1 2 1 1 2\n1 1 2 2\n3 1 1\n2 1 2\n4 1 1\n4 2 1\n4 2 2\n","output":"1 -1 1\n1 1 2 1 -1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EZaprosiChastot"}}}

use algo_lib::collections::treap_map::TreapSet;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::{out, out_line};
use std::collections::{BTreeSet, HashSet};
use std::thread;

fn solve(input: &mut Input, _test_case: usize) {
    let t = input.read();
    let mut tests = Vec::new();
    for _ in 0..t {
        let n = input.read();
        let q = input.read();
        let a: Vec<i32> = input.read_vec(n);
        let p = input.read_vec::<usize>(n - 1).dec_by_one();
        let queries: Vec<(usize, usize, usize)> = input.read_vec(q);
        tests.push((n, q, a, p, queries));
    }
    thread::Builder::new()
        .stack_size(400000000)
        .spawn(move || {
            for (n, q, a, p, queries) in tests {
                let mut by_vertex = vec![Vec::new(); n];
                for (i, (v, _, _)) in queries.iter().enumerate() {
                    by_vertex[*v - 1].push(i);
                }
                let mut graph = Graph::new(n);
                for i in 1..n {
                    graph.add_edge(p[i - 1], Edge::new(i));
                }
                let mut ans = vec![0; q];
                let mut set = TreapSet::new();
                let mut qty = vec![0isize; n + 1];
                let mut once = BTreeSet::new();
                let mut dfs = RecursiveFunction::new(|f, vert: usize| {
                    let was = qty[a[vert] as usize];
                    if was < -1 {
                        set.remove(&(was, a[vert]));
                    } else if was == -1 {
                        once.remove(&a[vert]);
                    }
                    qty[a[vert] as usize] -= 1;
                    if was < 0 {
                        set.insert((was - 1, a[vert]));
                    } else {
                        once.insert(a[vert]);
                    }
                    for i in by_vertex[vert].drain(..) {
                        let (_, l, mut k) = queries[i];
                        if l == 1 {
                            if k <= once.len() {
                                ans[i] = *once.iter().next().unwrap();
                                continue;
                            }
                            k -= once.len();
                        }
                        let qt = set.lower_bound(&(-(l as isize), (n + 1) as i32));
                        if k > qt {
                            ans[i] = -1;
                        } else {
                            let (_, a) = set.get_at(qt - k).unwrap();
                            ans[i] = *a;
                        }
                    }
                    for e in graph[vert].iter() {
                        f.call(e.to());
                    }
                    if was < 0 {
                        set.remove(&(was - 1, a[vert]));
                    } else {
                        once.remove(&a[vert]);
                    }
                    if was < -1 {
                        set.insert((was, a[vert]));
                    } else if was == -1 {
                        once.insert(a[vert]);
                    }
                    qty[a[vert] as usize] += 1;
                });
                dfs.call(0);
                out_line!(ans);
            }
        })
        .unwrap()
        .join()
        .unwrap();
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
        let time_limit = std::time::Duration::from_millis(4000);
        let mut paths = std::fs::read_dir("./tests/e_zaprosi_chastot/")
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
fn e_zaprosi_chastot() {
    assert!(tester::run_tests());
}
