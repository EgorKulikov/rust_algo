use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::fmt::Display;

pub(crate) trait TestSet {
    type TestId: Display + Clone;

    fn name(&self) -> &str;
    fn tests(&self) -> impl Iterator<Item = Self::TestId>;
    fn input(&self, test: &Self::TestId) -> Vec<u8>;
    fn output(&self, test: &Self::TestId, input: &[u8]) -> Option<Vec<u8>>;
    fn print_details(&self) -> bool;
}

pub(crate) struct SampleTests {
    pub(crate) task_folder: String,
}

impl TestSet for SampleTests {
    type TestId = String;

    fn name(&self) -> &str {
        "Sample tests"
    }

    fn tests(&self) -> impl Iterator<Item = Self::TestId> {
        let mut paths = std::fs::read_dir(format!("{}/tests", self.task_folder))
            .unwrap()
            .map(|res| res.unwrap())
            .collect::<Vec<_>>();
        paths.sort_by_key(|dir| dir.path());
        paths.into_iter().filter_map(|path| {
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
        })
    }

    fn input(&self, test: &Self::TestId) -> Vec<u8> {
        std::fs::read(format!("{}/tests/{}.in", self.task_folder, test)).unwrap()
    }

    fn output(&self, test: &Self::TestId, _input: &[u8]) -> Option<Vec<u8>> {
        std::fs::read(format!("{}/tests/{}.out", self.task_folder, test)).ok()
    }

    fn print_details(&self) -> bool {
        true
    }
}

pub trait GeneratedTestSet {
    type TestId: Display + Clone;

    fn tests(&self) -> impl Iterator<Item = Self::TestId>;
    fn input(&self, test: &Self::TestId, input: &mut Output);
    fn output(&self, test: &Self::TestId, input: &mut Input, output: &mut Output) -> bool;
}

pub(crate) struct GeneratedTests<TestSet: GeneratedTestSet> {
    pub(crate) name: String,
    pub(crate) print_details: bool,
    pub(crate) set: TestSet,
}

impl<Set: GeneratedTestSet> TestSet for GeneratedTests<Set> {
    type TestId = Set::TestId;

    fn name(&self) -> &str {
        &self.name
    }

    fn tests(&self) -> impl Iterator<Item = Self::TestId> {
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
