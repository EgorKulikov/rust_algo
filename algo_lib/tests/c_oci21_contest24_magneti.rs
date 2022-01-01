//{"name":"COCI '21 Contest 2 #4 Magneti","group":"DMOJ","url":"https://dmoj.ca/problem/coci21c2p4","interactive":false,"timeLimit":1000,"tests":[{"input":"1 10\n10\n","output":"10\n"},{"input":"4 4\n1 1 1 1\n","output":"24\n"},{"input":"3 4\n1 2 1\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"COCI21Contest24Magneti"}}}

use algo_lib::collections::arr3d::Arr3d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable4, RecursiveFunction4};
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let l: usize = input.read();
    let mut r: Vec<usize> = input.read_vec(n);

    type Mod = ModInt7;

    if n == 1 {
        out_line!(l);
        return;
    }
    r.sort_unstable();
    r.reverse();

    let mut d = Vec::new();
    for i in 0..n {
        d.push(Arr3d::new(i + 1, l - n + 1, 4, None));
    }
    let c: Combinations<Mod> = Combinations::new(l + 1);
    let mut rec =
        RecursiveFunction4::new(|f, at: usize, pairs: usize, rem: usize, edges: usize| {
            assert!(at >= pairs);
            match d[at][(pairs, rem, edges)] {
                Some(val) => val,
                None => {
                    if at == n - 1 {
                        let res = if edges == 3 && pairs == 2 || edges != 0 && pairs == 1 {
                            c.c(rem + n, n)
                        } else {
                            Mod::zero()
                        };
                        d[at][(pairs, rem, edges)] = Some(res);
                        res
                    } else {
                        let mut res = Mod::zero();
                        for i in 0..2 {
                            if !edges.is_set(i) {
                                if pairs > 0 && !edges.is_set(1 - i) || pairs > 1 {
                                    res += f.call(at + 1, pairs, rem, edges | (1 << i));
                                }
                                if rem >= r[at] - 1 {
                                    res += f.call(
                                        at + 1,
                                        pairs + 1,
                                        rem - (r[at] - 1),
                                        edges | (1 << i),
                                    );
                                }
                            }
                        }
                        let free_ends = (3 - edges).count_ones() as usize;
                        if rem >= 2 * (r[at] - 1) {
                            assert!(pairs > 0 || edges != 3);
                            res += f.call(at + 1, pairs + 1, rem - 2 * (r[at] - 1), edges)
                                * Mod::from_index(free_ends + pairs - 1);
                        }
                        if rem >= r[at] - 1 && pairs > 0 {
                            assert!(pairs + free_ends >= 2);
                            res += f.call(at + 1, pairs, rem - (r[at] - 1), edges)
                                * Mod::from_index(2 * (pairs - 1) + free_ends);
                        }
                        if pairs >= 2 && edges != 3 || pairs >= 3 {
                            res +=
                                f.call(at + 1, pairs - 1, rem, edges) * Mod::from_index(pairs - 1);
                        }
                        d[at][(pairs, rem, edges)] = Some(res);
                        res
                    }
                }
            }
        });
    out_line!(rec.call(0, 0, l - n, 0));
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
        let mut paths = std::fs::read_dir("./tests/c_oci21_contest24_magneti/")
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
fn c_oci21_contest24_magneti() {
    assert!(tester::run_tests());
}
