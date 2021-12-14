//{"name":"G. Торговая задача","group":"Codeforces - Codeforces Round #760 (Div. 3)","url":"https://codeforces.com/contest/1618/problem/G","interactive":false,"timeLimit":4500,"tests":[{"input":"3 4 5\n10 30 15\n12 31 14 18\n0 1 2 3 4\n","output":"55\n56\n60\n64\n64\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GTorgovayaZadacha"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let q = input.read();
    let a: Vec<u64> = input.read_vec(n);
    let b: Vec<u64> = input.read_vec(m);
    let k: Vec<u64> = input.read_vec(q);

    let mut cur_ans = a.iter().sum::<u64>();
    let mut things = a
        .into_iter()
        .map(|i| (i, true))
        .chain(b.into_iter().map(|i| (i, false)))
        .collect_vec();

    things.sort_unstable();
    let mut part_sums = vec![(0u64, 0usize)];
    for (i, our) in things.iter().cloned() {
        let (last_sum, last_q) = part_sums.last().unwrap().clone();
        part_sums.push((last_sum + i, last_q + if our { 1 } else { 0 }));
    }
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    enum EventInner {
        Join(usize),
        Query(usize),
    }
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    struct Event {
        at: u64,
        t: EventInner,
    }
    let mut events = Vec::new();
    for i in 1..(n + m) {
        events.push(Event {
            at: things[i].0 - things[i - 1].0,
            t: EventInner::Join(i),
        });
    }
    for (i, j) in k.into_iter().enumerate() {
        events.push(Event {
            at: j,
            t: EventInner::Query(i),
        })
    }
    events.sort_unstable();
    let mut ans = vec![0u64; q];
    let mut dsu = DSU::new(n + m);
    for Event { at: _, t } in events {
        match t {
            EventInner::Join(i) => {
                let left = (i - dsu.size(i - 1), i);
                let right = (i, i + dsu.size(i));
                for (from, to) in [left, right] {
                    let our = part_sums[to].1 - part_sums[from].1;
                    cur_ans -= part_sums[to].0 - part_sums[to - our].0;
                }
                let our = part_sums[right.1].1 - part_sums[left.0].1;
                cur_ans += part_sums[right.1].0 - part_sums[right.1 - our].0;
                dsu.join(i - 1, i);
            }
            EventInner::Query(i) => {
                ans[i] = cur_ans;
            }
        }
    }
    output().print_per_line(&ans);
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
    let time_limit = std::time::Duration::from_millis(4500);
    let mut paths = std::fs::read_dir("./tests/g_torgovaya_zadacha/")
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
fn g_torgovaya_zadacha() {
    assert!(tester::run_tests());
}
