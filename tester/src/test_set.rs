#[allow(dead_code)]
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::fmt::Display;
#[cfg(not(feature = "test"))]
use std::fs::File;
#[cfg(not(feature = "test"))]
use std::io::Read;

pub(crate) trait TestSet {
    type TestId: Display + Clone;

    fn name(&self) -> &str;
    fn tests(&self) -> Box<dyn Iterator<Item = Self::TestId>>;
    fn input(&self, test: &Self::TestId) -> Vec<u8>;
    fn output(&self, test: &Self::TestId, input: &[u8]) -> Option<Vec<u8>>;
    fn print_details(&self) -> bool;
    fn save_output(&self, _test: &Self::TestId, _output: &[u8]) {}
    fn output_diff(&self, _test: &Self::TestId) {}
}

pub(crate) struct SampleTests {
    pub(crate) task_folder: String,
}

#[cfg(feature = "test")]
impl SampleTests {
    fn test_path(&self) -> String {
        format!("tests/{}", self.task_folder)
    }
}

#[cfg(not(feature = "test"))]
impl SampleTests {
    fn test_path(&self) -> String {
        format!("tasks/{}/tests", self.task_folder)
    }
}

impl TestSet for SampleTests {
    type TestId = String;

    fn name(&self) -> &str {
        "Sample tests"
    }

    fn tests(&self) -> Box<dyn Iterator<Item = Self::TestId>> {
        let mut paths = std::fs::read_dir(self.test_path())
            .unwrap()
            .map(|res| res.unwrap())
            .collect::<Vec<_>>();
        paths.sort_by_key(|dir| dir.path());
        Box::new(paths.into_iter().filter_map(|path| {
            if path.file_type().unwrap().is_file() {
                let path = path.path();
                match path.extension() {
                    None => None,
                    Some(extension) => {
                        if extension.to_str() == Some("in") {
                            Some(path.file_stem().unwrap().to_str().unwrap().to_string())
                        } else {
                            None
                        }
                    }
                }
            } else {
                None
            }
        }))
    }

    fn input(&self, test: &Self::TestId) -> Vec<u8> {
        #[cfg(feature = "test")]
        let file = format!("tests/{}/{}.in", self.task_folder, test);
        #[cfg(not(feature = "test"))]
        let file = format!("tasks/{}/tests/{}.in", self.task_folder, test);
        std::fs::read(file).unwrap()
    }

    fn output(&self, test: &Self::TestId, _input: &[u8]) -> Option<Vec<u8>> {
        #[cfg(feature = "test")]
        let file = format!("tests/{}/{}.out", self.task_folder, test);
        #[cfg(not(feature = "test"))]
        let file = format!("tasks/{}/tests/{}.out", self.task_folder, test);
        std::fs::read(file).ok()
    }

    fn print_details(&self) -> bool {
        true
    }

    fn save_output(&self, test: &Self::TestId, output: &[u8]) {
        #[cfg(not(feature = "test"))]
        {
            let file = format!("tasks/{}/tests/{}.ans", self.task_folder, test);
            std::fs::write(file, output).unwrap();
        }
        #[cfg(feature = "test")]
        {
            let _ = (test, output);
        }
    }

    fn output_diff(&self, test: &Self::TestId) {
        #[cfg(not(feature = "test"))]
        {
            let mut expected = String::new();
            File::open(format!("tasks/{}/tests/{}.out", self.task_folder, test))
                .unwrap()
                .read_to_string(&mut expected)
                .unwrap();
            if expected.len() > 500 {
                return;
            }
            let mut actual = String::new();
            File::open(format!("tasks/{}/tests/{}.ans", self.task_folder, test))
                .unwrap()
                .read_to_string(&mut actual)
                .unwrap();
            println!(
                "{}",
                pretty_assertions::StrComparison::new(&expected, &actual)
            );
        }
        #[cfg(feature = "test")]
        {
            let _ = test;
        }
    }
}

pub trait GeneratedTestSet {
    type TestId: Display + Clone;

    fn tests(&self) -> Box<dyn Iterator<Item = Self::TestId>>;
    fn input(&self, test: &Self::TestId, input: &mut Output);
    fn output(&self, test: &Self::TestId, input: &mut Input, output: &mut Output) -> bool;
}

#[allow(dead_code)]
pub(crate) struct GeneratedTests<TestSet: GeneratedTestSet> {
    pub(crate) name: String,
    pub(crate) print_details: bool,
    pub(crate) set: TestSet,
}

impl<Set: GeneratedTestSet> TestSet for GeneratedTests<Set>
where
    Set::TestId: 'static,
{
    type TestId = Set::TestId;

    fn name(&self) -> &str {
        &self.name
    }

    fn tests(&self) -> Box<dyn Iterator<Item = Self::TestId>> {
        #[cfg(feature = "test")]
        return Box::new(self.set.tests().take(100));
        #[cfg(not(feature = "test"))]
        self.set.tests()
    }

    fn input(&self, test: &Self::TestId) -> Vec<u8> {
        let mut input_container = Vec::new();
        let mut input = Output::new(&mut input_container);
        self.set.input(test, &mut input);
        input.flush();
        input_container
    }

    fn output(&self, test: &Self::TestId, mut input: &[u8]) -> Option<Vec<u8>> {
        let mut output_container = Vec::new();
        let mut input = Input::new(&mut input);
        let mut output = Output::new(&mut output_container);
        if self.set.output(test, &mut input, &mut output) {
            output.flush();
            Some(output_container)
        } else {
            None
        }
    }

    fn print_details(&self) -> bool {
        self.print_details
    }
}
