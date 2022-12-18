use std::{fs::File, io::Read};

pub fn read_all(path: String) -> String {
    let mut f = File::open(&path).expect("文件打开失败");

    let mut result = String::new();
    f.read_to_string(&mut result).expect("文件读取失败");

    result
}