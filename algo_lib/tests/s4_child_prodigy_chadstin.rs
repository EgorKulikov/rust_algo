//{"name":"S4 - Child Prodigy Chadstin","group":"DMOJ - BSSPC '21","url":"https://dmoj.ca/problem/bsspc21s4","interactive":false,"timeLimit":5000,"tests":[{"input":"4 4 2\n1 1 2 2\n3 3 4 4\n","output":"1\n"},{"input":"6 6 4\n1 2 2 6\n2 1 6 2\n1 1 1 1\n5 5 6 6\n","output":"1\n"},{"input":"2 4 1\n1 1 2 2\n","output":"1\n"},{"input":"2 2 3\n1 1 1 1\n1 2 1 2\n2 2 2 2\n","output":"0\n"},{"input":"9 9 4\n4 1 6 3\n1 4 3 6\n4 7 6 9\n7 4 9 6\n","output":"0\n"},{"input":"4 4 2\n1 1 3 4\n4 1 4 4\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"S4ChildProdigyChadstin"}}}

use algo_lib::collections::vec_ext::{Detuple, IncDec};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable6, RecursiveFunction6};
use algo_lib::numbers::gcd::gcd;
use algo_lib::{compress, out_line, zip};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let rect: Vec<(usize, usize, usize, usize)> = input.read_vec(k);

    let (t, l, b, r) = rect.detuple();
    let t = t.dec_by_one();
    let l = l.dec_by_one();
    let (y, (t, b)) = compress!(t, b);
    let (x, (l, r)) = compress!(l, r);
    let mut ans = gcd(n, m);
    for (y, x, t, l, b, r) in [(&y, &x, &t, &l, &b, &r), (&x, &y, &l, &t, &r, &b)] {
        #[derive(Clone)]
        struct Node {
            has_zero: bool,
            delta: usize,
        }

        impl Default for Node {
            fn default() -> Self {
                Self {
                    has_zero: true,
                    delta: 0,
                }
            }
        }
        let mut nodes = vec![Node::default(); 4 * x.len()];
        #[derive(Ord, PartialOrd, Eq, PartialEq)]
        struct Event {
            at: usize,
            remove: bool,
            l: usize,
            r: usize,
        }
        let mut events = Vec::with_capacity(2 * k);
        for (t, b, l, r) in zip!(t.iter(), b.iter(), l.iter(), r.iter()) {
            let t = *t;
            let b = *b;
            let l = *l;
            let r = *r;
            events.push(Event {
                at: t,
                remove: false,
                l,
                r,
            });
            events.push(Event {
                at: b,
                remove: true,
                l,
                r,
            });
        }
        events.sort();
        let mut query = RecursiveFunction6::new(
            |f,
             root: usize,
             left: usize,
             right: usize,
             from: usize,
             to: usize,
             to_remove: bool|
             -> bool {
                if to <= left || right <= from {
                    false
                } else if from <= left && right <= to {
                    if to_remove {
                        nodes[root].delta -= 1;
                        nodes[root].delta == 0 && nodes[root].has_zero
                    } else {
                        nodes[root].delta += 1;
                        nodes[root].delta == 1 && nodes[root].has_zero
                    }
                } else {
                    let mid = (left + right) >> 1;
                    let l_res = f.call(2 * root + 1, left, mid, from, to, to_remove);
                    let r_res = f.call(2 * root + 2, mid, right, from, to, to_remove);
                    nodes[root].has_zero = nodes[2 * root + 1].has_zero
                        && nodes[2 * root + 1].delta == 0
                        || nodes[2 * root + 2].has_zero && nodes[2 * root + 2].delta == 0;
                    nodes[root].delta == 0 && (l_res || r_res)
                }
            },
        );
        for event in events {
            if query.call(0, 0, x.len(), event.l, event.r, event.remove) {
                ans = gcd(ans, y[event.at]);
            }
        }
    }
    out_line!(ans.trailing_zeros());
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
        let time_limit = std::time::Duration::from_millis(5000);
        let mut paths = std::fs::read_dir("./tests/s4_child_prodigy_chadstin/")
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
fn s4_child_prodigy_chadstin() {
    assert!(tester::run_tests());
}
