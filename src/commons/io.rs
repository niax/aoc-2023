use itertools::Itertools;
use std::env;
use std::error::Error as StdError;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Error as IoError};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseLinesError<L: StdError> {
    #[error("IO Error reading from stream")]
    IoError(#[from] IoError),
    #[error("Parser error")]
    ParseError(L),
}

enum InputStorage {
    Mmap(memmap2::Mmap),
    String(Vec<u8>),
}

pub struct Input {
    storage: InputStorage,
}

impl Input {
    pub fn from_argv() -> Result<Self, Box<dyn std::error::Error>> {
        match env::args().nth(1) {
            Some(path) => Self::from_file(&path),
            None => Self::from_stdin(),
        }
    }

    pub fn from_file(p: &String) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(p)?;
        let storage = InputStorage::Mmap(unsafe { memmap2::Mmap::map(&file)? });
        Ok(Self { storage })
    }

    pub fn from_stdin() -> Result<Self, Box<dyn std::error::Error>> {
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin.lock());
        let mut buf = Vec::with_capacity(8196);
        reader.read_to_end(&mut buf)?;

        Ok(Self {
            storage: InputStorage::String(buf),
        })
    }

    pub fn as_str(&self) -> &str {
        let buf = match &self.storage {
            InputStorage::Mmap(mmap) => &mmap[..],
            InputStorage::String(buf) => &buf,
        };
        std::str::from_utf8(buf).unwrap()
    }

    pub fn as_lines_parsed<T, E>(&self) -> impl Iterator<Item = Result<T, E>> + '_
    where
        T: FromStr<Err = E>,
    {
        self.as_str().lines().map(|l| l.parse::<T>())
    }
}

pub fn get_argv_reader() -> BufReader<Box<dyn Read>> {
    let read: Box<dyn Read> = match env::args().nth(1) {
        Some(path) => Box::new(File::open(path).expect("File")),
        None => Box::new(io::stdin()),
    };
    BufReader::new(read)
}

pub fn load_argv_lines<T>() -> impl Iterator<Item = Result<T, ParseLinesError<<T as FromStr>::Err>>>
where
    T: FromStr,
    <T as FromStr>::Err: StdError,
{
    let reader = get_argv_reader();
    parse_lines(reader.lines())
}

pub fn load_stdin_lines<T>() -> impl Iterator<Item = Result<T, ParseLinesError<<T as FromStr>::Err>>>
where
    T: FromStr,
    <T as FromStr>::Err: StdError,
{
    let file = io::stdin();
    let reader = BufReader::new(file);
    parse_lines(reader.lines())
}

pub fn load_file_lines<T>(
    path: &str,
) -> impl Iterator<Item = Result<T, ParseLinesError<<T as FromStr>::Err>>>
where
    T: FromStr,
    <T as FromStr>::Err: StdError,
{
    let file = File::open(path).expect("Could not open input file");
    let reader = BufReader::new(file);
    parse_lines(reader.lines())
}

pub fn load_argv_records<T>(
    end_of_record: &str,
) -> impl Iterator<Item = Result<Vec<T>, ParseLinesError<<T as FromStr>::Err>>>
where
    T: FromStr,
    <T as FromStr>::Err: StdError,
{
    let reader = get_argv_reader();
    parse_records(reader.lines(), end_of_record.to_string())
}

pub fn load_file_records<T>(
    path: &str,
    end_of_record: &str,
) -> impl Iterator<Item = Result<Vec<T>, ParseLinesError<<T as FromStr>::Err>>>
where
    T: FromStr,
    <T as FromStr>::Err: StdError,
{
    let file = File::open(path).expect("Could not open input file");
    let reader = BufReader::new(file);
    parse_records(reader.lines(), end_of_record.to_string())
}

pub fn parse_lines<T, I>(
    input: I,
) -> impl Iterator<Item = Result<T, ParseLinesError<<T as FromStr>::Err>>>
where
    I: Iterator<Item = Result<String, std::io::Error>>,
    T: FromStr,
    <T as FromStr>::Err: StdError,
{
    input.map(|line| match line {
        Ok(l) => match l.parse() {
            Ok(t) => Ok(t),
            Err(e) => Err(ParseLinesError::ParseError(e)),
        },
        Err(e) => Err(e.into()),
    })
}

pub fn parse_records<T, I>(
    input: I,
    end_of_record: String,
) -> impl Iterator<Item = Result<Vec<T>, ParseLinesError<<T as FromStr>::Err>>>
where
    I: Iterator<Item = Result<String, std::io::Error>> + Itertools,
    T: FromStr,
    <T as FromStr>::Err: StdError,
{
    input.batching(move |it| {
        let mut batch = Vec::new();
        for res in it {
            if let Err(e) = res {
                return Some(Err(e.into()));
            }
            let line_string = res.unwrap();
            if line_string == end_of_record {
                break;
            }

            match line_string.parse() {
                Ok(t) => batch.push(t),
                Err(e) => return Some(Err(ParseLinesError::ParseError(e))),
            }
        }
        if batch.is_empty() {
            None
        } else {
            Some(Ok(batch))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn items() {
        let input = "1\n2\n3\n4";
        let cursor = Cursor::new(input.to_string());
        let output: Vec<u32> = parse_lines(cursor.lines()).map(|x| x.unwrap()).collect();
        assert_eq!(vec![1, 2, 3, 4], output);
    }
}
