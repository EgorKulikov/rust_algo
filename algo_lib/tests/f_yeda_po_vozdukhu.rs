//{"name":"F. Еда по воздуху","group":"Yandex - SNWS-2022, Round 1","url":"https://contest.yandex.ru/snws2022/contest/23957/problems/F/","interactive":false,"timeLimit":2000,"tests":[{"input":"5 5 1\n0..2.\n.XXX.\n.....\n.....\n.X.X1\n","output":"9\n"},{"input":"2 2 2\n03\n12\n","output":"4\n"},{"input":"1 5 1\n.0X21\n","output":"8\n"},{"input":"3 4 1\nX0XX\nX.X2\n1XXX\n","output":"-1\n"},{"input":"3 9 5\n...1...2.\n..3..4..5\nXX06XXXXX\n","output":"21\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FYedaPoVozdukhu"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::arr5d::Arr5d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let r = input.read_usize();
    let c = input.read_usize();
    let k = input.read_usize();
    let grid = input.read_table::<char>(r, c);

    let mut ans: Arr5d<i16> = Arr5d::new(k + 1, r, c, 2 * r + 1, 2 * c + 1, -1);
    let mut start = None;
    let mut finish = None;
    for i in 0..r {
        for j in 0..c {
            if grid[(i, j)] == '0' {
                start = Some((i, j));
            }
            if grid[(i, j)] == (b'0'.into_usize() + k + 1).into_u8() as char {
                finish = Some((i, j));
            }
        }
    }
    let start = (0, start.unwrap().0, start.unwrap().1, r, c);
    let finish = (k, finish.unwrap().0, finish.unwrap().1, r, c);
    ans[start] = 0;
    let mut q = VecDeque::new();
    q.push_back(start);
    while let Some(next) = q.pop_front() {
        if next == finish {
            out_line!(ans[next]);
            return;
        }
        assert_ne!(ans[next], -1);
        let res = ans[next];
        let (step, cr, cc, dr, dc) = next;
        for i in -1..=1 {
            for j in -1..=1 {
                let delta_r = dr.into_isize() - r.into_isize() + i;
                let delta_c = dc.into_isize() - c.into_isize() + j;
                if delta_r < 0 && cr.into_isize() < -delta_r {
                    continue;
                }
                if delta_c < 0 && cc.into_isize() < -delta_c {
                    continue;
                }
                if delta_r > 0 && cr + delta_r.into_usize() >= r {
                    continue;
                }
                if delta_c > 0 && cc + delta_c.into_usize() >= c {
                    continue;
                }
                let nr = (cr.into_isize() + delta_r).into_usize();
                let nc = (cc.into_isize() + delta_c).into_usize();
                if grid[(nr, nc)] == 'X' {
                    continue;
                }
                let ndr = (delta_r + r.into_isize()).into_usize();
                let ndc = (delta_c + c.into_isize()).into_usize();
                let nstep = if step < k
                    && grid[(nr, nc)] == (b'0'.into_usize() + step + 1).into_u8() as char
                {
                    step + 1
                } else {
                    step
                };
                let key = (nstep, nr, nc, ndr, ndc);
                if ans[key] == -1 {
                    ans[key] = res + 1;
                    q.push_back(key);
                }
            }
        }
    }
    out_line!(-1);
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
    let mut paths = std::fs::read_dir("./tests/f_yeda_po_vozdukhu/")
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
fn f_yeda_po_vozdukhu() {
    assert!(tester::run_tests());
}
