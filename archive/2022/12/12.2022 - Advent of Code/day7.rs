//{"name":"day7","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day7"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::out_line;
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    assert_eq!(input.read_line().as_str(), "$ cd /");
    assert_eq!(input.read_string().as_str(), "$");

    enum File {
        Directory(Vec<(String, String)>, Option<usize>),
        File(usize),
    }

    impl File {
        fn files_mut(&mut self) -> &mut Vec<(String, String)> {
            match self {
                File::Directory(files, _) => files,
                File::File(_) => unreachable!(),
            }
        }

        fn files(&self) -> &Vec<(String, String)> {
            match self {
                File::Directory(files, _) => files,
                File::File(_) => unreachable!(),
            }
        }

        fn set_size(&mut self, size: usize) {
            match self {
                File::Directory(_, sz) => {
                    *sz = Some(size);
                }
                File::File(_) => unreachable!(),
            }
        }
    }

    let mut files = HashMap::new();
    files.insert(String::new(), File::Directory(Vec::new(), None));
    let mut cur = String::new();
    let mut processed = 0;
    let mut total = 1;

    while processed < total {
        let command = input.read_string();

        match command.as_str() {
            "cd" => {
                let dir = input.read_string();
                if dir == ".." {
                    cur = cur[..cur.rfind('/').unwrap()].to_string();
                } else {
                    cur.push('/');
                    cur.push_str(&dir);
                }
                assert_eq!(input.read_string().as_str(), "$");
            }
            "ls" => {
                processed += 1;
                while !input.is_exhausted() {
                    let tp = input.read_string();
                    match tp.as_str() {
                        "$" => break,
                        "dir" => {
                            let name = input.read_string();
                            files
                                .get_mut(&cur)
                                .unwrap()
                                .files_mut()
                                .push((name.clone(), cur.clone() + "/" + &name));
                            files.insert(
                                cur.clone() + "/" + &name,
                                File::Directory(Vec::new(), None),
                            );
                            total += 1;
                        }
                        sz => {
                            let name = input.read_string();
                            let size = sz.parse::<usize>().unwrap();
                            files
                                .get_mut(&cur)
                                .unwrap()
                                .files_mut()
                                .push((name.clone(), cur.clone() + "/" + &name));
                            files.insert(cur.clone() + "/" + &name, File::File(size));
                        }
                    }
                    input.skip_whitespace();
                }
            }
            _ => unreachable!(),
        }
    }

    // let mut ans = 0;
    // const MAX_SIZE: usize = 100_000;
    let mut rec = RecursiveFunction::new(|f, path: String| -> usize {
        let mut size = 0;
        let len = files[&path].files().len();
        for i in 0..len {
            let child = files[&path].files()[i].1.clone();
            match files[&child] {
                File::Directory(_, _) => {
                    size += f.call(child);
                }
                File::File(sz) => {
                    size += sz;
                }
            }
        }

        // if size <= MAX_SIZE {
        //     ans += size;
        // }
        files.get_mut(&path).unwrap().set_size(size);
        size
    });
    let total_size = rec.call(String::new());
    let need = total_size - 40_000_000;
    let mut ans = None;
    for file in files.into_values() {
        if let File::Directory(_, sz) = file {
            if sz.unwrap() >= need {
                ans.minim(sz.unwrap());
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
