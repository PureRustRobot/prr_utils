use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::io::BufReader;
use std::fs::File;

pub fn get_param(mut reader:Reader<BufReader<File>>, name:&str)->String
{
    let mut flg = false;

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {

            Ok(Event::Start(e)) => {
                let elm_name = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                if elm_name == name {
                    flg = true
                }
            }

            // テキストイベント
            Ok(Event::Text(e)) => {
                if flg {
                    return e.unescape().unwrap().to_string();
                }
            }

            // その他のイベントは何もしない
            _ => (),
        }
        buf.clear();  // メモリ節約のためbufをクリアする
    }
}