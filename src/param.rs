use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::io::BufReader;
use std::fs::File;

pub fn get_param(mut reader:Reader<BufReader<File>>, name:&str)->String
{
    let mut flg = false;

    let mut buf = Vec::new();

    
}