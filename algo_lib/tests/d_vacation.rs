//{"name":"D: Vacation","group":"Facebook Coding Competitions - Facebook Hacker Cup 2021 Final Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2021/final-round/problems/D","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n4 90\n10 20 30 40\n1 2 2\n4 91\n10 20 30 40\n1 2 2\n4 101\n10 20 30 40\n1 2 2\n5 222\n1 1 100 100 100\n1 1 2 2\n8 35\n3 5 7 2 4 8 1 6\n1 2 1 4 4 1 4\n8 36\n3 5 7 2 4 8 1 6\n1 2 1 4 4 1 4\n","output":"Case #1: 0\nCase #2: 1\nCase #3: -1\nCase #4: 1\nCase #5: 1\nCase #6: 2\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"vacation_.*input[.]txt"},"output":{"type":"file","fileName":"vacation_output.txt","pattern":null},"languages":{"java":{"taskClass":"DVacation"}}}

use algo_lib::collections::indexed_heap::IndexedHeap;
use algo_lib::collections::iter_ext::IterOrdExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        k: u64,
        c: Vec<u64>,
        p: Vec<usize>,
        ans: i32,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.k = input.read();
            self.c = input.read_vec(self.n);
            self.p = input.read_vec(self.n - 1).dec_by_one();
        }

        fn solve(&mut self) {
            let mut graph = Graph::new(self.n);
            let n = self.n;
            for i in 1..n {
                graph.add_edge(i, BiEdge::new(self.p[i - 1]));
            }
            let mut dist = vec![0u64; n];
            let mut dfs = RecursiveFunction2::new(|f, vert, prev| {
                dist[vert] = dist[prev] + self.c[vert];
                for e in graph[vert].iter() {
                    if e.to() != prev {
                        f.call(e.to(), vert);
                    }
                }
            });
            dfs.call(0, 0);
            let root = dist.iter().max_position().unwrap();
            let mut prev = vec![0usize; n];
            let mut next = vec![0usize; n];
            let mut dfs = RecursiveFunction2::new(|f, vert, p| {
                dist[vert] = 0;
                prev[vert] = p;
                next[vert] = vert;
                for e in graph[vert].iter() {
                    if e.to() != p {
                        let call = f.call(e.to(), vert);
                        if call > dist[vert] {
                            dist[vert] = call;
                            next[vert] = e.to();
                        }
                    }
                }
                dist[vert] += self.c[vert];
                dist[vert]
            });
            dfs.call(root, root);
            let mut heap = IndexedHeap::new(n);
            for e in graph[root].iter() {
                heap.add_or_adjust(e.to(), -(dist[e.to()] as i64));
            }
            let mut cur = self.c[root] as i64;
            let mut ans = 0usize;
            while cur < self.k as i64 && !heap.is_empty() {
                ans += 1;
                let mut c = Vec::new();
                for _ in 0..2 {
                    if let Some(head) = heap.pop() {
                        c.push(head);
                    }
                }
                for (mut vert, c) in c {
                    cur -= c;
                    while next[vert] != vert {
                        for e in graph[vert].iter() {
                            if e.to() != prev[vert] && e.to() != next[vert] {
                                heap.add_or_adjust(e.to(), -(dist[e.to()] as i64));
                            }
                        }
                        vert = next[vert];
                    }
                }
            }
            if cur >= self.k as i64 {
                self.ans = (ans.max(1) as i32) - 1;
            } else {
                self.ans = -1;
            }
        }

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!("Case #{}: {}", test_case, self.ans));
        }
    }

    run_parallel::<Job>(input);
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
        let time_limit = std::time::Duration::from_millis(360000);
        let mut paths = std::fs::read_dir("./tests/d_vacation/")
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
fn d_vacation() {
    assert!(tester::run_tests());
}
