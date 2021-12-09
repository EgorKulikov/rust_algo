pub mod build;

use algo_lib::string::string::Str;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;
use std::{fs, io};

#[derive(Deserialize, Serialize, Debug)]
pub enum IOEnum {
    #[serde(rename = "stdin")]
    StdIn,
    #[serde(rename = "stdout")]
    StdOut,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "regex")]
    Regex,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IOType {
    #[serde(rename = "type")]
    pub io_type: IOEnum,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    pub pattern: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Test {
    pub input: String,
    pub output: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskClass {
    #[serde(rename = "taskClass")]
    pub task_class: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Languages {
    pub java: TaskClass,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TestType {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "multi_number")]
    MultiNumber,
    #[serde(rename = "multi_eof")]
    MultiEof,
}

impl TestType {
    pub const INPUT_TYPES: [&'static str; 3] = [
        TestType::Single.ui(),
        TestType::MultiNumber.ui(),
        TestType::MultiEof.ui(),
    ];

    pub const fn ui(&self) -> &'static str {
        match self {
            TestType::Single => "Single test",
            TestType::MultiNumber => "Number of tests given",
            TestType::MultiEof => "Read until EOF",
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Task {
    pub name: String,
    pub group: String,
    pub url: String,
    pub interactive: bool,
    #[serde(rename = "timeLimit")]
    pub time_limit: u64,
    pub tests: Vec<Test>,
    #[serde(rename = "testType")]
    pub test_type: TestType,
    pub input: IOType,
    pub output: IOType,
    pub languages: Languages,
}

pub fn read_from_file<P: AsRef<Path>>(filename: P) -> String {
    fs::read_to_string(filename).unwrap()
}

pub fn read_lines<P>(filename: P) -> Vec<String>
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

pub fn write_to_file<'a, P: AsRef<Path>, C: Into<Str<'a>>>(filename: P, content: C) {
    fs::write(filename, content.into().as_slice()).unwrap();
}

pub fn write_lines<'a, P: AsRef<Path>, C: Into<Str<'a>>>(filename: P, lines: Vec<C>) {
    let mut file = File::create(filename).unwrap();
    for line in lines {
        file.write(line.into().as_slice()).unwrap();
        file.write("\n".as_bytes()).unwrap();
    }
    file.flush().unwrap();
}
