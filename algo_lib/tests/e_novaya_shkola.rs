//{"name":"E. Новая школа","group":"Codeforces - Hello 2022","url":"https://codeforces.com/contest/1621/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n1 1\n30\n3\n25 16 37\n4 2\n9 12 12 6\n2\n4 5\n3\n111 11 11\n","output":"101\n00100\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ENovayaShkola"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::rational::Rational;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let mut a: Vec<u64> = input.read_vec(n);

    a.sort_unstable();
    a.reverse();
    a.resize_with(m, || unreachable!());
    a.reverse();

    let mut groups = Vec::with_capacity(m);
    for _ in 0..m {
        let k = input.read_size();
        let b: Vec<u64> = input.read_vec(k);
        let s = b.iter().sum::<u64>();
        groups.push((Rational::new(s, k.into_u64()), s, k, b));
    }
    let mut ratio = groups.iter().map(|(r, _, _, _)| *r).collect_vec();
    ratio.sort();
    let mut same = BitSet::new(m);
    let mut up = BitSet::new(m);
    let mut down = BitSet::new(m);
    for i in 0..m {
        if ratio[i] <= Rational::new(a[i], 1) {
            same.set(i, true);
        }
        if i > 0 && ratio[i] <= Rational::new(a[i - 1], 1) {
            up.set(i, true);
        }
        if i < m - 1 && ratio[i] <= Rational::new(a[i + 1], 1) {
            down.set(i, true);
        }
    }
    fn count(b: &BitSet) -> Vec<usize> {
        let mut res = Vec::with_capacity(b.len() + 1);
        res.push(0);
        for i in 0..b.len() {
            res.push(*res.last().unwrap() + if b[i] { 1 } else { 0 });
        }
        res
    }
    let c_same = count(&same);
    let c_up = count(&up);
    let c_down = count(&down);
    let mut ans = Str::new();
    for (r, s, k, b) in groups {
        let pos = ratio.as_slice().lower_bound(&r);
        for v in b {
            let cr = Rational::new(s - v, (k - 1).into_u64());
            let mut n_pos = ratio.as_slice().lower_bound(&cr);
            if n_pos > pos {
                n_pos -= 1;
            }
            let l = pos.min(n_pos);
            let r = pos.max(n_pos) + 1;
            if c_same[l] != l || c_same[m] - c_same[r] != m - r || cr > Rational::new(a[n_pos], 1) {
                ans += b'0';
                continue;
            }
            if n_pos < pos && c_down[pos] - c_down[n_pos] != pos - n_pos {
                ans += b'0';
                continue;
            }
            if n_pos > pos && c_up[n_pos + 1] - c_up[pos + 1] != n_pos - pos {
                ans += b'0';
                continue;
            }
            ans += b'1';
        }
    }
    out_line!(ans);
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
        let mut paths = std::fs::read_dir("./tests/e_novaya_shkola/")
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
fn e_novaya_shkola() {
    assert!(tester::run_tests());
}
