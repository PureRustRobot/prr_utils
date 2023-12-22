pub mod error;
pub mod launch;
pub mod param;

use async_std::{io::BufReader, fs::File};
use quick_xml::reader::Reader;

pub fn get_xml(path:String)->Reader<B>
{
    let mut reader = Reader::from_file(path).unwrap();
    reader.trim_text(true);

    reader
}