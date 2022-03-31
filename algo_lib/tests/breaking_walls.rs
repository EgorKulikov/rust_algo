//{"name":"Breaking walls","group":"HackerEarth - January Circuits '22","url":"https://www.hackerearth.com/challenges/competitive/january-circuits-022/algorithm/break-the-wall-acf543a5/","interactive":false,"timeLimit":2000,"tests":[{"input":" 5 5\n *.#**\n *.#*.\n #####\n **#*.\n **#..\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BreakingWalls"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::dsu2d::DSU2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::dirs::Directions;
use algo_lib::{const_value_ref, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let grid = input.read_table::<char>(n, m);

    let mut dsu = DSU2d::new(n, m);
    for i in 0..n {
        for j in 0..m {
            if grid[(i, j)] == '#' {
                continue;
            }
            if i + 1 < n && grid[(i + 1, j)] != '#' {
                dsu.join(i, j, i + 1, j);
            }
            if j + 1 < m && grid[(i, j + 1)] != '#' {
                dsu.join(i, j, i, j + 1);
            }
        }
    }
    let mut stars = Arr2d::new(n, m, 0);
    for i in 0..n {
        for j in 0..m {
            if grid[(i, j)] == '*' {
                stars[dsu.get(i, j)] += 1;
            }
        }
    }
    const_value_ref!(
        Dirs DIRS_INNER: [(isize, isize); 4] as [(isize, isize)] = [
            (0isize, 2isize),
            (2isize, 0isize),
            (1isize, 1isize),
            (-1isize, 1isize),
        ]
    );
    type D4 = Directions<Dirs>;
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[(i, j)] == '#' {
                continue;
            }
            let par = dsu.get(i, j);
            ans.maxim(stars[par]);
            for other in D4::iter(i, j, n, m) {
                if grid[other] != '#' {
                    let c_par = dsu.get(other.0, other.1);
                    if c_par != par {
                        ans.maxim(stars[par] + stars[c_par]);
                    }
                }
            }
        }
    }
    out_line!(ans);
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
        let mut paths = std::fs::read_dir("./tests/breaking_walls/")
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
fn breaking_walls() {
    assert!(tester::run_tests());
}
