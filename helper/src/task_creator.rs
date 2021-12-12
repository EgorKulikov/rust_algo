use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use std::fs;
use std::process::Command;
use util::{IOEnum, IOType, Languages, Task, TaskClass, Test, TestType};

pub fn task_name(task: &Task) -> String {
    let mut res = String::new();
    let mut last_uppercase = true;
    for c in task.languages.java.task_class.chars() {
        if c.is_uppercase() {
            if !last_uppercase || res.len() == 1 {
                res.push('_');
            }
            last_uppercase = true;
            res.push(c.to_ascii_lowercase());
        } else {
            last_uppercase = false;
            res.push(c);
        }
    }
    res
}

pub fn adjust_input_type(task: &mut Task) {
    let need_to_select = get_solve(task).contains("$INVOKE");
    if need_to_select {
        let test_type = select_test_type();
        task.test_type = test_type;
    }
}

fn select_test_type() -> TestType {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select test type:")
        .default(0)
        .items(&TestType::INPUT_TYPES[..])
        .interact_on(&Term::stdout())
        .unwrap();
    let test_type = match selection {
        0 => TestType::Single,
        1 => TestType::MultiNumber,
        2 => TestType::MultiEof,
        _ => unreachable!(),
    };
    test_type
}

pub fn get_solve(task: &Task) -> String {
    let mut spl = task.group.split(" ");
    let site = match spl.next() {
        None => "default".to_string(),
        Some(site) => site.to_lowercase(),
    };
    let solve = std::fs::read_to_string(format!("templates/sites/{}.rs", site));
    match solve {
        Err(_) => util::read_from_file("templates/sites/default.rs"),
        Ok(solve) => solve,
    }
}

pub fn get_invoke(task: &Task) -> String {
    util::read_from_file(match task.test_type {
        TestType::Single => "templates/single.rs",
        TestType::MultiNumber => "templates/multi_number.rs",
        TestType::MultiEof => "templates/multi_eof.rs",
    })
}

pub fn create(task: Task) {
    let name = task_name(&task);
    let mut lines = Vec::new();
    for l in util::read_lines("../Cargo.toml") {
        if l.contains(format!("\"{}\"", name).as_str()) {
            eprintln!("Task {} exists", name);
            return;
        }
        lines.push(l.clone());
        if l.as_str() == "members = [" {
            lines.push(format!("    \"{}\",", name));
        }
    }
    util::write_lines("../Cargo.toml", lines);
    fs::create_dir_all(format!("../{}/src", name)).unwrap();
    fs::create_dir_all(format!("../{}/tests", name)).unwrap();
    for (i, test) in task.tests.iter().enumerate() {
        util::write_to_file(format!("../{}/tests/{}.in", name, i + 1), &test.input);
        util::write_to_file(format!("../{}/tests/{}.out", name, i + 1), &test.output);
    }
    util::write_to_file(
        format!("../{}/build.rs", name),
        util::read_from_file("templates/build.rs"),
    );
    let mut solve = get_solve(&task);
    solve = solve.replace("$INVOKE", get_invoke(&task).as_str());
    let mut main = util::read_from_file("templates/main.rs");
    main = main.replace("$SOLVE", solve.as_str());
    main = main.replace("$JSON", serde_json::to_string(&task).unwrap().as_str());
    let (row, col) = match main.find("$CARET") {
        None => (1, 1),
        Some(pos) => {
            let chars = main.chars().take(pos);
            let mut row = 1;
            let mut col = 1;
            for c in chars {
                if c == '\n' {
                    row += 1;
                    col = 0;
                }
                col += 1;
            }
            (row, col)
        }
    };
    main = main.replace("$CARET", "");
    util::write_to_file(format!("../{}/src/main.rs", name), main);
    let mut tester = util::read_from_file("templates/tester.rs");
    tester = tester.replace("$TIME_LIMIT", task.time_limit.to_string().as_str());
    tester = tester.replace("$TASK", name.as_str());
    util::write_to_file(format!("../{}/src/tester.rs", name), tester);
    let mut toml = util::read_from_file("templates/Cargo.toml");
    toml = toml.replace("$TASK", name.as_str());
    util::write_to_file(format!("../{}/Cargo.toml", name).as_str(), toml);
    println!("Task {} parsed", name);
    #[cfg(windows)]
    match Command::new("..\\..\\clion.cmd")
        .args([
            "--line",
            row.to_string().as_str(),
            "--column",
            col.to_string().as_str(),
            format!("../{}/src/main.rs", name).as_str(),
        ])
        .output()
    {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    }
    #[cfg(not(windows))]
    match Command::new("../../clion.sh")
        .args([
            "--line",
            row.to_string().as_str(),
            "--column",
            col.to_string().as_str(),
            format!("../{}/src/main.rs", name).as_str(),
        ])
        .output()
    {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    }
}

fn select_name() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Task name:")
        .interact_on(&Term::stdout())
        .unwrap()
}

fn select_num_tests() -> usize {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Num sample tests:")
        .validate_with(|input: &String| match input.parse::<usize>() {
            Ok(num) => {
                if num > 10 {
                    Err("Too many")
                } else {
                    Ok(())
                }
            }
            Err(_) => Err("Please enter number"),
        })
        .interact_on(&Term::stdout())
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

const INPUT_OPTIONS: [&'static str; 2] = ["Stdin", "File"];

fn select_input_type() -> IOType {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select option:")
        .default(0)
        .items(&INPUT_OPTIONS)
        .interact_on(&Term::stdout())
        .unwrap();
    let (io_type, file_name) = match selection {
        0 => (IOEnum::StdIn, None),
        1 => (
            IOEnum::File,
            Some(
                Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Num sample tests:")
                    .interact_on(&Term::stdout())
                    .unwrap(),
            ),
        ),
        _ => unreachable!(),
    };
    IOType {
        io_type,
        file_name,
        pattern: None,
    }
}

const OUTPUT_OPTIONS: [&'static str; 2] = ["Stdout", "File"];

fn select_output_type() -> IOType {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select option:")
        .default(0)
        .items(&OUTPUT_OPTIONS)
        .interact_on(&Term::stdout())
        .unwrap();
    let (io_type, file_name) = match selection {
        0 => (IOEnum::StdOut, None),
        1 => (
            IOEnum::File,
            Some(
                Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Num sample tests:")
                    .interact_on(&Term::stdout())
                    .unwrap(),
            ),
        ),
        _ => unreachable!(),
    };
    IOType {
        io_type,
        file_name,
        pattern: None,
    }
}

pub fn create_task_wizard() {
    let name = select_name();
    let task = Task {
        name: name.clone(),
        group: "Manual".to_string(),
        url: "".to_string(),
        interactive: false,
        time_limit: 2000,
        tests: vec![
            Test {
                input: "".to_string(),
                output: "".to_string()
            };
            select_num_tests()
        ],
        test_type: select_test_type(),
        input: select_input_type(),
        output: select_output_type(),
        languages: Languages {
            java: TaskClass {
                task_class: name.replace(" ", ""),
            },
        },
    };
    create(task);
}
