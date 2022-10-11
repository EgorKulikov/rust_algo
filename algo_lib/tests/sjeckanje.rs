//{"name":"#5 - Sjeckanje","group":"DMOJ - COCI '20 Contest 5","url":"https://dmoj.ca/problem/coci20c5p5","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3\n1 2 3 4\n1 2 1\n1 1 2\n2 3 1\n","output":"2\n2\n0\n"},{"input":"4 3\n2 0 2 1\n4 4 1\n2 2 3\n1 3 2\n","output":"2\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Sjeckanje"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let q = input.read_usize();
    let a = input.read_long_vec(n);

    const INFTY: i64 = i64::MIN / 3;

    #[derive(Default, Copy, Clone)]
    struct Node {
        delta: i64,
        val: [[i64; 3]; 3],
        length: usize,
    }

    impl SegmentTreeNode for Node {
        fn new(left: usize, right: usize) -> Self {
            let mut res = Self {
                length: right - left,
                ..Default::default()
            };
            if res.length == 1 {
                res.val[1][1] = INFTY;
                res.val[2][2] = INFTY;
            }
            res
        }

        fn join(&mut self, left: &Self, right: &Self, _: usize, _: usize, _: usize) {
            for v in &mut self.val {
                for j in v {
                    *j = INFTY;
                }
            }
            for i in 0..3 {
                for j in 0..3 {
                    for k in 0..3 {
                        let l = (3 - j) % 3;
                        self.val[i][k].maxim(left.val[i][j] + right.val[l][k]);
                    }
                }
            }
        }

        fn accumulate(&mut self, value: &Self, _: usize, _: usize) {
            self.delta += value.delta;
            for i in 0..3 {
                for j in 0..3 {
                    if self.length == 1 && i == j && i > 0 {
                        if i == j && i > 0 {
                            self.val[i][i] = INFTY;
                        }
                    } else {
                        if i == 1 {
                            self.val[i][j] -= value.delta;
                        } else if i == 2 {
                            self.val[i][j] += value.delta;
                        }
                        if j == 1 {
                            self.val[i][j] -= value.delta;
                        } else if j == 2 {
                            self.val[i][j] += value.delta;
                        }
                    }
                }
            }
        }

        fn reset_delta(&mut self, _: usize, _: usize) {
            self.delta = 0;
        }
    }
    let mut st = SegmentTree::from_generator(n, |i| {
        let mut res = Node::new(i, i + 1);
        res.accumulate(
            &Node {
                delta: a[i],
                ..Default::default()
            },
            i,
            i + 1,
        );
        res
    });
    for _ in 0..q {
        let l = input.read_usize() - 1;
        let r = input.read_usize();
        let x = input.read_long();
        // for i in &mut a[l..r] {
        //     *i += x;
        // }
        st.update(
            l..r,
            &Node {
                delta: x,
                ..Default::default()
            },
        );
        /*let mut no_min = None;
        let mut no_max = None;
        let mut ans = 0;
        for &i in &a {
            let mut n_ans = ans;
            if let Some(a) = no_min {
                n_ans.maxim(a - i);
            }
            no_min.maxim(ans + i);
            if let Some(a) = no_max {
                n_ans.maxim(a + i);
            }
            no_max.maxim(ans - i);
            ans = n_ans;
        }
        out_line!(ans);*/
        out_line!(st.query(..).val[0][0]);
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
        let time_limit = std::time::Duration::from_millis(2000);
        let mut paths = std::fs::read_dir("./tests/sjeckanje/")
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
fn sjeckanje() {
    assert!(tester::run_tests());
}
