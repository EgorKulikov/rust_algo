//{"name":"A. Analytics","group":"Yandex - SNWS-2022, Round 4","url":"https://contest.yandex.ru/snws2022/contest/23960/problems/A/","interactive":false,"timeLimit":2000,"tests":[{"input":"6 7 8\n1 2\n3 5\n2 4\n2 4\n2 6\n1 5\n2 1\n1 1 7 1\n1 1 7 2\n1 1 7 3\n1 1 7 4\n1 1 7 5\n1 1 7 6\n2 4 5 2\n1 1 1 1\n","output":"6\n3\n5\n4\n2\n1\n4\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAnalytics"}}}

use algo_lib::collections::permutation::Permutation;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let q = input.read_usize();
    let changes = input.read_usize_pair_vec(m).dec_by_one();

    const BUBEN: usize = 350;
    struct Part<'s> {
        from: usize,
        to: usize,
        changes: &'s [(usize, usize)],
        p: Permutation,
        q: Permutation,
    }

    impl<'s> Part<'s> {
        fn new(from: usize, to: usize, n: usize, changes: &'s [(usize, usize)]) -> Self {
            let changes = &changes[from..to];
            let mut p = Permutation::new_ident(n);
            for &(a, b) in changes {
                p.swap(a, b);
            }
            let q = p.inv();
            Self {
                from,
                to,
                changes,
                p,
                q,
            }
        }

        fn direct(&self, start: usize, end: usize, mut cur: usize) -> usize {
            if end <= self.from || start >= self.to {
                cur
            } else if start <= self.from && self.to <= end {
                self.q[cur]
            } else {
                for i in start.max(self.from)..end.min(self.to) {
                    let (a, b) = self.changes[i - self.from];
                    if a == cur {
                        cur = b;
                    } else if b == cur {
                        cur = a;
                    }
                }
                cur
            }
        }

        fn reverse(&self, start: usize, end: usize, mut cur: usize) -> usize {
            if end <= self.from || start >= self.to {
                cur
            } else if start <= self.from && self.to <= end {
                self.p[cur]
            } else {
                for i in (start.max(self.from)..end.min(self.to)).rev() {
                    let (a, b) = self.changes[i - self.from];
                    if a == cur {
                        cur = b;
                    } else if b == cur {
                        cur = a;
                    }
                }
                cur
            }
        }
    }

    let mut parts = Vec::new();
    for i in (0..m).step_by(BUBEN) {
        parts.push(Part::new(i, (i + BUBEN).min(m), n, &changes));
    }

    for _ in 0..q {
        let tp = input.read_usize();
        let f = input.read_usize() - 1;
        let t = input.read_usize();
        let mut x = input.read_usize() - 1;

        if tp == 2 {
            for i in (f / BUBEN)..=((t - 1) / BUBEN) {
                x = parts[i].direct(f, t, x);
            }
        } else {
            for i in ((f / BUBEN)..=((t - 1) / BUBEN)).rev() {
                x = parts[i].reverse(f, t, x);
            }
        }
        out_line!(x + 1);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
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
    let mut paths = std::fs::read_dir("./tests/a_analytics/")
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
fn a_analytics() {
    assert!(tester::run_tests());
}
