use std::env;
use std::error::Error as StdError;
use std::fmt::Debug;
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
    Buffer(Vec<u8>),
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
            storage: InputStorage::Buffer(buf),
        })
    }

    pub fn from_string(s: String) -> Self {
        Self {
            storage: InputStorage::Buffer(s.as_bytes().to_vec()),
        }
    }

    pub fn as_str(&self) -> &str {
        let buf = match &self.storage {
            InputStorage::Mmap(mmap) => &mmap[..],
            InputStorage::Buffer(buf) => &buf,
        };
        std::str::from_utf8(buf).unwrap()
    }

    pub fn as_lines_parsed<T>(&self) -> impl Iterator<Item = Result<T, <T as FromStr>::Err>> + '_
    where
        T: FromStr,
        <T as FromStr>::Err: StdError,
    {
        self.as_str().lines().map(|l| l.parse::<T>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn items() {
        let input = Input::from_string("1\n2\n3\n4".to_string());
        let output: Vec<u32> = input.as_lines_parsed().map(|x| x.unwrap()).collect();
        assert_eq!(vec![1, 2, 3, 4], output);
    }
}
