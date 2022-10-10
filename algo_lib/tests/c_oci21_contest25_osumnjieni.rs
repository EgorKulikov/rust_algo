//{"name":"COCI '21 Contest 2 #5 Osumnjičeni","group":"DMOJ","url":"https://dmoj.ca/problem/coci21c2p5","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n1 1\n1 1\n3\n1 1\n2 2\n1 2\n","output":"1\n1\n2\n"},{"input":"3\n1 1\n2 2\n3 3\n3\n1 1\n2 3\n1 3\n","output":"1\n1\n1\n"},{"input":"5\n1 3\n3 3\n4 6\n2 3\n1 1\n3\n1 4\n3 5\n1 5\n","output":"3\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"COCI21Contest25Osumnjieni"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{compress, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let range: Vec<(u32, u32)> = input.read_vec(n);

    let l = range.iter().map(|(i, _)| *i).collect_vec();
    let r = range.into_iter().map(|(_, j)| j).collect_vec();
    let (v, (l, r)) = compress!(l, r);
    let x = v.len();

    #[derive(Copy, Clone)]
    struct Node {
        val: usize,
        delta: usize,
    }

    impl SegmentTreeNode for Node {
        fn new(_: usize, _: usize) -> Self {
            Node {
                val: usize::MAX,
                delta: usize::MAX,
            }
        }

        fn join(&mut self, left: &Self, right: &Self, _: usize, _: usize, _: usize) {
            self.val = left.val.min(right.val);
        }

        fn accumulate(&mut self, value: &Self, _: usize, _: usize) {
            self.val.minim(value.delta);
            self.delta.minim(value.delta);
        }

        fn reset_delta(&mut self, _: usize, _: usize) {
            self.delta = usize::MAX;
        }
    }

    let mut tree = SegmentTree::from_generator(x, |_| Node {
        val: n,
        delta: usize::MAX,
    });
    let mut next = vec![0; n];
    for i in (0..n).rev() {
        next[i] = tree.query(l[i], r[i] + 1).val;
        if i != n - 1 {
            let val = next[i + 1];
            next[i].minim(val);
        }
        tree.update(l[i], r[i] + 1, &Node { val: n, delta: i });
    }
    let mut levels = vec![next];
    while levels.last().unwrap()[0] != n {
        let last = levels.last().unwrap();
        let mut next = Vec::with_capacity(n);
        for i in 0..n {
            if last[i] == n {
                break;
            }
            next.push(last[last[i]]);
        }
        next.resize(n, n);
        levels.push(next);
    }

    let q = input.read();
    for _ in 0..q {
        let mut p = input.read::<usize>() - 1;
        let q = input.read();
        let mut ans = 1;
        for i in (0..levels.len()).rev() {
            if levels[i][p] < q {
                ans += 1 << i;
                p = levels[i][p];
            }
        }
        out_line!(ans);
    }
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
        let time_limit = std::time::Duration::from_millis(1000);
        let mut paths = std::fs::read_dir("./tests/c_oci21_contest25_osumnjieni/")
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
fn c_oci21_contest25_osumnjieni() {
    assert!(tester::run_tests());
}
