pub mod error;
pub mod launch;
pub mod param;


use quick_xml::reader::Reader;
use std::io::BufReader;
use std::fs::File;
use colored::{self, Colorize};

pub fn get_xml(path:String)->Reader<BufReader<File>>
{
    let mut reader = Reader::from_file(path).unwrap();
    reader.trim_text(true);

    reader
}