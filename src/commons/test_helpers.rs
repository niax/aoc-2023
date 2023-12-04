use super::io::{load_file_lines, ParseLinesError};
use std::{error::Error, fs, path::PathBuf, str::FromStr};

pub struct TestCase<P1, P2> {
    pub input_path: &'static str,
    pub part1_expected: P1,
    pub part2_expected: P2,
}

impl<P1, P2> TestCase<P1, P2> {
    fn path(&self) -> PathBuf {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push(self.input_path);
        input_path
    }

    pub fn load_file(&self) -> String {
        let input_path = self.path();
        fs::read_to_string(input_path).unwrap()
    }

    pub fn load_file_lines<T>(
        &self,
    ) -> impl Iterator<Item = Result<T, ParseLinesError<<T as FromStr>::Err>>>
    where
        T: FromStr,
        <T as FromStr>::Err: Error,
    {
        let input_path = self.path();
        load_file_lines::<T>(input_path.to_str().unwrap())
    }
}
