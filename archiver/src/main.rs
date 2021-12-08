use algo_lib::io::input::Input;
use chrono::{Datelike, Utc};
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let mut res = Vec::new();
    for line in io::BufReader::new(file).lines() {
        res.push(line.unwrap());
    }
    res
}

fn write_lines(filename: &str, lines: Vec<String>) {
    let mut file = File::create(filename).unwrap();
    for line in lines {
        file.write(line.as_bytes()).unwrap();
        file.write("\n".as_bytes()).unwrap();
    }
    file.flush().unwrap();
}

fn get_input(input: &mut Input) -> bool {
    loop {
        let answer = input.read::<String>();
        if answer.starts_with("y") {
            break true;
        }
        if answer.starts_with("n") {
            break false;
        }
        println!("can't parse, please repeat");
    }
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let filter = args.next().unwrap_or("".to_string());
    let lines = read_lines("../Cargo.toml");
    let mut stdin = std::io::stdin();
    let mut input = Input::new(&mut stdin);
    let mut result = Vec::new();
    for line in lines {
        let or_line = line.clone();
        if !line.starts_with("    ") {
            result.push(or_line);
            continue;
        }
        let line = line.trim().as_bytes();
        let task =
            String::from_utf8_lossy(&(line[1..line.len() - 2].iter().cloned().collect::<Vec<_>>()))
                .to_string();
        let task = task.as_str();
        if task == "archiver"
            || task == "listener"
            || task == "algo_lib"
            || task == "main"
            || task == "advent"
        {
            result.push(or_line);
            continue;
        }
        if !task.starts_with(filter.as_str()) {
            result.push(or_line);
            continue;
        }
        println!("task {}", task);
        println!("skip?");
        let skip = get_input(&mut input);
        if skip {
            result.push(or_line);
            continue;
        }
        println!("archive?");
        let archive = get_input(&mut input);
        if archive {
            let now = Utc::now();
            let lines = read_lines(format!("../{}/src/main.rs", task));
            let path = format!(
                "../archive/{}/{}/{}.{}.{} - {}",
                now.year(),
                now.month(),
                now.day(),
                now.month(),
                now.year(),
                String::from_utf8_lossy(
                    &lines[0]
                        .as_bytes()
                        .iter()
                        .skip(2)
                        .cloned()
                        .collect::<Vec<_>>()
                )
                .to_string()
            );
            let path = path.replace(":", "_");
            std::fs::create_dir_all(path.clone()).unwrap();
            write_lines(format!("{}/{}.rs", path, task).as_str(), lines.clone());
            println!("test?");
            let test = get_input(&mut input);
            if test {
                let mut test_lines = Vec::new();
                let mut in_main = false;
                for mut line in lines {
                    if line
                        .trim()
                        .starts_with("let mut paths = std::fs::read_dir(")
                    {
                        line = format!(
                            "    let mut paths = std::fs::read_dir(\"./tests/{}/\")",
                            task,
                        );
                    }
                    if line == "//BEGIN MAIN".to_string() {
                        in_main = true;
                    }
                    if !in_main {
                        test_lines.push(line.clone());
                    }
                    if line == "//END MAIN".to_string() {
                        in_main = false;
                    }
                }
                test_lines.push("#[test]".to_string());
                test_lines.push(format!("fn {}() {{", task));
                test_lines.push("    assert!(run_tests());".to_string());
                test_lines.push("}".to_string());
                write_lines(
                    format!("../algo_lib/tests/{}.rs", task).as_str(),
                    test_lines,
                );
                let from = format!("../{}/tests", task);
                std::fs::rename(from, format!("../algo_lib/tests/{}", task)).unwrap();
            }
        }
        std::fs::remove_dir_all(format!("../{}/", task)).unwrap();
    }
    write_lines("../Cargo.toml", result);
}
