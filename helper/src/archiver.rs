use algo_lib::collections::iter_ext::IterExt;
use chrono::{Datelike, Utc};
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead, BufReader};
use util::Task;

fn contest_list() -> Vec<(String, Vec<String>)> {
    let lines = util::read_lines("../Cargo.toml");
    let mut result = BTreeMap::new();
    for line in lines {
        if !line.starts_with("    ") {
            continue;
        }
        let line = line.trim().as_bytes();
        let task_name =
            String::from_utf8_lossy(&(line[1..line.len() - 2].iter().cloned().collect::<Vec<_>>()))
                .to_string();
        let main = fs::File::open(format!("../{}/src/main.rs", task_name));
        if main.is_err() {
            continue;
        }
        let main = main.unwrap();
        let mut content = BufReader::new(main).lines();
        let first_line = content.next();
        if first_line.is_none() {
            continue;
        }
        let first_line = first_line.unwrap().unwrap();
        if !first_line.starts_with("//") {
            continue;
        }
        let json = first_line.chars().skip(2).collect::<String>();
        match serde_json::from_str::<Task>(json.as_str()) {
            Ok(task) => {
                if !result.contains_key(&task.group) {
                    result.insert(task.group.clone(), Vec::new());
                }
                result.get_mut(&task.group).unwrap().push(task_name);
            }
            Err(_) => {}
        }
    }
    result.into_iter().collect()
}

const OPTIONS: [&'static str; 4] = ["Skip", "Delete", "Archive only", "Archive and tests"];

fn ask_archive(task_name: String) {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Task {}", task_name))
        .default(2)
        .items(&OPTIONS[..])
        .interact_on(&Term::stdout())
        .unwrap();

    if selection == 0 {
        return;
    }
    if selection >= 2 {
        let now = Utc::now();
        let mut main = util::read_lines(format!("../{}/src/main.rs", task_name));
        let task =
            serde_json::from_str::<Task>(main[0].chars().skip(2).collect::<String>().as_str())
                .unwrap();
        let path = format!(
            "../archive/{}/{}/{}.{}.{} - {}",
            now.year(),
            now.month(),
            now.day(),
            now.month(),
            now.year(),
            task.group
        );
        let path = path.replace(":", "_");
        fs::create_dir_all(path.clone()).unwrap();
        util::write_lines(format!("{}/{}.rs", path, task_name), main.clone());
        if selection == 3 {
            let tester = util::read_lines(format!("../{}/src/tester.rs", task_name));
            main.push("mod tester {".to_string());
            main.extend_from_slice(tester.as_slice());
            main.push("}".to_string());
            let mut test_lines = Vec::new();
            let mut in_main = false;
            for mut line in main {
                if line
                    .trim()
                    .starts_with("let mut paths = std::fs::read_dir(")
                {
                    line = format!(
                        "    let mut paths = std::fs::read_dir(\"./tests/{}/\")",
                        task_name,
                    );
                }
                if line == "//START MAIN".to_string() {
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
            test_lines.push(format!("fn {}() {{", task_name));
            test_lines.push("    assert!(tester::run_tests());".to_string());
            test_lines.push("}".to_string());
            util::write_lines(format!("../algo_lib/tests/{}.rs", task_name), test_lines);
            let from = format!("../{}/tests", task_name);
            std::fs::rename(from, format!("../algo_lib/tests/{}", task_name)).unwrap();
        }
    }
    std::fs::remove_dir_all(format!("../{}/", task_name)).unwrap();

    let lines = util::read_lines("../Cargo.toml")
        .into_iter()
        .filter(|line| line != &format!("    \"{}\",", task_name))
        .collect_vec();
    util::write_lines("../Cargo.toml", lines);
}

pub fn archive() {
    let contest_list = contest_list();
    if contest_list.is_empty() {
        eprintln!("No tasks");
        return;
    }
    let contests = contest_list
        .iter()
        .map(|(contest, _)| contest.clone())
        .collect_vec();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select contest:")
        .default(0)
        .items(contests.as_slice())
        .interact_on_opt(&Term::stdout())
        .unwrap();

    if selection.is_none() {
        return;
    }

    let selection = selection.unwrap();
    let tasks = contest_list[selection].1.clone();

    for task in tasks {
        ask_archive(task);
    }
}
