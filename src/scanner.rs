use std::io;

pub struct Scanner {}

impl Scanner {
    pub fn new(_src: &str) -> Self {
        Self {}
    }
    pub fn scan_tokens(self: &Self) -> Result<Vec<Token>, io::Error> {
        todo!()
    }
}
#[derive(Debug)]
pub struct Token {}
